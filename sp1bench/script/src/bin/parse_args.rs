use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ProofSystem {
    Plonk,
    Groth16,
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, default_value = "20")]
    n: u32,

    #[arg(long, default_value = "false")]
    fibonacci: bool,

    #[arg(long, default_value = "false")]
    sha256: bool,

    #[arg(long, default_value = "false")]
    poseidon: bool,

    #[arg(long, value_enum, default_value = "groth16")]
    system: ProofSystem,

}

/// Command line arguments parser Parse:
/// - n (number of runs) from command line arguments
/// - function - fibonacci/ sha256/ poseidon
/// - evm - bool value if we want to use snark or not
///   -- groth16/ plonk
/// You can run using: cargo run --release -- --n 20 --fibonacci --snark true
pub fn parse_args() -> (u32, Vec<u32>, u32) {
    let args = Args::parse();

    let n: u32 = args.n;
    let fibonacci: bool = args.fibonacci;
    let sha256: bool = args.sha256;
    let poseidon: bool = args.poseidon;

    let mut function: u32 = 1;
    let mut inputs: Vec<u32> = vec![
        5, 10, 50, 100, 200, 500, 1000
    ];

    if args.system == ProofSystem::Groth16 {
        function = 3;
    } else if args.system == ProofSystem::Plonk {
        function = 4;
    }
    else if fibonacci {
        function = 0;
        inputs = vec![
            5, 10, 100, 1000, 10000, 50000
        ];
    } else if sha256 {
        function = 1;
    } else if poseidon {
        function = 2;
    }

    (n, inputs, function)
}