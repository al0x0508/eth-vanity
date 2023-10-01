use std::io::{self, Write};
use std::sync::{atomic::AtomicU64, Arc};

pub fn parse_hex_addr(hex: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    let mut chars = hex.chars();
    while let (Some(a), Some(b)) = (chars.next(), chars.next()) {
        let byte = u8::from_str_radix(&format!("{}{}", a, b), 16).unwrap();
        bytes.push(byte);
    }
    bytes
}

fn convert_hash(hash_value: u64) -> String {
    let (value, unit) = if hash_value >= 1_000_000_000_000_000 {
        (hash_value / 1_000_000_000_000_000, "PH")
    } else if hash_value >= 1_000_000_000_000 {
        (hash_value / 1_000_000_000_000, "TH")
    } else if hash_value >= 1_000_000_000 {
        (hash_value / 1_000_000_000, "GH")
    } else if hash_value >= 1_000_000 {
        (hash_value / 1_000_000, "MH")
    } else if hash_value >= 1_000 {
        (hash_value / 1_000, "KH")
    } else {
        (hash_value, "H")
    };

    format!("{}{}", value, unit)
}

pub fn print_iterations(total_iterations: Arc<AtomicU64>) {
    let start = std::time::Instant::now();
    loop {
        let elapsed_seconds = start.elapsed().as_secs();
        if elapsed_seconds == 0 {
            continue;
        }

        let total_iterations = total_iterations.load(std::sync::atomic::Ordering::Relaxed);
        io::stdout().flush().unwrap();
        print!(
            "\rSpeed: {}/s",
            convert_hash(total_iterations / elapsed_seconds)
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
