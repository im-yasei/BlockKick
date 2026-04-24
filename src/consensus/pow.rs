use crate::consensus::DIFFICULTY;
use crate::types::BlockHeader;

pub fn verify_pow(block_hash: &str) -> bool {
    if block_hash.is_empty() {
        return false;
    }
    let prefix = "0".repeat(DIFFICULTY as usize);
    block_hash.starts_with(&prefix)
}

pub fn validate_pow(header: &BlockHeader, block_hash: &str) -> bool {
    let computed_hash = header.calculate_hash();
    computed_hash == block_hash && verify_pow(block_hash)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_pow_valid() {
        let hash = "0000abc123def456";
        assert!(verify_pow(hash));
    }

    #[test]
    fn test_verify_pow_invalid() {
        let hash = "000abc123def456";
        assert!(!verify_pow(hash));
    }

    #[test]
    fn test_verify_pow_empty_hash() {
        let hash = "";
        assert!(!verify_pow(hash));
    }

    #[test]
    fn test_validate_pow_wrong_hash() {
        let header = BlockHeader {
            index: 1,
            timestamp: 1234567890,
            prev_hash: "0".repeat(64),
            merkle_root: "abc123".repeat(10),
            nonce: 0,
        };
        let wrong_hash = "wrong_hash";
        assert!(!validate_pow(&header, wrong_hash));
    }
}
