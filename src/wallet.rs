use super::*;
use secp256k1::rand::rngs::OsRng;
use tiny_keccak::{Hasher, Keccak};

#[derive(Debug, Clone)]
pub struct Wallet {
    pub key: SecretKey,
    pub address: [u8; 20],
}

impl Wallet {
    #[inline]
    pub fn generate(secp: &Secp256k1<secp256k1::All>) -> Self {
        let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
        let public_key_bytes = public_key.serialize_uncompressed();

        let mut keccak = Keccak::v256();
        let mut output = [0u8; 32];
        keccak.update(&public_key_bytes[1..]);
        keccak.finalize(&mut output);

        Wallet {
            key: secret_key,
            address: output[12..]
                .try_into()
                .expect("slice with incorrect length"),
        }
    }

    pub fn to_address_hex(&self) -> String {
        let mut hex = String::new();
        for byte in self.address.iter() {
            hex += &format!("{:02x}", byte);
        }
        hex
    }
}
