use bellman::{Circuit, ConstraintSystem, SynthesisError};
use pairing::bn256::{Bn256, Fr};
use bellman::groth16::{generate_random_parameters, create_random_proof, verify_proof, prepare_verifying_key};
use rand::thread_rng;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
fn pedersen_commit(value: u64, blinding: Scalar) -> RistrettoPoint {
    let value_scalar = Scalar::from(value);
    value_scalar * RISTRETTO_BASEPOINT_POINT + blinding * RISTRETTO_BASEPOINT_POINT
}

struct PrivateTransfer {
    sender_balance_commitment: RistrettoPoint,  
    receiver_balance_commitment: RistrettoPoint, 
    transfer_amount_commitment: RistrettoPoint,  
    sender_blinding: Scalar,     
    receiver_blinding: Scalar,  
    transfer_blinding: Scalar,   
    transfer_amount: u64,        
}

