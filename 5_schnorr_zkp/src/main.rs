use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use rand::rngs::OsRng;
use rand::RngCore;  // Import RngCore trait for generating random bytes

fn schnorr_prove(secret: Scalar) -> (RistrettoPoint, Scalar, Scalar) {
    let mut random_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut random_bytes);
    
    let commitment = random_nonce * RISTRETTO_BASEPOINT_POINT;
    
    let mut challenge_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut challenge_bytes);
    let challenge = Scalar::from_bytes_mod_order(challenge_bytes);

    let response = random_nonce + challenge * secret;
    
    (commitment, challenge, response)
}


fn schnorr_verify(public_key: RistrettoPoint, commitment: RistrettoPoint, challenge: Scalar, response: Scalar) -> bool {
    let lhs = response * RISTRETTO_BASEPOINT_POINT;
    
    let rhs = commitment + challenge * public_key;
    
    lhs == rhs
}

fn main() {
    
    
    println!("Prover's Secret (hidden): {:?}", secret);
    println!("Prover's Public Key: {:?}", public_key);
    
    let (commitment, challenge, response) = schnorr_prove(secret);
    
    println!("Commitment: {:?}", commitment);
    println!("Challenge: {:?}", challenge);
    println!("Response: {:?}", response);
    
    let is_valid = schnorr_verify(public_key, commitment, challenge, response);
    
}
