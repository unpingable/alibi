pub mod edac;
pub mod fsusage;
pub mod kernellog;
pub mod mounts;
pub mod net;

pub use edac::{Edac, EdacController};
pub use fsusage::{FsCapacity, FsUsage};
pub use kernellog::KernelLog;
pub use mounts::Mount;
pub use net::{Conntrack, Iface, NetFacts, RouteFacts};

use std::io::{ErrorKind, Read};
use std::os::fd::OwnedFd;
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt;
use std::process::Child;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Completeness {
    Complete,
    Partial,
    None,
}

impl Completeness {
    pub fn as_str(self) -> &'static str {
        match self {
            Completeness::Complete => "complete",
            Completeness::Partial => "partial",
            Completeness::None => "none",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceWindow {
    pub started_at: Duration,
    pub ended_at: Duration,
}

impl EvidenceWindow {
    pub fn duration(self) -> Duration {
        self.ended_at.saturating_sub(self.started_at)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SourceId {
    KernelLog,
    Mounts,
    FsUsage,
    Edac,
    Net,
    Routes,
    Conntrack,
}

impl SourceId {
    pub fn as_str(self) -> &'static str {
        match self {
            SourceId::KernelLog => "kernel-log",
            SourceId::Mounts => "mounts",
            SourceId::FsUsage => "fs-usage",
            SourceId::Edac => "edac",
            SourceId::Net => "net",
            SourceId::Routes => "routes",
            SourceId::Conntrack => "conntrack",
        }
    }

    pub const ALL: [SourceId; 7] = [
        SourceId::KernelLog,
        SourceId::Mounts,
        SourceId::FsUsage,
        SourceId::Edac,
        SourceId::Net,
        SourceId::Routes,
        SourceId::Conntrack,
    ];
}

#[derive(Debug, Clone)]
pub struct CollectionPlan {
    sources: Vec<SourceId>,
}

impl CollectionPlan {
    pub fn new(mut sources: Vec<SourceId>) -> Self {
        sources.sort();
        sources.dedup();
        CollectionPlan { sources }
    }

    pub fn all() -> Self {
        Self::new(SourceId::ALL.to_vec())
    }

    pub fn contains(&self, source: SourceId) -> bool {
        self.sources.binary_search(&source).is_ok()
    }

    pub fn sources(&self) -> &[SourceId] {
        &self.sources
    }
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
    pub completeness: Completeness,
    pub value: Option<T>,
    pub detail: Option<String>,
    pub method: Option<String>,
    pub elapsed: Duration,
    pub completed_at: Duration,
    pub window: Option<EvidenceWindow>,
}

impl<T> Source<T> {
    pub fn ok(value: T, method: impl Into<String>, elapsed: Duration) -> Self {
        Source {
            status: Status::Ok,
            completeness: Completeness::Complete,
            value: Some(value),
            detail: None,
            method: Some(method.into()),
            elapsed,
            completed_at: elapsed,
            window: None,
        }
    }

    pub fn partial(
        value: T,
        status: Status,
        detail: impl Into<String>,
        method: impl Into<String>,
        elapsed: Duration,
    ) -> Self {
        debug_assert!(status != Status::Ok);
        Source {
            status,
            completeness: Completeness::Partial,
            value: Some(value),
            detail: Some(detail.into()),
            method: Some(method.into()),
            elapsed,
            completed_at: elapsed,
            window: None,
        }
    }

    pub fn failed(status: Status, detail: impl Into<String>, elapsed: Duration) -> Self {
        debug_assert!(status != Status::Ok);
        Source {
            status,
            completeness: Completeness::None,
            value: None,
            detail: Some(detail.into()),
            method: None,
            elapsed,
            completed_at: elapsed,
            window: None,
        }
    }

    pub fn is_ok(&self) -> bool {
        self.status == Status::Ok && self.completeness == Completeness::Complete
    }

    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub fn is_complete(&self) -> bool {
        self.is_ok()
    }

    pub fn complete_at(mut self, completed_at: Duration) -> Self {
        self.completed_at = completed_at;
        self
    }

    pub fn with_window(mut self, started_at: Duration, ended_at: Duration) -> Self {
        self.window = Some(EvidenceWindow {
            started_at,
            ended_at,
        });
        self
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
    let t0 = Instant::now();
    collect_facts_for(&CollectionPlan::all(), t0)
}

pub fn collect_facts_for(plan: &CollectionPlan, t0: Instant) -> Facts {
    let mounts = if plan.contains(SourceId::Mounts) || plan.contains(SourceId::FsUsage) {
        mounts::collect().complete_at(t0.elapsed())
    } else {
        not_requested()
    };
    let fs_usage = if plan.contains(SourceId::FsUsage) {
        fsusage::collect(&mounts).complete_at(t0.elapsed())
    } else {
        not_requested()
    };
    Facts {
        hostname: hostname(),
        boot_time: boot_time(),
        kernel_log: if plan.contains(SourceId::KernelLog) {
            kernellog::collect().complete_at(t0.elapsed())
        } else {
            not_requested()
        },
        mounts,
        fs_usage,
        edac: if plan.contains(SourceId::Edac) {
            edac::collect().complete_at(t0.elapsed())
        } else {
            not_requested()
        },
        net: if plan.contains(SourceId::Net) {
            net::collect_net(t0).complete_at(t0.elapsed())
        } else {
            not_requested()
        },
        routes: if plan.contains(SourceId::Routes) {
            net::collect_routes().complete_at(t0.elapsed())
        } else {
            not_requested()
        },
        conntrack: if plan.contains(SourceId::Conntrack) {
            net::collect_conntrack().complete_at(t0.elapsed())
        } else {
            not_requested()
        },
    }
}

fn not_requested<T>() -> Source<T> {
    Source::failed(Status::Unsupported, "not requested", Duration::ZERO)
}

impl Facts {
    pub fn source_complete(&self, id: SourceId) -> bool {
        match id {
            SourceId::KernelLog => self.kernel_log.is_complete(),
            SourceId::Mounts => self.mounts.is_complete(),
            SourceId::FsUsage => self.fs_usage.is_complete(),
            SourceId::Edac => self.edac.is_complete(),
            SourceId::Net => self.net.is_complete(),
            SourceId::Routes => self.routes.is_complete(),
            SourceId::Conntrack => self.conntrack.is_complete(),
        }
    }

    pub fn source_failure(&self, id: SourceId) -> String {
        match id {
            SourceId::KernelLog => self.kernel_log.failure_reason(),
            SourceId::Mounts => self.mounts.failure_reason(),
            SourceId::FsUsage => self.fs_usage.failure_reason(),
            SourceId::Edac => self.edac.failure_reason(),
            SourceId::Net => self.net.failure_reason(),
            SourceId::Routes => self.routes.failure_reason(),
            SourceId::Conntrack => self.conntrack.failure_reason(),
        }
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

// Limits apply to each stream. Hitting a limit is incomplete acquisition, not
// permission to interpret the retained prefix as complete evidence.
const COMMAND_OUTPUT_LIMIT: usize = 8 * 1024 * 1024;
const COMMAND_CLEANUP_ALLOWANCE: Duration = Duration::from_millis(100);

pub fn run_command(program: &str, args: &[&str], timeout: Duration) -> Result<String, String> {
    run_command_limited(program, args, timeout, COMMAND_OUTPUT_LIMIT)
}

fn run_command_limited(
    program: &str,
    args: &[&str],
    timeout: Duration,
    limit: usize,
) -> Result<String, String> {
    ensure_waitable_children().map_err(|error| format!("{}: {}", program, error))?;
    let start = Instant::now();
    let (mut stdout, child_stdout) =
        UnixStream::pair().map_err(|e| format!("{}: stdout channel: {}", program, e))?;
    let (mut stderr, child_stderr) =
        UnixStream::pair().map_err(|e| format!("{}: stderr channel: {}", program, e))?;
    stdout.set_nonblocking(true).map_err(|e| e.to_string())?;
    stderr.set_nonblocking(true).map_err(|e| e.to_string())?;
    let mut child = Command::new(program)
        .args(args)
        .env("LC_ALL", "C")
        .stdin(Stdio::null())
        .stdout(Stdio::from(OwnedFd::from(child_stdout)))
        .stderr(Stdio::from(OwnedFd::from(child_stderr)))
        .process_group(0)
        .spawn()
        .map_err(|e| format!("{}: {}", program, e))?;
    let mut out = Vec::new();
    let mut err = Vec::new();
    let mut out_eof = false;
    let mut err_eof = false;
    let failure = loop {
        if start.elapsed() >= timeout {
            break format!(
                "timed out after {}ms (including output drain)",
                timeout.as_millis()
            );
        }
        // One bounded read from each stream per turn prevents a busy writer
        // from starving the other stream or the deadline check.
        let mut progress = false;
        for (stream, buffer, eof, label) in [
            (&mut stdout, &mut out, &mut out_eof, "stdout"),
            (&mut stderr, &mut err, &mut err_eof, "stderr"),
        ] {
            if !*eof {
                match drain_chunk(stream, buffer, eof, limit) {
                    Ok(read_progress) => progress |= read_progress,
                    Err(e) => {
                        let cleanup = terminate_command(&mut child);
                        return Err(format!("{}: {}: {}{}", program, label, e, cleanup));
                    }
                }
            }
        }
        if start.elapsed() >= timeout {
            continue;
        }
        // Do not reap the leader while descendants could retain output. Its
        // unreaped identity pins the process-group ID for failure cleanup.
        if out_eof && err_eof {
            match child.try_wait() {
                Ok(Some(status)) => {
                    if start.elapsed() >= timeout {
                        return Err(format!(
                            "{}: timed out after {}ms",
                            program,
                            timeout.as_millis()
                        ));
                    }
                    if status.success() {
                        return String::from_utf8(out)
                            .map_err(|_| format!("{}: stdout is not valid UTF-8", program));
                    }
                    let err = String::from_utf8_lossy(&err);
                    let trimmed = err.trim();
                    let detail: String = trimmed
                        .strip_prefix(program)
                        .and_then(|r| r.strip_prefix(':'))
                        .unwrap_or(trimmed)
                        .trim_start()
                        .chars()
                        .take(200)
                        .collect();
                    return Err(if detail.is_empty() {
                        format!("{}: exited with {}", program, status)
                    } else {
                        format!("{}: {}", program, detail)
                    });
                }
                Ok(None) => {}
                Err(e) => {
                    // ECHILD (or any unexpected wait failure) invalidates our
                    // leader-ownership evidence. Never signal a numeric group
                    // after losing that evidence, even as attempted cleanup.
                    return Err(format!(
                        "{}: wait failed: {}; cleanup not signaled: child ownership unconfirmed",
                        program, e
                    ));
                }
            }
        }
        if !progress {
            thread::sleep(Duration::from_millis(1).min(timeout.saturating_sub(start.elapsed())));
        }
    };
    let cleanup = terminate_command(&mut child);
    Err(format!("{}: {}{}", program, failure, cleanup))
}

fn drain_chunk(
    stream: &mut UnixStream,
    buffer: &mut Vec<u8>,
    eof: &mut bool,
    limit: usize,
) -> Result<bool, String> {
    let mut chunk = [0u8; 8192];
    match stream.read(&mut chunk) {
        Ok(0) => {
            *eof = true;
            Ok(true)
        }
        Ok(n) if n <= limit.saturating_sub(buffer.len()) => {
            buffer.extend_from_slice(&chunk[..n]);
            Ok(true)
        }
        Ok(_) => Err(format!(
            "output exceeds {} byte limit; acquisition incomplete",
            limit
        )),
        Err(e) if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::Interrupted) => Ok(false),
        Err(e) => Err(format!("read failed: {}", e)),
    }
}

// Public library callers must own child reaping and keep SIGCHLD disposition
// stable throughout collection. Preflight cannot exclude a concurrent waiter or
// signal mutation in another thread. The standalone CLI installs neither.
#[cfg(all(target_os = "linux", target_arch = "x86_64", target_env = "gnu"))]
#[repr(C)]
struct SigchldAction {
    handler: usize,
    mask: [u64; 16],
    flags: i32,
    restorer: usize,
}

#[cfg(all(target_os = "linux", target_arch = "x86_64", target_env = "gnu"))]
unsafe extern "C" {
    fn sigaction(signal: i32, action: *const SigchldAction, old: *mut SigchldAction) -> i32;
}

#[cfg(all(target_os = "linux", target_arch = "x86_64", target_env = "gnu"))]
fn ensure_waitable_children() -> Result<(), String> {
    // ABI is qualified against Linux x86_64 glibc's signal.h, not guessed for
    // other targets: handler, 1024-bit mask, int flags, aligned restorer.
    let mut action = SigchldAction {
        handler: 0,
        mask: [0; 16],
        flags: 0,
        restorer: 0,
    };
    let result = unsafe { sigaction(17, std::ptr::null(), &mut action) };
    if result != 0 {
        return Err(format!(
            "cannot inspect SIGCHLD disposition: {}",
            std::io::Error::last_os_error()
        ));
    }
    if action.handler != 0 || action.flags & 2 != 0 {
        return Err("unsupported SIGCHLD disposition: default handler without SA_NOCLDWAIT required; command not spawned".to_string());
    }
    Ok(())
}

#[cfg(not(all(target_os = "linux", target_arch = "x86_64", target_env = "gnu")))]
fn ensure_waitable_children() -> Result<(), String> {
    Err(
        "command collection signal ABI is qualified only for Linux x86_64 GNU; command not spawned"
            .to_string(),
    )
}

fn terminate_command(child: &mut Child) -> String {
    // Linux/POSIX kill targets a process group for a negative pid. The child
    // was created in its own group and has not been reaped on these paths.
    // No ambient process group or unrelated host process is signaled.
    unsafe extern "C" {
        fn kill(pid: i32, signal: i32) -> i32;
    }
    let signal_result = unsafe { kill(-(child.id() as i32), 9) };
    let signal_error = (signal_result != 0).then(std::io::Error::last_os_error);
    let cleanup_start = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => {
                return signal_error.map_or_else(String::new, |e| format!("; group signal: {}", e))
            }
            Ok(None) if cleanup_start.elapsed() < COMMAND_CLEANUP_ALLOWANCE => {
                thread::sleep(Duration::from_millis(1));
            }
            Ok(None) => return "; cleanup incomplete: child not reaped within 100ms".to_string(),
            Err(e) => return format!("; cleanup incomplete: {}", e),
        }
    }
}

#[cfg(all(test, target_os = "linux", target_arch = "x86_64", target_env = "gnu"))]
mod command_tests {
    use super::*;

    #[test]
    fn nonwaitable_sigchld_dispositions_refuse_before_spawn() {
        const MODE: &str = "ALIBI_TEST_SIGCHLD_MODE";
        if let Ok(mode) = std::env::var(MODE) {
            // This branch runs only in a fresh single-test subprocess, never
            // in the parallel test runner or an arbitrary library caller.
            let action = SigchldAction {
                handler: if mode == "ignored" { 1 } else { 0 },
                mask: [0; 16],
                flags: if mode == "no-child-wait" { 2 } else { 0 },
                restorer: 0,
            };
            assert_eq!(unsafe { sigaction(17, &action, std::ptr::null_mut()) }, 0);
            let error = run_command("/alibi-fixture-must-not-exist", &[], Duration::from_secs(1))
                .unwrap_err();
            assert!(error.contains("unsupported SIGCHLD disposition"), "{error}");
            assert!(error.contains("command not spawned"));
            return;
        }
        for mode in ["ignored", "no-child-wait"] {
            let output = Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "collect::command_tests::nonwaitable_sigchld_dispositions_refuse_before_spawn",
                ])
                .env(MODE, mode)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{mode}: {} {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }

    #[test]
    fn inherited_streams_are_inside_acquisition_deadline() {
        for script in [
            "sleep 2 2>/dev/null & printf ok; exit 0",
            "sleep 2 >/dev/null & printf ok; exit 0",
        ] {
            let start = Instant::now();
            let result = run_command("/bin/sh", &["-c", script], Duration::from_millis(50));
            assert!(result.unwrap_err().contains("timed out"));
            assert!(start.elapsed() < Duration::from_secs(1));
        }
    }

    #[test]
    fn active_child_timeout_is_bounded() {
        let start = Instant::now();
        assert!(
            run_command("/bin/sh", &["-c", "sleep 2"], Duration::from_millis(50))
                .unwrap_err()
                .contains("timed out")
        );
        assert!(start.elapsed() < Duration::from_secs(1));
    }

    #[test]
    fn complete_command_success_and_nonzero_exit_are_distinct() {
        assert_eq!(
            run_command("/bin/sh", &["-c", "printf ok"], Duration::from_secs(1)),
            Ok("ok".to_string())
        );
        assert!(run_command(
            "/bin/sh",
            &["-c", "printf ok; exit 1"],
            Duration::from_secs(1)
        )
        .is_err());
    }

    #[test]
    fn invalid_utf8_and_output_cap_never_succeed() {
        assert!(
            run_command("/bin/sh", &["-c", "printf '\\377'"], Duration::from_secs(1))
                .unwrap_err()
                .contains("UTF-8")
        );
        for script in ["printf 123456789", "printf 123456789 >&2"] {
            assert!(
                run_command_limited("/bin/sh", &["-c", script], Duration::from_secs(1), 8)
                    .unwrap_err()
                    .contains("byte limit")
            );
        }
    }
}
