use sha2_v0_10_8::{Digest, Sha256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Sha256Output {
    pub sha256hash: Vec<[u8; 32]>
}

/// This function runs the SHA256 hash function n times
pub fn run_sha256(n: u32) -> Sha256Output {
    let mut result = Vec::with_capacity(n as usize);

    let input = [123u8; 32];      // 123 is just a random number

    // hash the input n times
    for _ in 0..n {
        let mut hasher = Sha256::new();
        hasher.update(&input);
        
        let hash = hasher.finalize();
        result.push(hash.into());
    }

    let output: Sha256Output = Sha256Output {
        sha256hash: result
    };

    // return the sha256 hash
    output
}