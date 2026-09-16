use crate::receipt::{Receipt, TOOL, VERSION};
use std::time::Duration;

#[derive(Debug)]
pub enum Json {
    Null,
    Bool(bool),
    Num(u64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

impl Json {
    pub fn render(&self) -> String {
        let mut out = String::new();
        self.write(&mut out, 0);
        out
    }

    fn write(&self, out: &mut String, indent: usize) {
        match self {
            Json::Null => out.push_str("null"),
            Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
            Json::Num(n) => out.push_str(&n.to_string()),
            Json::Str(s) => escape_into(s, out),
            Json::Array(items) => {
                if items.is_empty() {
                    out.push_str("[]");
                    return;
                }
                out.push_str("[\n");
                for (i, item) in items.iter().enumerate() {
                    push_indent(out, indent + 1);
                    item.write(out, indent + 1);
                    if i + 1 < items.len() {
                        out.push(',');
                    }
                    out.push('\n');
                }
                push_indent(out, indent);
                out.push(']');
            }
            Json::Object(pairs) => {
                if pairs.is_empty() {
                    out.push_str("{}");
                    return;
                }
                out.push_str("{\n");
                for (i, (key, value)) in pairs.iter().enumerate() {
                    push_indent(out, indent + 1);
                    escape_into(key, out);
                    out.push_str(": ");
                    value.write(out, indent + 1);
                    if i + 1 < pairs.len() {
                        out.push(',');
                    }
                    out.push('\n');
                }
                push_indent(out, indent);
                out.push('}');
            }
        }
    }
}

fn push_indent(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push_str("  ");
    }
}

pub fn escape_into(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

pub fn receipt_json(receipt: &Receipt) -> Json {
    let findings = receipt
        .findings
        .iter()
        .map(|f| {
            Json::Object(vec![
                ("accusation".to_string(), Json::Str(f.accusation.to_string())),
                ("verdict".to_string(), Json::Str(f.verdict.as_str().to_string())),
                ("summary".to_string(), Json::Str(f.summary.clone())),
                ("indications".to_string(), strings(&f.indications)),
                ("evidence".to_string(), strings(&f.evidence)),
                ("skipped".to_string(), strings(&f.skipped)),
                (
                    "time_to_verdict_ms".to_string(),
                    Json::Num(millis(f.time_to_verdict)),
                ),
            ])
        })
        .collect();

    let sources = receipt
        .sources
        .iter()
        .map(|s| {
            let mut obj = vec![
                ("name".to_string(), Json::Str(s.name.to_string())),
                ("status".to_string(), Json::Str(s.status.as_str().to_string())),
            ];
            if let Some(method) = &s.method {
                obj.push(("method".to_string(), Json::Str(method.clone())));
            }
            if let Some(detail) = &s.detail {
                obj.push(("detail".to_string(), Json::Str(detail.clone())));
            }
            obj.push(("elapsed_ms".to_string(), Json::Num(millis(s.elapsed))));
            Json::Object(obj)
        })
        .collect();

    let boot_time = match receipt.boot_time {
        Some(epoch) => Json::Str(crate::timeutil::rfc3339_utc(epoch)),
        None => Json::Null,
    };

    Json::Object(vec![
        ("tool".to_string(), Json::Str(TOOL.to_string())),
        ("version".to_string(), Json::Str(VERSION.to_string())),
        ("host".to_string(), Json::Str(receipt.host.clone())),
        (
            "started_at".to_string(),
            Json::Str(receipt.started_at.clone()),
        ),
        ("elapsed_ms".to_string(), Json::Num(millis(receipt.elapsed))),
        ("mtti_ms".to_string(), Json::Num(millis(receipt.mtti))),
        (
            "innocence_established".to_string(),
            Json::Bool(receipt.innocence_established),
        ),
        (
            "verdict".to_string(),
            Json::Str(receipt.verdict.as_str().to_string()),
        ),
        ("findings".to_string(), Json::Array(findings)),
        (
            "envelope".to_string(),
            Json::Object(vec![
                ("boot_time".to_string(), boot_time),
                ("sources".to_string(), Json::Array(sources)),
            ]),
        ),
        (
            "semantics".to_string(),
            Json::Object(vec![
                (
                    "clear".to_string(),
                    Json::Str(
                        "all required evidence sources were collected and no collected evidence indicates the failure class"
                            .to_string(),
                    ),
                ),
                (
                    "degraded".to_string(),
                    Json::Str(
                        "collected evidence contains at least one positive indication of the failure class"
                            .to_string(),
                    ),
                ),
                (
                    "unknown".to_string(),
                    Json::Str(
                        "no indication was found but at least one required evidence source was unavailable, unsupported, inaccessible, or malformed; innocence was not established"
                            .to_string(),
                    ),
                ),
                (
                    "mtti".to_string(),
                    Json::Str(
                        "duration from interrogation start until every requested accusation reached a terminal verdict; unknown ends the clock but does not establish innocence"
                            .to_string(),
                    ),
                ),
            ]),
        ),
    ])
}

pub fn render(receipt: &Receipt) -> String {
    receipt_json(receipt).render()
}

fn strings(items: &[String]) -> Json {
    Json::Array(items.iter().map(|s| Json::Str(s.clone())).collect())
}

fn millis(d: Duration) -> u64 {
    d.as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::Verdict;

    #[test]
    fn escapes_special_characters() {
        let mut out = String::new();
        escape_into("a\"b\\c\nd\u{1}e", &mut out);
        assert_eq!(out, "\"a\\\"b\\\\c\\nd\\u0001e\"");
    }

    #[test]
    fn renders_scalar_types() {
        assert_eq!(Json::Null.render(), "null");
        assert_eq!(Json::Bool(true).render(), "true");
        assert_eq!(Json::Num(42).render(), "42");
        assert_eq!(Json::Str("x".to_string()).render(), "\"x\"");
        assert_eq!(Json::Array(vec![]).render(), "[]");
        assert_eq!(Json::Object(vec![]).render(), "{}");
    }

    #[test]
    fn receipt_contains_required_top_level_keys() {
        let receipt = Receipt {
            host: "h".to_string(),
            started_at: "2026-09-15T00:00:00Z".to_string(),
            elapsed: Duration::from_millis(5),
            mtti: Duration::from_millis(4),
            innocence_established: true,
            verdict: Verdict::Clear,
            findings: vec![],
            boot_time: None,
            sources: vec![],
        };
        let rendered = render(&receipt);
        for key in [
            "\"tool\"",
            "\"version\"",
            "\"host\"",
            "\"started_at\"",
            "\"elapsed_ms\"",
            "\"mtti_ms\"",
            "\"innocence_established\"",
            "\"verdict\"",
            "\"findings\"",
            "\"envelope\"",
            "\"semantics\"",
        ] {
            assert!(rendered.contains(key), "missing key {}", key);
        }
        assert!(rendered.contains("\"boot_time\": null"));
    }
}
