use crate::consensus::DIFFICULTY;
use crate::types::BlockHeader;

// Проверяет, удовлетворяет ли хеш требуемой сложности
pub fn verify_pow(block_hash: &str) -> bool {
    // TODO: заглушка, убрать
    true
}

// Полная валидация PoW:
// 1) соответствие сложности
// 2) хэш, указанный в заголовке, соответствует заново вычисленному хешу
pub fn validate_pow(header: &BlockHeader, block_hash: &str) -> bool {
    // TODO: заглушка, убрать
    true
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

    // TODO: возможно сделать тест на happy path для validate_pow
}
