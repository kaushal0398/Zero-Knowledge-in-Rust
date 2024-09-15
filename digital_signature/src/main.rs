use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs1v15::VerificationKey;
use rsa::signature::{Signer, Verifier};
use sha2::{Sha256, Digest};
use rand::rngs::OsRng;

fn generate_rsa_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    // Generate a 2048-bit RSA private key
    let private_key = RsaPrivateKey::new(&mut OsRng, 2048).expect("Failed to generate key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

fn sign_message(private_key: &RsaPrivateKey, message: &str) -> Vec<u8> {
    // Hash the message using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hashed_message = hasher.finalize();

    // Use PKCS#1 v1.5 padding for signing
    let signing_key = SigningKey::<Sha256>::new(private_key);

    // Sign the hashed message with the private key
    signing_key.sign(&hashed_message).to_vec()
}

fn verify_signature(public_key: &RsaPublicKey, message: &str, signature: &[u8]) -> bool {
    // Hash the message using SHA-256
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hashed_message = hasher.finalize();

    // Use PKCS#1 v1.5 padding for verifying
    let verifying_key = VerificationKey::<Sha256>::new(public_key);

    // Verify the signature with the public key
    verifying_key.verify(&hashed_message, signature).is_ok()
}

fn main() {
    // Generate RSA keypair
    let (private_key, public_key) = generate_rsa_keypair();
    let message = "Hello, Rust Zero Knowledge!";
    println!("Original Message: {}", message);

    // Sign the message with the private key
    let signature = sign_message(&private_key, message);
    println!("Signature: {:?}", signature);

    // Verify the signature with the public key
    let is_valid = verify_signature(&public_key, message, &signature);
    println!("Signature valid: {}", is_valid);
}
