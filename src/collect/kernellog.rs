use crate::collect::{run_command, Source, Status};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct KernelLog {
    pub lines: Vec<String>,
    pub method: String,
}

pub fn collect() -> Source<KernelLog> {
    let start = Instant::now();
    let dmesg_error = match run_command("dmesg", &[], Duration::from_secs(3)) {
        Ok(out) if usable_lines(&out, false).is_some() => {
            return Source::ok(
                KernelLog {
                    lines: usable_lines(&out, false).expect("checked above"),
                    method: "dmesg".to_string(),
                },
                "dmesg",
                start.elapsed(),
            );
        }
        Ok(_) => "dmesg produced no log lines".to_string(),
        Err(error) => error,
    };

    match run_command(
        "journalctl",
        &["-k", "-b", "--no-pager", "--quiet"],
        Duration::from_secs(5),
    ) {
        Ok(out) if usable_lines(&out, true).is_some() => {
            let method = "journalctl -k -b --no-pager --quiet";
            Source::ok(
                KernelLog {
                    lines: usable_lines(&out, true).expect("checked above"),
                    method: method.to_string(),
                },
                method,
                start.elapsed(),
            )
        }
        Ok(_) => Source::failed(
            Status::Malformed,
            format!("{}; journalctl produced no log lines", dmesg_error),
            start.elapsed(),
        ),
        Err(journal_error) => Source::failed(
            Status::Unavailable,
            format!("{}; {}", dmesg_error, journal_error),
            start.elapsed(),
        ),
    }
}

fn usable_lines(output: &str, journal: bool) -> Option<Vec<String>> {
    let lines: Vec<String> = output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect();
    if lines.is_empty() || (journal && lines.iter().all(|line| line.trim() == "-- No entries --")) {
        None
    } else {
        Some(lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_output_is_not_usable_evidence() {
        assert!(usable_lines("\n  \n", false).is_none());
    }

    #[test]
    fn journal_no_entries_sentinel_is_not_usable_evidence() {
        assert!(usable_lines("-- No entries --\n", true).is_none());
    }

    #[test]
    fn real_log_lines_are_retained() {
        assert_eq!(
            usable_lines("kernel: booted\n", true),
            Some(vec!["kernel: booted".to_string()])
        );
    }
}
