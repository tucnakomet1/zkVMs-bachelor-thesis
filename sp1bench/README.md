# Running SP1 benchmark


For example:
> RUST_LOG=info cargo run --release -- --n 50 --sha256

or 

> RUST_LOG=info cargo run --release --bin evm -- --n 50 --system groth16

CLI usage:

```bash
Usage: sha256 [OPTIONS]

Options:
      --n <N>            [default: 20]
      --fibonacci        
      --sha256           
      --poseidon         
      --system <SYSTEM>  [default: groth16] [possible values: plonk, groth16]
  -h, --help             Print help
```

You can run fibonacci/sha256/poseidon only in normal mode. 
By running EVM, sha256 is always forced. (other functions are not compatible)