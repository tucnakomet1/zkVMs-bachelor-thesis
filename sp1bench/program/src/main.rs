//! A simple program that takes a number `n` as input and computes the 'n' SHA256 hashes

#![no_main]

mod fibonacci;
mod poseidon;
mod sha256;
mod sha256_evm;

sp1_zkvm::entrypoint!(main);

pub fn main() {
    // Read an input to the program.
    // This compiles down to a custom system call which handles reading inputs from the prover.
    let input: u32 = sp1_zkvm::io::read::<u32>();
    let method: u32 = sp1_zkvm::io::read::<u32>();

    // Call the appropriate method based on the input
    // and commit the result to the environment
    match method {
        0 => sp1_zkvm::io::commit(&fibonacci::run_fibonacci(input)),
        1 => sp1_zkvm::io::commit(&sha256::run_sha256(input)),
        2 => sp1_zkvm::io::commit(&poseidon::run_poseidon(input)),
        3 => sp1_zkvm::io::commit(&sha256_evm::run_sha256_lib(input)),  // for evm
        4 => sp1_zkvm::io::commit(&sha256_evm::run_sha256_lib(input)),  // for evm
        _ => panic!("Invalid method specified"),
    }
}