use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use rand::rngs::OsRng;
use rand::RngCore;  // Import RngCore trait for generating random bytes

fn pedersen_commit(value: u64, randomness: Scalar) -> RistrettoPoint {
    let value_scalar = Scalar::from(value);
    value_scalar * RISTRETTO_BASEPOINT_POINT + randomness * RISTRETTO_BASEPOINT_POINT
}

fn verify_commitment(commitment: RistrettoPoint, value: u64, randomness: Scalar) -> bool {
    let recomputed_commitment = pedersen_commit(value, randomness);
    commitment == recomputed_commitment
}

fn main() {
    let mut rng = OsRng;

    // Generate random bytes and convert them into a Scalar
    let mut random_bytes = [0u8; 32];
    rng.fill_bytes(&mut random_bytes);
    let randomness = Scalar::from_bytes_mod_order(random_bytes);

    let value = 42;
    println!("Value to commit: {}", value);
    
    // The Pedersen commitment.
    let commitment = pedersen_commit(value, randomness);
    println!("Pedersen Commitment: {:?}", commitment);
    
    // Verify.
    let is_valid = verify_commitment(commitment, value, randomness);
    println!("Commitment valid: {}", is_valid);
}
