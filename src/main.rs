use alibi::{collect, eval, explain, receipt};
use std::process::ExitCode;
use std::time::{Instant, SystemTime};

const VALID_ACCUSATIONS: &str = "kernel-scream, silent-killer, memrot, network-choke-point";

const USAGE: &str = "\
alibi — read-only Linux host interrogation

usage:
  alibi check [--json] [--verbose] [accusation...]
  alibi explain <accusation>
  alibi --version

accusations: kernel-scream, silent-killer, memrot, network-choke-point

exit codes: 0 innocence established · 1 degraded · 2 unknown · 64 usage
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("check") => cmd_check(&args[1..]),
        Some("explain") => cmd_explain(&args[1..]),
        Some("--version") | Some("-V") => {
            println!("alibi {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        Some("--help") | Some("-h") => {
            print!("{}", USAGE);
            ExitCode::SUCCESS
        }
        None => {
            eprint!("{}", USAGE);
            ExitCode::from(64)
        }
        Some(other) => {
            eprintln!("alibi: unknown subcommand: {}", other);
            eprintln!("valid subcommands: check, explain");
            ExitCode::from(64)
        }
    }
}

fn cmd_check(args: &[String]) -> ExitCode {
    let mut json = false;
    let mut verbose = false;
    let mut requested: Vec<&str> = Vec::new();
    for arg in args {
        match arg.as_str() {
            "--json" => json = true,
            "--verbose" | "-v" => verbose = true,
            "--help" | "-h" => {
                print!("{}", USAGE);
                return ExitCode::SUCCESS;
            }
            s if s.starts_with('-') => {
                eprintln!("alibi: unknown flag: {}", s);
                return ExitCode::from(64);
            }
            s => requested.push(s),
        }
    }
    let registry = eval::registry();
    for name in &requested {
        if !registry.iter().any(|e| e.accusation() == *name) {
            eprintln!("alibi: unknown accusation: {}", name);
            eprintln!("valid accusations: {}", VALID_ACCUSATIONS);
            return ExitCode::from(64);
        }
    }
    let evaluators: Vec<_> = registry
        .into_iter()
        .filter(|e| requested.is_empty() || requested.contains(&e.accusation()))
        .collect();

    let t0 = Instant::now();
    let started = SystemTime::now();
    let facts = collect::collect_facts();
    let mut findings = Vec::new();
    for evaluator in &evaluators {
        let mut finding = evaluator.evaluate(&facts);
        finding.time_to_verdict = t0.elapsed();
        findings.push(finding);
    }
    let elapsed = t0.elapsed();
    let code = receipt::exit_code(&findings);
    let receipt = receipt::build(&facts, findings, started, elapsed);
    if json {
        println!("{}", receipt::json::render(&receipt));
    } else {
        print!("{}", receipt::human::render(&receipt, verbose));
    }
    ExitCode::from(code)
}

fn cmd_explain(args: &[String]) -> ExitCode {
    if args.len() != 1 {
        eprintln!("alibi: explain takes exactly one accusation");
        eprintln!("valid accusations: {}", VALID_ACCUSATIONS);
        return ExitCode::from(64);
    }
    match explain::explain(&args[0]) {
        Some(text) => {
            print!("{}", text);
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("alibi: unknown accusation: {}", args[0]);
            eprintln!("valid accusations: {}", VALID_ACCUSATIONS);
            ExitCode::from(64)
        }
    }
}
