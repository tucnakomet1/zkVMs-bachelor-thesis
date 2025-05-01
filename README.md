# zkVMs benchmarks

This repository contain Fibonacci, SHA-256, Poseidon and STARK to SNARK benchmarks of [SP1](https://succinct.xyz/) and [RISC Zero](https://risczero.com/) zkVMs.

## Installation

It is assumed that rust is installed.

Here is a quick linux installation guide, but it is recommended to go to the official sites - [RISC Zero docs](https://dev.risczero.com/api/), [SP1 docs](https://docs.succinct.xyz/docs/sp1/introduction).

If you plan to generate SNARK proofs, you also need to have **docker** installed.

### RISC Zero installation

```bash
# make rzup available in CLI
curl -L https://risczero.com/install | bash

# download RISC Zero toolchain
rzup install
```


### SP1 installation

```bash
# make sp1up available in CLI
curl -L https://sp1up.succinct.xyz | bash

# download SP1 toolchain
sp1up
```

## Structure

This repository contains three folders [risc0bench](risc0bench), [sp1bench](sp1bench) and [graphs](graphs).

### Structure of risc0 folder

This folder contains following subfolders:
- [host](risc0bench/host/): host method, parse CLI, runs the prover and verifier.
- [method/guest](risc0bench/methods/guest/): guest method, contains Fibonacci, SHA-256 and Poseidon hash functions.

### Structure of SP1 folder

This folder contains following subfolders:
- [script](risc0bench/script/src/bin/): host method, parse CLI, runs the prover and verifier.
- [program](risc0bench/methods/guest/): guest method, contains Fibonacci, SHA-256 and Poseidon hash function.
- [lib](risc0bench/lib/src/): guest sha256 function, called by evm program
- [utils](sp1bench/utils/): method for calculating size of the proof
- [contracts](sp1bench/contracts/): evm folder, contains generated solidity code

## Usage

To run the benchmark, go to a specific folder first:
```bash
cd risc0bench 
# or
cd sp1bench/script
# or
cd graphs
```

### RISC Zero

Run the following command in the main RISC Zero directory (`/risc0bench`):

```bash
cargo run -- --n 10 --sha256 --snark  # > log.txt
```
- where `n` can be any integer (by default 20) and denotes the number of benchmark repetitions
- you can choose between flags `--fibonacci`, `--sha256`, `--poseidon`, `--poseidon2`
- flag `--snark` is optional - if used, the Groth16 proof is generated
- if you want to save logs, use also `> log.txt`

> *There are two Poseidon hash implementation, one using SP1 library, the second (poseidon2) using RISC Zero library.*

### SP1

Inside `sp1bench/script` run the following script:
```bash
# for normal STARK proof generation
RUST_LOG=info cargo run --release -- --n 10 --sha256 # > log.txt

# for a SNARK proof generation - choose between groth16 and plonk
RUST_LOG=info cargo run --release --bin evm -- --system groth16 --n 10 # > log.txt
RUST_LOG=info cargo run --release --bin evm -- --system plonk --n 10 # > log.txt
```
- where `n` is again any integer denoting the number of benchmark repetitions
- you can choose between flags `--fibonacci`, `--sha256`, `--poseidon`
- if you want to save logs, use also `> log.txt`
- if you want to generate SNARK:
    - you have to include `--bin evm` flag
    - you can choose between `--system groth16` and `--system plonk`

> You can run fibonacci/sha256/poseidon only in normal mode. By running EVM, sha256 is always forced - other functions are not compatible


### Graphs

This folder contains python codes for scraping the results of benchmarks.

- [split_to_parts.py](graphs/split_to_parts.py): split the log file into parts by an input
- [scraper.py](graphs/scraper.py): scrape the log file and save it into csv
- [complet_to_csv.py](graphs/complet_to_csv.py): merge all the csvs into one `complet.csv` file