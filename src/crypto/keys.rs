use ed25519_dalek::SigningKey;
use hex;
use rand::rngs::OsRng;

pub struct KeyPair {
    pub private_key: SigningKey,
    pub public_key: String,
}

impl KeyPair {
    pub fn generate() -> Self {
        let mut csprng = OsRng;
        let private_key = SigningKey::generate(&mut csprng);
        let public_key = private_key.verifying_key();
        Self {
            private_key,
            public_key: hex::encode(public_key.to_bytes()).to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_key_gen() {
        let keypair = KeyPair::generate();
        println!("test {}", keypair.public_key);
        assert!(!keypair.public_key.is_empty());
    }
}
