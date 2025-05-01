use p3_baby_bear::BabyBear;
use p3_field::AbstractField;
use sp1_primitives::poseidon2_hash;

/// This function runs the poseidon2 hash function (SP1 implementation) n times on a given input
pub fn run_poseidon(n: u32) -> [BabyBear; 8] {
    let input = [BabyBear::from_canonical_u32(123); 8];      // 123 is just a random number

    let mut hash = input;

    // hash the input n times
    for _ in 0..n {
        hash = poseidon2_hash(hash.to_vec());
    }

    // return the hash as an array of BabyBear elements
    hash
}