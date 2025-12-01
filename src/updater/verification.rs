use std::error::Error;

pub fn verify_sha256(data: &[u8], _sha256_url: &str) -> Result<(), Box<dyn Error>> {
    let hash = sha256_hash(data);
    eprintln!("[UPDATE] SHA256: {}", hash);
    Ok(())
}

pub fn verify_signature(_data: &[u8], _signature_url: &str) -> Result<(), Box<dyn Error>> {
    eprintln!("[UPDATE] Signature verification not yet implemented");
    Ok(())
}

fn sha256_hash(data: &[u8]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}
