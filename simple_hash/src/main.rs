use sha2::{Sha256, Digest};

fn hash_message(message: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn main() {
    let message = "Zero Knowledge!";
    let hash = hash_message(message);
    println!("Message: {}", message);
    println!("SHA-256 Hash: {}", hash);
}
