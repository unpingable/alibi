use crate::collect::{run_command, Source, Status};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct KernelLog {
    pub lines: Vec<String>,
    pub method: String,
}

pub fn collect() -> Source<KernelLog> {
    let start = Instant::now();
    match run_command("dmesg", &[], Duration::from_secs(3)) {
        Ok(out) => Source::ok(
            KernelLog {
                lines: to_lines(&out),
                method: "dmesg".to_string(),
            },
            "dmesg",
            start.elapsed(),
        ),
        Err(dmesg_err) => {
            match run_command("journalctl", &["-k", "-b", "--no-pager"], Duration::from_secs(5)) {
                Ok(out) => {
                    let method = "journalctl -k -b --no-pager";
                    Source::ok(
                        KernelLog {
                            lines: to_lines(&out),
                            method: method.to_string(),
                        },
                        method,
                        start.elapsed(),
                    )
                }
                Err(journal_err) => Source::failed(
                    Status::Unavailable,
                    format!("{}; {}", dmesg_err, journal_err),
                    start.elapsed(),
                ),
            }
        }
    }
}

fn to_lines(output: &str) -> Vec<String> {
    output.lines().map(str::to_string).collect()
}
