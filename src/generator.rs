use super::*;
use rayon::prelude::*;
use std::sync::{atomic::AtomicU64, Arc};

use utils::parse_hex_addr;
use wallet::Wallet;

#[derive(Debug)]
pub struct Generator {
    pub starts_with: String,
    pub total_iterations: Arc<AtomicU64>,
}

impl Generator {
    pub fn new(starts_with: String) -> Self {
        Self {
            starts_with,
            total_iterations: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn run(&self) -> Wallet {
        let secp = Secp256k1::new();
        let starts_with_bytes = parse_hex_addr(&self.starts_with);

        (0..)
            .par_bridge()
            .map(|_| Wallet::generate(&secp))
            .inspect(|_| {
                self.total_iterations
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            })
            .filter(|wallet| starts_with_bytes == wallet.address[..starts_with_bytes.len()])
            .take_any(1)
            .collect::<Vec<_>>()
            .first()
            .unwrap()
            .clone()
    }
}
