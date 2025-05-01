use risc0_zkvm::sha::{Impl, Digest, Sha256};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Sha256Output {
    pub sha256hash: Vec<Digest>
}

/// This function runs the SHA256 hash function n times
pub fn run_sha256(n: u32) -> Sha256Output {
    let mut result: Vec<Digest> = Vec::new();

    let input = [123u8; 32];      // 123 is just a random number

    // hash the input n times
    for _ in 0..n {
        let hash: Digest = *Impl::hash_bytes(&input);
        result.push(hash);
    }

    let output: Sha256Output = Sha256Output {
        sha256hash: result
    };

    // return the sha256 hash
    output
}
