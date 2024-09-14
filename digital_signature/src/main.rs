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

