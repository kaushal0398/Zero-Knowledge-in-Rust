use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;

pub fn pedersen_commit(value: u64, blinding: Scalar) -> RistrettoPoint {
    // Convert the value into a scalar.
    let value_scalar = Scalar::from(value);

    // Ensure the value is in the correct range (example: non-negative, bounded).
    if value > (1 << 63) {
        panic!("Value out of bounds");
    }

    // Return the Pedersen commitment.
    value_scalar * RISTRETTO_BASEPOINT_POINT + blinding * RISTRETTO_BASEPOINT_POINT
}


