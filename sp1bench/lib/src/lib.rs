use alloy_sol_types::sol;
use sha2_v0_10_8::{Digest, Sha256};

// Define the Solidity-compatible struct for public values
sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 n;         // Number of hashes to compute
        bytes32 hash;     // The output of the hash function
    }
}

/// Compute the SHA256 hash of a 32-byte input, n times, and commit the result.
pub fn sha256(n: u32) -> [u8; 32] {
    // Fixed init input for the hash function
    let mut hash = [123u8; 32];      // 123 is just a random number

    // Hash the input n times
    for _ in 0..n {
        let mut hasher = Sha256::new();
        hasher.update(&hash);
        hash = hasher.clone().finalize().into();
    }
    hash
}
