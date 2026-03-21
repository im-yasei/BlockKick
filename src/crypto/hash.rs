use hex;
use sha2::{Digest, Sha256};

pub fn hash_data(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

pub fn hash_string(data: &str) -> String {
    hash_data(data.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_deterministic() {
        let hash1 = hash_string("test");
        let hash2 = hash_string("test");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_different() {
        let hash1 = hash_string("test");
        let hash2 = hash_string("another test");
        assert_ne!(hash1, hash2);
    }
}
