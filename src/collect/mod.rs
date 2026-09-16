pub mod edac;
pub mod fsusage;
pub mod kernellog;
pub mod mounts;
pub mod net;

pub use edac::Edac;
pub use fsusage::FsUsage;
pub use kernellog::KernelLog;
pub use mounts::Mount;
pub use net::{Conntrack, Iface, NetFacts, RouteFacts};

use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ok,
    Unavailable,
    Unsupported,
    Malformed,
}

impl Status {
    pub fn as_str(self) -> &'static str {
        match self {
            Status::Ok => "ok",
            Status::Unavailable => "unavailable",
            Status::Unsupported => "unsupported",
            Status::Malformed => "malformed",
        }
    }
}

#[derive(Debug)]
pub struct Source<T> {
    pub status: Status,
    pub value: Option<T>,
    pub detail: Option<String>,
    pub method: Option<String>,
    pub elapsed: Duration,
}

impl<T> Source<T> {
    pub fn ok(value: T, method: impl Into<String>, elapsed: Duration) -> Self {
        Source {
            status: Status::Ok,
            value: Some(value),
            detail: None,
            method: Some(method.into()),
            elapsed,
        }
    }

    pub fn failed(status: Status, detail: impl Into<String>, elapsed: Duration) -> Self {
        debug_assert!(status != Status::Ok);
        Source {
            status,
            value: None,
            detail: Some(detail.into()),
            method: None,
            elapsed,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.status == Status::Ok
    }

    pub fn get(&self) -> Option<&T> {
        if self.is_ok() {
            self.value.as_ref()
        } else {
            None
        }
    }

    pub fn failure_reason(&self) -> String {
        self.detail
            .clone()
            .unwrap_or_else(|| self.status.as_str().to_string())
    }
}

#[derive(Debug)]
pub struct Facts {
    pub hostname: String,
    pub boot_time: Option<u64>,
    pub kernel_log: Source<KernelLog>,
    pub mounts: Source<Vec<Mount>>,
    pub fs_usage: Source<Vec<FsUsage>>,
    pub edac: Source<Edac>,
    pub net: Source<NetFacts>,
    pub routes: Source<RouteFacts>,
    pub conntrack: Source<Conntrack>,
}

pub fn collect_facts() -> Facts {
    let mounts = mounts::collect();
    let fs_usage = fsusage::collect(&mounts);
    Facts {
        hostname: hostname(),
        boot_time: boot_time(),
        kernel_log: kernellog::collect(),
        mounts,
        fs_usage,
        edac: edac::collect(),
        net: net::collect_net(),
        routes: net::collect_routes(),
        conntrack: net::collect_conntrack(),
    }
}

fn hostname() -> String {
    std::fs::read_to_string("/proc/sys/kernel/hostname")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".to_string())
}

fn boot_time() -> Option<u64> {
    let stat = std::fs::read_to_string("/proc/stat").ok()?;
    parse_btime(&stat)
}

fn parse_btime(stat: &str) -> Option<u64> {
    stat.lines()
        .find_map(|l| l.strip_prefix("btime "))
        .and_then(|v| v.trim().parse().ok())
}

pub fn run_command(program: &str, args: &[&str], timeout: Duration) -> Result<String, String> {
    let start = Instant::now();
    let mut child = Command::new(program)
        .args(args)
        .env("LC_ALL", "C")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("{}: {}", program, e))?;

    let mut out_pipe = child.stdout.take().expect("stdout piped");
    let mut err_pipe = child.stderr.take().expect("stderr piped");
    let out_reader = thread::spawn(move || {
        let mut buf = Vec::new();
        out_pipe.read_to_end(&mut buf).map(|_| buf)
    });
    let err_reader = thread::spawn(move || {
        let mut buf = Vec::new();
        err_pipe.read_to_end(&mut buf).map(|_| buf)
    });

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let out = out_reader.join().ok().and_then(|r| r.ok()).unwrap_or_default();
                let err = err_reader.join().ok().and_then(|r| r.ok()).unwrap_or_default();
                if status.success() {
                    return Ok(String::from_utf8_lossy(&out).into_owned());
                }
                let err = String::from_utf8_lossy(&err);
                let trimmed = err.trim();
                let stripped = trimmed.strip_prefix(program).and_then(|r| r.strip_prefix(':'));
                let detail: String = stripped.unwrap_or(trimmed).trim_start().chars().take(200).collect();
                if detail.is_empty() {
                    return Err(format!("{}: exited with {}", program, status));
                }
                return Err(format!("{}: {}", program, detail));
            }
            Ok(None) => {
                if start.elapsed() >= timeout {
                    let _ = child.kill();
                    let _ = child.wait();
                    let _ = out_reader.join();
                    let _ = err_reader.join();
                    return Err(format!("{}: timed out after {}s", program, timeout.as_secs()));
                }
                thread::sleep(Duration::from_millis(25));
            }
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = out_reader.join();
                let _ = err_reader.join();
                return Err(format!("{}: wait failed: {}", program, e));
            }
        }
    }
}
