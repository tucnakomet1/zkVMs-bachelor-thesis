mod parse_args;

use methods::{SHA_ELF, SHA_ID};
use risc0_zkvm::{default_prover, ExecutorEnv, ProveInfo, ProverOpts, Receipt};
use std::time::Instant;

/// Initialization of logger - tracing
fn init_logger() {
    tracing_subscriber::fmt()
        .with_ansi(false)   // disable color output
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
}

/// Main method - initializes the logger, parses command line arguments, and runs the benchmark
fn main() {
    init_logger();

    // Parse command line arguments
    let (num_of_rep, inputs, snark, function) = parse_args::parse_args();

    // Run each input n times
    for input in inputs {
        for i in 0..num_of_rep {
            println!("Run: {}; Input: {}", i, input);
            run_benchmark(input, snark, function);
        }
    }
}

/// Run the benchmark - this function is called for each input
fn run_benchmark(n: u32, snark: bool, method: u32) {
    // Initialize the timer
    let timer = Instant::now();

    // Send the data to the prover (Guest program)
    let env = ExecutorEnv::builder()
        .write(&n).unwrap()
        .write(&method).unwrap()
        .build().unwrap();

    // Initialize the prover and run the prove function
    let prover = default_prover();
    let ProveInfo { receipt, stats, .. }: ProveInfo;

    if snark {
        ProveInfo { receipt, stats, .. } = prover.prove_with_opts(env, SHA_ELF, &ProverOpts::groth16()).unwrap();
    } else {
        ProveInfo { receipt, stats, .. } = prover.prove(env, SHA_ELF).unwrap();
    }

    // Get the receipt
    // contains Journal (public data) and Seal (proof)
    let receipt: Receipt = receipt;

    // Calculate the prover time
    let prover_time: u128 = timer.elapsed().as_millis();

    // Run the verifier
    receipt.verify(SHA_ID).unwrap();

    // Print the results
    println!("Prover time: {:?} ms", prover_time);
    println!("Verifier time: {:?} ms", timer.elapsed().as_millis() - prover_time);
    
    let receipt_size = bincode::serialize(&receipt).unwrap().len();
    println!("Receipt size: {} bytes", receipt_size);
    println!("Proof size: {} bytes", receipt.seal_size());
    println!("Journal size: {} bytes", receipt_size - receipt.seal_size());
    println!("Total Cycles: {:?}", stats.total_cycles);
    println!("Paging Cycles: {:?}", stats.paging_cycles);
    println!("User Cycles: {:?}", stats.user_cycles);
    println!("Reserved Cycles: {:?}\n", stats.reserved_cycles);
}