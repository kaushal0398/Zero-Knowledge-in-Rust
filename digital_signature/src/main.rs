use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use sha2::{Sha256, Digest};
use rand::rngs::OsRng;

fn generate_rsa_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    let private_key = RsaPrivateKey::new(&mut OsRng, 2048).expect("Failed to generate key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}
fn sign_message(private_key: &RsaPrivateKey, message: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hashed_message = hasher.finalize();

    let padding = PaddingScheme::new_pkcs1v15_sign(Some(rsa::pkcs1v15::Hash::SHA2_256));

    private_key
        .sign(padding, &hashed_message)
        .expect("Failed to sign message")
}

fn verify_signature(public_key: &RsaPublicKey, message: &str, signature: &[u8]) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let hashed_message = hasher.finalize();

    let padding = PaddingScheme::new_pkcs1v15_sign(Some(rsa::pkcs1v15::Hash::SHA2_256));

    public_key
        .verify(padding, &hashed_message, signature)
        .is_ok()
}

fn main() {
    let (private_key, public_key) = generate_rsa_keypair();
    let message = "Hello, Rust Zero Knowledge!";
    println!("Original Message: {}", message);

    let signature = sign_message(&private_key, message);
    println!("Signature: {:?}", signature);

    let is_valid = verify_signature(&public_key, message, &signature);
    println!("Signature valid: {}", is_valid);
}
