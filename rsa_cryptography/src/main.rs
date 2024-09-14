use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;

fn generate_rsa_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    let private_key = RsaPrivateKey::new(&mut OsRng, 2048).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}
use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;

fn generate_rsa_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    let private_key = RsaPrivateKey::new(&mut OsRng, 2048).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

fn encrypt_message(public_key: &RsaPublicKey, message: &str) -> Vec<u8> {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    public_key
        .encrypt(&mut OsRng, padding, message.as_bytes())
        .expect("Failed to encrypt")
}


