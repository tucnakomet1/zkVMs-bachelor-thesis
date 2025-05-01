//! EVM-Compatible proof generated which can be verified on-chain.
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release --bin evm -- --system groth16
//! RUST_LOG=info cargo run --release --bin evm -- --system plonk
//! ```

mod parse_args;
use alloy_sol_types::SolType;
use sha256_lib::PublicValuesStruct;
use serde::{Deserialize, Serialize};
use sp1_sdk::{
    include_elf, HashableKey, ProverClient, SP1ProofWithPublicValues, SP1Stdin, SP1VerifyingKey,
};
use std::path::PathBuf;
use utils::size;
use std::time::Instant;

/// The ELF file for the Succinct RISC-V zkVM.
pub const SHA_ELF: &[u8] = include_elf!("sha256-program");

/// A fixture that can be used to test the verification of SP1 zkVM proofs inside Solidity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SP1FibonacciProofFixture {
    hash: [u8; 32],
    n: u32,
    vkey: String,
    public_values: String,
    proof: String,
}

// Initialization of logger - tracing
fn init_logger() {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("NO_COLOR", "1");
    sp1_sdk::utils::setup_logger();
}

fn main() {
    // Setup the logger.
    init_logger();

    // Parse the command line arguments.
    let (num_of_rep, inputs, function) = parse_args::parse_args();
    
    for input in inputs {
        for i in 0..num_of_rep {
            println!("Run: {}; Input: {}", i, input);
            run_benchmark(input, function);
        }
    }
}


/// Run the benchmark - this function is called for each input
fn run_benchmark(n: u32, proof_system: u32) {
    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);
    stdin.write(&proof_system);

    println!("Proof System: {}", proof_system);

    // Setup the prover client and program.
    let prover = ProverClient::from_env();

    // Get the number of cycles
    let (_, report) = prover.execute(SHA_ELF, &stdin).run().unwrap();
    println!("Total cycles: {}", report.total_instruction_count());

    let (pk, vk) = prover.setup(SHA_ELF);

    let timer = Instant::now();
    // Generate the proof based on the selected proof system.
    let proof = match proof_system {
        3 => prover.prove(&pk, &stdin).plonk().run(),
        4 => prover.prove(&pk, &stdin).groth16().run(),
        _ => panic!("Unsupported proof system"),
    }.expect("failed to generate proof");

    let prover_time = timer.elapsed().as_millis();
    println!("Prover time: {:?}", prover_time);

    // Verify the proof.
    prover.verify(&proof, &vk).expect("failed to verify proof");
    println!("Verifier time: {:?}", timer.elapsed().as_millis() - prover_time);

    create_proof_fixture(&proof, &vk, proof_system);
}

/// Create a fixture for the given proof.
fn create_proof_fixture(proof: &SP1ProofWithPublicValues, vk: &SP1VerifyingKey, system: u32 ) {
    let mut proof_system = "Groth16";
    if system == 3 {
        proof_system = "Plonk";
    }

    // Deserialize the public values.
    let bytes = proof.public_values.as_slice();
    let PublicValuesStruct { n, hash } = PublicValuesStruct::abi_decode(bytes).unwrap();

    // Create the testing fixture so we can test things end-to-end.
    let fixture = SP1FibonacciProofFixture {
        hash: *hash,
        n,
        vkey: vk.bytes32().to_string(),
        public_values: format!("0x{}", hex::encode(bytes)),
        proof: format!("0x{}", hex::encode(proof.bytes())),
    };

    // The verification key is used to verify that the proof corresponds to the execution of the
    // program on the given input.
    println!("Verification Key: {}", fixture.vkey);

    // The public values are the values which are publicly committed to by the zkVM.
    println!("Public Values: {}", fixture.public_values);

    // We prove that the program was executed with some inputs that led to the give public values.
    println!("Proof Size={}", size(&fixture.proof));

    // Save the fixture to a file.
    let fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../contracts/src/fixtures");
    std::fs::create_dir_all(&fixture_path).expect("failed to create fixture path");
    std::fs::write(
        fixture_path.join(format!("{:?}-fixture.json", proof_system).to_lowercase()),
        serde_json::to_string_pretty(&fixture).unwrap(),
    )
    .expect("failed to write fixture");
}
