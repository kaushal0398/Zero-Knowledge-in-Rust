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

fn decrypt_message(private_key: &RsaPrivateKey, encrypted_message: &[u8]) -> String {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    let decrypted_data = private_key
        .decrypt(padding, encrypted_message)
        .expect("Failed to decrypt");
    String::from_utf8(decrypted_data).expect("Invalid UTF-8 data")
}

fn main() {
    let (private_key, public_key) = generate_rsa_keypair();
    let message = "Hello, Rust Zero Knowledge!";
    println!("Original Message: {}", message);
    
    let encrypted_message = encrypt_message(&public_key, message);
    println!("Encrypted Message (in bytes): {:?}", encrypted_message);
    
    let decrypted_message = decrypt_message(&private_key, &encrypted_message);
    println!("Decrypted Message: {}", decrypted_message);
}
