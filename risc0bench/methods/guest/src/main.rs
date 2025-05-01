mod sha256;
mod poseidon;
mod fibonacci;
mod poseidon_risc0;

use risc0_zkvm::guest::env;

fn main() {
    // Read the input and method from the environment
    let input: u32 = env::read();
    let method: u32 = env::read();

    // Call the appropriate method based on the input
    // and commit the result to the environment
    match method {
        0 => env::commit(&fibonacci::run_fibonacci(input)),
        1 => env::commit(&sha256::run_sha256(input)),
        2 => env::commit(&poseidon::run_poseidon(input)),
        3 => env::commit(&poseidon_risc0::run_poseidon(input)),
        _ => panic!("Invalid method specified"),
    }
}
