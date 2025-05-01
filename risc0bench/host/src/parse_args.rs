use clap::Parser;

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

    #[arg(long, default_value = "false")]
    poseidon2: bool,

    #[arg(long, default_value = "false")]
    snark: bool,

}

/// Command line arguments parser Parse:
/// - n (number of runs) from command line arguments
/// - function - fibonacci/ sha256/ poseidon/ poseidon using risc0
/// - snark - bool value if we want to use snark or not
/// You can run using: cargo run --release -- --n 20 --fibonacci --snark true
pub fn parse_args() -> (u32, Vec<u32>, bool, u32) {
    let args = Args::parse();

    let n: u32 = args.n;
    let fibonacci: bool = args.fibonacci;
    let sha256: bool = args.sha256;
    let poseidon: bool = args.poseidon;
    let poseidon_risc0: bool = args.poseidon2;
    let snark: bool = args.snark;

    let mut function: u32 = 1;
    let mut inputs: Vec<u32> = vec![
        5, 10, 50, 100, 200, 500, 1000
    ];

    if fibonacci {
        function = 0;
        inputs = vec![
            5, 10, 100, 1000, 10000, 50000
        ];
    } else if sha256 {
        function = 1;
    } else if poseidon {
        function = 2;
    } else if poseidon_risc0 {
        function = 3;
    }

    (n, inputs, snark, function)
}
