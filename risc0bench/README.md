# Running RISC Zero benchmark


For example:
> cargo run --release -- --n 50 --sha256

CLI usage:

```bash
Usage: cargo run --release -- [OPTIONS]

Options:
      --n <N>      [default: 20]
      --fibonacci
      --sha256  
      --poseidon   
      --poseidon2  
      --snark      
  -h, --help       Print help
```

Flag `--snark` is optional. 
It converts STARK into SNARK (Groth16) proof. 
In this case it is necessary to have docker installed.