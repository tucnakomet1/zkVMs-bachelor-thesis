use risc0_core::field::{baby_bear::BabyBearElem};
use risc0_zkp::core::hash::poseidon2::Poseidon2HashSuite;

/// This function runs the poseidon2 hash function (RISC Zero implementation) n times
pub fn run_poseidon(n: u32) -> Vec<u32> {

    // use the poseidon hash function from the risc0 library
    let risc0_pos = Poseidon2HashSuite::new_suite();
    let input = [123u32; 8];    // 123 is just a random number

    // convert the input to BabyBearElem
    let risc0_input: Vec<BabyBearElem> = input.iter()
        .map(|x| BabyBearElem::from(*x))
        .collect();

    // hash the input n times
    let hash = risc0_pos.hashfn.hash_elem_slice(&risc0_input);
    for _ in 0..n-1 {
        risc0_pos.hashfn.hash_elem_slice(&risc0_input);
    }

    //println!("Hash: {:?}", hash.to_string());

    // convert the hash to a u32 array
    let bytes = hash.as_bytes();
    let parts: Vec<u32> = bytes.chunks(4)
        .map(|chunk| u32::from_le_bytes(chunk.try_into().unwrap()))
        .take(8).collect();

    // return the hash as a vector of u32
    parts
}
