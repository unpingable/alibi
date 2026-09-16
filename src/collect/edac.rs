use crate::collect::{Source, Status};
use std::fs;
use std::time::Instant;

const EDAC_MC_DIR: &str = "/sys/devices/system/edac/mc";

#[derive(Debug)]
pub struct Edac {
    pub controllers: Vec<(String, u64, u64)>,
}

pub fn collect() -> Source<Edac> {
    let start = Instant::now();
    let entries = match fs::read_dir(EDAC_MC_DIR) {
        Ok(d) => d,
        Err(_) => return unsupported(start.elapsed()),
    };
    let mut names: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with("mc"))
        .collect();
    names.sort();
    if names.is_empty() {
        return unsupported(start.elapsed());
    }
    let mut controllers = Vec::new();
    for name in names {
        let ue = match read_counter(&name, "ue_count") {
            Ok(v) => v,
            Err((status, detail)) => return Source::failed(status, detail, start.elapsed()),
        };
        let ce = match read_counter(&name, "ce_count") {
            Ok(v) => v,
            Err((status, detail)) => return Source::failed(status, detail, start.elapsed()),
        };
        controllers.push((name, ue, ce));
    }
    Source::ok(Edac { controllers }, EDAC_MC_DIR, start.elapsed())
}

fn unsupported(elapsed: std::time::Duration) -> Source<Edac> {
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
