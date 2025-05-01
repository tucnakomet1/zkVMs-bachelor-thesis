/// Compute the n-th Fibonacci number in a RISC Zero guest program.
pub fn run_fibonacci(n: u32) -> u32 {
    // calculate the n-th Fibonacci number
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }

    // return the n-th Fibonacci number
    b
}