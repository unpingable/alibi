use std::time::{Duration, Instant};
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("route-mask") => {
            let header = "Iface Destination Gateway Flags RefCnt Use Metric Mask MTU Window IRTT\n";
            for (name, mask) in [("zero-prefix-8", "000000FF"), ("zero-host-32", "FFFFFFFF"), ("default-control", "00000000")] {
                let row = format!("{}eth0 00000000 00000000 0001 0 0 0 {} 0 0 0\n", header, mask);
                println!("{} input={:?} actual={:?}", name, row, alibi::collect::net::parse_v4_default(&row));
            }
        }
        Some("pipe-timeout") => {
            let started = Instant::now();
            let result = alibi::collect::run_command("/bin/sh", &["-c", "sleep 1 & printf 'benign\\n'; exit 0"], Duration::from_millis(100));
            println!("timeout_ms=100 elapsed_ms={} result={:?}", started.elapsed().as_millis(), result);
        }
        Some("malformed-parsers") => {
            println!("df free>total: {:?}", alibi::collect::fsusage::parse_df_bytes("Filesystem 1B-blocks Used Available Capacity Mounted on\n/dev/other 100 0 101 0% /other\n"));
            println!("mount invalid options: {:?}", alibi::collect::mounts::parse_mounts("/dev/vda / ext4 nonsense 0 0\n"));
            let outcome = std::panic::catch_unwind(|| alibi::collect::mounts::parse_mounts("/dev/vda /\\éé ext4 rw 0 0\n"));
            println!("mount non-ASCII escape panic={}", outcome.is_err());
        }
        _ => panic!("unknown reproducer"),
    }
}
