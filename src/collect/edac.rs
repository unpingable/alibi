use crate::collect::{Source, Status};
use std::fs;
use std::io::ErrorKind;
use std::time::{Duration, Instant};

const EDAC_MC_DIR: &str = "/sys/devices/system/edac/mc";

#[derive(Debug)]
pub struct Edac {
    pub controllers: Vec<EdacController>,
}

#[derive(Debug)]
pub struct EdacController {
    pub name: String,
    pub ue: Option<u64>,
    pub ce: Option<u64>,
}

pub fn collect() -> Source<Edac> {
    let start = Instant::now();
    let entries = match fs::read_dir(EDAC_MC_DIR) {
        Ok(d) => d,
        Err(error) if error.kind() == ErrorKind::NotFound => return unsupported(start.elapsed()),
        Err(error) => {
            return Source::failed(
                Status::Unavailable,
                format!("{}: {}", EDAC_MC_DIR, error),
                start.elapsed(),
            );
        }
    };
    let mut names = Vec::new();
    let mut issues = Vec::new();
    for entry in entries {
        match entry {
            Ok(entry) => {
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.starts_with("mc") {
                    names.push(name);
                }
            }
            Err(error) => issues.push(format!("{}: directory entry: {}", EDAC_MC_DIR, error)),
        }
    }
    names.sort();
    if names.is_empty() && issues.is_empty() {
        return unsupported(start.elapsed());
    }
    let mut controllers = Vec::new();
    let mut issue_status = Status::Unavailable;
    for name in names {
        let ue = match read_counter(&name, "ue_count") {
            Ok(value) => Some(value),
            Err((status, detail)) => {
                if status == Status::Malformed {
                    issue_status = Status::Malformed;
                }
                issues.push(detail);
                None
            }
        };
        let ce = match read_counter(&name, "ce_count") {
            Ok(value) => Some(value),
            Err((status, detail)) => {
                if status == Status::Malformed {
                    issue_status = Status::Malformed;
                }
                issues.push(detail);
                None
            }
        };
        controllers.push(EdacController { name, ue, ce });
    }
    let facts = Edac { controllers };
    if issues.is_empty() {
        Source::ok(facts, EDAC_MC_DIR, start.elapsed())
    } else if facts
        .controllers
        .iter()
        .any(|controller| controller.ue.is_some() || controller.ce.is_some())
    {
        Source::partial(
            facts,
            issue_status,
            issues.join("; "),
            EDAC_MC_DIR,
            start.elapsed(),
        )
    } else {
        Source::failed(issue_status, issues.join("; "), start.elapsed())
    }
}

fn unsupported(elapsed: Duration) -> Source<Edac> {
    Source::failed(
        Status::Unsupported,
        "platform exposes no EDAC (VM, or driver not loaded)",
        elapsed,
    )
}

fn read_counter(mc: &str, file: &str) -> Result<u64, (Status, String)> {
    let path = format!("{}/{}/{}", EDAC_MC_DIR, mc, file);
    let content =
        fs::read_to_string(&path).map_err(|e| (Status::Unavailable, format!("{}: {}", path, e)))?;
    content
        .trim()
        .parse()
        .map_err(|_| (Status::Malformed, format!("{}: not a number", path)))
}
