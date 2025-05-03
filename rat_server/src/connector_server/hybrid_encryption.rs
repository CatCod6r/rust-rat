use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use rand::{rngs::OsRng, RngCore};
use rsa::{
    pkcs1::{DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey},
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey,
};

pub fn encrypt_data_combined(
    public_key: RsaPublicKey,
    data_to_enc: Vec<u8>,
) -> HybridEncryptionResult {
    //Aes key generation
    let aes_key = Aes256Gcm::generate_key(&mut OsRng);

    // Encrypt file with AES
    let cipher = Aes256Gcm::new(&aes_key);
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let cyphertext = cipher.encrypt(nonce, data_to_enc.as_ref()).unwrap();

    let encrypted_key = encrypt_data(public_key, &aes_key);
    let result = HybridEncryptionResult::new(
        String::from_utf8(encrypted_key).unwrap(),
        String::from_utf8(nonce.as_slice().to_vec()).unwrap(),
        String::from_utf8(cyphertext).unwrap(),
    );
    result
}
pub fn encrypt_data(public_key: RsaPublicKey, data_to_enc: &[u8]) -> Vec<u8> {
    //rng for RSA
    let mut rng = rand::thread_rng();

    public_key
        .clone()
        .encrypt(&mut rng, Pkcs1v15Encrypt, data_to_enc)
        .expect("failed to encrypt")
}

pub fn generate_private_key() -> RsaPrivateKey {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key")
}
pub fn decrypt_data(private_key: RsaPrivateKey, data_to_decrypt: &[u8]) -> String {
    let decrypted_data = private_key
        .decrypt(Pkcs1v15Encrypt, data_to_decrypt)
        .unwrap();
    String::from_utf8(decrypted_data).unwrap()
}
pub struct HybridEncryptionResult {
    encrypted_key: String,
    nonce: String,
    encrypted_data: String,
}
impl HybridEncryptionResult {
    pub fn new(
        encrypted_key: String,
        nonce: String,
        encrypted_data: String,
    ) -> HybridEncryptionResult {
        HybridEncryptionResult {
            encrypted_key,
            nonce,
            encrypted_data,
        }
    }
}
