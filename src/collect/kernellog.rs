use crate::collect::{run_command, Source, Status};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct KernelLog {
    pub lines: Vec<String>,
    pub method: String,
}

const DMESG_ARGS: &[&str] = &["--raw", "--force-prefix"];
const DMESG_METHOD: &str = "dmesg --raw --force-prefix";
const JOURNAL_ARGS: &[&str] = &[
    "-k",
    "-b",
    "--no-pager",
    "--quiet",
    "-o",
    "short-monotonic",
    "--no-hostname",
    "--all",
];
const JOURNAL_METHOD: &str =
    "journalctl -k -b --no-pager --quiet -o short-monotonic --no-hostname --all";

pub fn collect() -> Source<KernelLog> {
    let start = Instant::now();
    let dmesg_error = match run_command("dmesg", DMESG_ARGS, Duration::from_secs(3)) {
        Ok(out) => match parse_output(&out, false, DMESG_METHOD, start.elapsed()) {
            Ok(source) => return source,
            Err(error) => error,
        },
        Err(error) => error,
    };
    match run_command("journalctl", JOURNAL_ARGS, Duration::from_secs(5)) {
        Ok(out) => match parse_output(&out, true, JOURNAL_METHOD, start.elapsed()) {
            Ok(mut source) => {
                let fallback_detail = format!(
                    "dmesg acquisition failed: {}; used journal fallback",
                    dmesg_error
                );
                source.detail = Some(match source.detail.take() {
                    Some(detail) => format!("{}; {}", detail, fallback_detail),
                    None => fallback_detail,
                });
                source
            }
            Err(error) => Source::failed(
                Status::Malformed,
                format!("{}; {}", dmesg_error, error),
                start.elapsed(),
            ),
        },
        Err(error) => Source::failed(
            Status::Unavailable,
            format!("{}; {}", dmesg_error, error),
            start.elapsed(),
        ),
    }
}

// These checks establish the requested command representation, not provenance,
// freshness, or complete since-boot retention. Message bodies remain opaque.
fn parse_output(
    output: &str,
    journal: bool,
    method: &str,
    elapsed: Duration,
) -> Result<Source<KernelLog>, String> {
    let mut lines: Vec<String> = Vec::new();
    let mut malformed = 0;
    let mut continuation_indent = None;
    for record in output.split_inclusive('\n') {
        let line = record.trim_end_matches('\n');
        if record.ends_with('\n') && valid_record(line, journal) {
            continuation_indent =
                journal.then(|| line.find(": ").expect("validated identifier") + 2);
            lines.push(line.to_string());
        } else if record.ends_with('\n')
            && !line.chars().any(|c| c.is_control() && c != '\t')
            && continuation_indent.is_some_and(|indent| {
                line.as_bytes()
                    .get(..indent)
                    .is_some_and(|prefix| prefix.iter().all(|b| *b == b' '))
            })
        {
            // short-monotonic aligns continuation lines with the message
            // column. Treat them as part of their framed predecessor, not
            // as separate records or as proof of message provenance.
            let previous = lines
                .last_mut()
                .expect("continuation has framed predecessor");
            previous.push('\n');
            previous.push_str(line);
        } else {
            malformed += 1;
            continuation_indent = None;
        }
    }
    if lines.is_empty() {
        return Err(format!(
            "{}: no complete records in requested log representation ({} invalid records)",
            method, malformed
        ));
    }
    let log = KernelLog {
        lines,
        method: method.to_string(),
    };
    if malformed == 0 {
        Ok(Source::ok(log, method, elapsed))
    } else {
        Ok(Source::partial(log, Status::Malformed,
            format!("{} record(s) do not match requested log representation; retained valid records only", malformed),
            method, elapsed))
    }
}

fn valid_record(record: &str, journal: bool) -> bool {
    if record.chars().any(|c| c.is_control() && c != '\t') {
        return false;
    }
    let framed = if journal {
        record
    } else {
        let Some((priority, rest)) = record.strip_prefix('<').and_then(|s| s.split_once('>'))
        else {
            return false;
        };
        if priority.is_empty()
            || !priority.bytes().all(|b| b.is_ascii_digit())
            || priority.parse::<u16>().map_or(true, |n| n > 191)
        {
            return false;
        }
        rest
    };
    let Some((timestamp, body)) = framed.strip_prefix('[').and_then(|s| s.split_once(']')) else {
        return false;
    };
    let Some((seconds, fraction)) = timestamp.trim().split_once('.') else {
        return false;
    };
    if seconds.is_empty()
        || !seconds.bytes().all(|b| b.is_ascii_digit())
        || fraction.is_empty()
        || !fraction.bytes().all(|b| b.is_ascii_digit())
    {
        return false;
    }
    let Some(body) = body.strip_prefix(' ') else {
        return false;
    };
    if journal {
        // Early userspace records written through kmsg may carry e.g.
        // systemd[1], even under journalctl -k. Validate the selected framing,
        // not an invented requirement that every identifier is "kernel".
        let Some((identifier, _message)) = body.split_once(": ") else {
            return false;
        };
        if identifier.is_empty() || identifier.chars().any(char::is_whitespace) {
            return false;
        }
    }
    // Empty messages are legal within an explicit complete frame.
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collect::Completeness;

    #[test]
    fn edac_journal_continuation_is_not_a_clean_negative() {
        use crate::eval::{fixtures, memrot::Memrot, Evaluator, Verdict};
        let first = "[ 1.0] kernel: EDAC MC0: Giving out device to module sample controller sample: DEV sample (POLLED)";
        let indent = " ".repeat(first.find(": ").unwrap() + 2);
        let output = format!("{first}\n{indent}EDAC MC0: unsupported error report\n");
        let source = parse_output(&output, true, "journal fixture", Duration::ZERO).unwrap();
        assert!(source.is_complete());
        assert_eq!(source.get().unwrap().lines.len(), 1);
        let mut facts = fixtures::clean_facts();
        facts.kernel_log = source;
        assert_eq!(Memrot.evaluate(&facts).verdict, Verdict::Unknown);
    }

    #[test]
    fn edac_journal_positive_grammar_stays_on_first_physical_line() {
        use crate::eval::{fixtures, memrot::Memrot, Evaluator, Verdict};
        for (report, expected) in [
            ("2 CE", Verdict::Unknown),
            ("2 CE memory read error on DIMM0", Verdict::Degraded),
        ] {
            let first = format!("[ 1.0] kernel: EDAC MC0: {report}");
            let indent = " ".repeat(first.find(": ").unwrap() + 2);
            let output = format!("{first}\n{indent}additional event information\n");
            let source = parse_output(&output, true, "journal fixture", Duration::ZERO).unwrap();
            assert!(source.is_complete());
            let mut facts = fixtures::clean_facts();
            facts.kernel_log = source;
            assert_eq!(Memrot.evaluate(&facts).verdict, expected);
        }
    }

    #[test]
    fn explicit_representations_accept_benign_records() {
        for (text, journal) in [
            ("<6>[    0.000000] Linux version test\n", false),
            ("[    0.000000] kernel: Linux version test\n", true),
            (
                "[   13.949319] systemd[1]: Detected architecture x86-64.\n",
                true,
            ),
        ] {
            let source = parse_output(text, journal, "fixture", Duration::ZERO).unwrap();
            assert!(source.is_complete());
            assert_eq!(source.get().unwrap().lines.len(), 1);
        }
    }

    #[test]
    fn nonlog_empty_invalid_and_truncated_records_are_inadequate() {
        for text in [
            "",
            "\n",
            "not a kernel log\n",
            "-- No entries --\n",
            "<x>[    0.000000] message\n",
            "<192>[    0.000000] message\n",
            "<6>[now] message\n",
            "<6>[    0.000000]missing separator\n",
            "<6>[    0.000000] message",
        ] {
            assert!(
                parse_output(text, false, "fixture", Duration::ZERO).is_err(),
                "{text:?}"
            );
        }
    }

    #[test]
    fn journal_empty_and_aligned_multiline_messages_are_complete() {
        let output = "[   14.874158] kernel: \n[25385.444274] kernel: first line\n                        continuation\n";
        let source = parse_output(output, true, "fixture", Duration::ZERO).unwrap();
        assert!(source.is_complete());
        assert_eq!(source.get().unwrap().lines.len(), 2);
        assert!(source.get().unwrap().lines[1].contains("continuation"));
        assert!(parse_output(
            "                        continuation\n",
            true,
            "fixture",
            Duration::ZERO
        )
        .is_err());
    }

    #[test]
    fn valid_positive_record_survives_malformed_neighbor_as_partial() {
        let source = parse_output(
            "<3>[    1.000000] Oops: example\ninvalid record\n",
            false,
            "fixture",
            Duration::ZERO,
        )
        .unwrap();
        assert_eq!(source.completeness, Completeness::Partial);
        assert_eq!(source.status, Status::Malformed);
        assert_eq!(
            source.get().unwrap().lines,
            vec!["<3>[    1.000000] Oops: example"]
        );
    }
}
