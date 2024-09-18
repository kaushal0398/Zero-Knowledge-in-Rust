use crate::circuits::commitment::CommitmentCircuit;
use crate::zk_protocols::snark::create_snark_proof;
use pairing::bn256::Fr;
use rand::thread_rng;
pub fn private_transfer_example() -> bool {
    let circuit = CommitmentCircuit {
        value: Some(Fr::from_str("100").unwrap()),
        blinding: Some(Fr::from_str("42").unwrap()),
    };

    create_snark_proof(circuit)
}

fn main() {
    let is_valid = private_transfer_example();
    println!("Private transfer proof valid: {}", is_valid);
}
