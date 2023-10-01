use super::*;
use rayon::prelude::*;
use std::{
    sync::{atomic::AtomicU64, Arc},
    time::Instant,
};

use utils::parse_hex_addr;
use wallet::Wallet;

#[derive(Debug)]
pub struct Generator {
    pub starts_with: String,
    pub count: u64,
    pub total_iterations: Arc<AtomicU64>,
}

impl Generator {
    pub fn new(starts_with: String, count: u64) -> Self {
        Self {
            starts_with,
            count,
            total_iterations: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn run(&self) {
        let secp = Secp256k1::new();
        let starts_with_bytes = parse_hex_addr(&self.starts_with);
        let start = Instant::now();

        (0..)
            .par_bridge()
            .map(|_| Wallet::generate(&secp))
            .inspect(|_| {
                self.total_iterations
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            })
            .filter(|wallet| starts_with_bytes == wallet.address[..starts_with_bytes.len()])
            .take_any(self.count as usize)
            .inspect(|wallet| {
                println!(
                    "\nSuccessfully found vanity address in {:?}",
                    start.elapsed()
                );
                println!("Address: 0x{}", wallet.to_address_hex());
                println!("Private Key: 0x{}", wallet.key.display_secret());
            })
            .collect::<Vec<_>>();
    }
}
