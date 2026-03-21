// я думаю можно обойтись этой хуйней но если хочешь запариться или наоборот найдешь другой
// ассиметричный алгоритм шифрования то я не против
use ed25519_dalek::*;
use hex;

pub fn sign_data(private_key: &SigningKey, data: &[u8]) -> String {
    return "test".to_string();
}

pub fn verify_signature(public_key: &str, signature: &str, data: &[u8]) -> bool {
    return false;
}
