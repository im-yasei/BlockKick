use crate::crypto::verify_signature;
use crate::types::{Block, Transaction};

pub struct Validator;

impl Validator {
    pub fn validate_transaction(tx: &Transaction) -> bool {
        if !tx.requires_signature() {
            return tx.id == tx.calculate_id();
        }

        let expected_id = tx.calculate_id();

        if tx.id != expected_id {
            return false;
        }

        if let (Some(_from), Some(_signature)) = (&tx.from, &tx.signature) {
            return verify_signature(_from, _signature, &tx.get_signing_data().as_bytes());
        }

        true
    }

    pub fn validate_block(block: &Block) -> bool {
        block.validate()
    }

    pub fn validate_chain(prev_block: &Block, new_block: &Block) -> bool {
        new_block.header.prev_hash == prev_block.calculate_hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::sign_data;
    use crate::keys::KeyPair;
    use crate::types::transaction::*;
    use crate::types::Block;

    #[test]
    fn test_validate_transaction_id() {
        let key_pair = KeyPair::generate();

        let mut tx = Transaction::new(
            TransactionType::Transfer,
            Some(key_pair.public_key),
            Some("test_to".to_string()),
            TransactionData::Transfer(TransferData {
                amount: 100,
                message: "test".to_string(),
            }),
            1234567890,
        );

        let signing_data = tx.get_signing_data();
        tx.add_signature(sign_data(&key_pair.private_key, signing_data.as_bytes()));
        assert!(Validator::validate_transaction(&tx));
    }

    #[test]
    fn test_validate_coinbase_transaction() {
        let tx = Transaction::new(
            TransactionType::Coinbase,
            None,
            Some("miner_address".to_string()),
            TransactionData::Coinbase(CoinbaseData {
                reward: 50,
                block_height: 100,
            }),
            1234567890,
        );

        assert!(Validator::validate_transaction(&tx));
    }

    #[test]
    fn test_validate_block() {
        let transactions = vec![Transaction::new(
            TransactionType::Coinbase,
            None,
            Some("miner".to_string()),
            TransactionData::Coinbase(CoinbaseData {
                reward: 50,
                block_height: 0,
            }),
            1234567890,
        )];

        let block = Block::new(0, "0".repeat(64), transactions, 0);
        assert!(Validator::validate_block(&block));
    }

    #[test]
    fn test_validate_genesis_block() {
        let genesis = Block::genesis();
        assert!(Validator::validate_block(&genesis));
    }
}
