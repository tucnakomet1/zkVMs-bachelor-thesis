use alloy_sol_types::private::FixedBytes;
use alloy_sol_types::SolType;
use sha256_lib::{sha256, PublicValuesStruct};

/// This function runs the SHA256 hash function n times - used for EVM
pub fn run_sha256_lib(n: u32) -> Vec<u8> {
    // Call the SHA256 function from the lib/src/lib.rs
    // The result of the hash function after `n` iterations.
    let hash_result = sha256(n);

    // Encode the public values of the program.
    let public_values = PublicValuesStruct {
        n,
        hash: FixedBytes::from(hash_result),
    };

    let bytes = PublicValuesStruct::abi_encode(&public_values);

    // Commit to the public values of the program.
    // The final proof will have a commitment to all the bytes that were committed to.
    bytes
}