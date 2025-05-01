//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release
//! ```

mod parse_args;

use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use utils::size;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const SHA_ELF: &[u8] = include_elf!("sha256-program");

/// Initialization of logger - tracing
fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("NO_COLOR", "1");
    sp1_sdk::utils::setup_logger();

    dotenv::dotenv().ok();
}

fn main() {
    // Setup the logger.
    init_logger();

    // Parse command line arguments
    let (num_of_rep, inputs, function) = parse_args::parse_args();
    
    println!("Number of repetitions: {}; Function: {}", num_of_rep, function);

    for input in inputs {
        for i in 0..num_of_rep {
            println!("Run: {}; Input: {}", i, input);
            run_benchmark(input, function);
        }
    }
}

/// Run the benchmark - this function is called for each input
fn run_benchmark(n: u32, method: u32) {
    // Send the data to the prover
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);
    stdin.write(&method);

    // Initialize the prover client.
    let prover = ProverClient::from_env();

    let (_, r) = prover.execute(SHA_ELF, &stdin).run().unwrap();
    println!("Total cycles: {}", r.total_instruction_count());

    // Initialize the program for proving.
    let (pk, vk) = prover.setup(SHA_ELF);

    // Generate the proof
    let proof = prover
        .prove(&pk, &stdin)
        .run()
        .expect("failed to generate proof");

    println!("Proof size: {} bytes", size(&proof));

    // Verify the proof.
    prover.verify(&proof, &vk).expect("failed to verify proof");
    println!("Successfully verified proof!\n");
}
