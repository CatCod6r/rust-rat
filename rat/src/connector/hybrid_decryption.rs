use std::str::from_utf8;

use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use screenshots::image::EncodableLayout;

pub struct HybridDecryption {
    encrypted_key: String,
    nonce: String,
    encrypted_data: String,
}
impl HybridDecryption {
    pub fn new(encrypted_key: String, nonce: String, encrypted_data: String) -> HybridDecryption {
        HybridDecryption {
            encrypted_key,
            nonce,
            encrypted_data,
        }
    }
    pub fn decrypt(&self, private_key: RsaPrivateKey) -> String {
        //Decrypt AES key
        let aes_key = private_key
            .decrypt(Pkcs1v15Encrypt, self.encrypted_key.as_bytes())
            .expect("key decryption failed");
        // Decrypt file
        let cipher = Aes256Gcm::new_from_slice(&aes_key).unwrap();
        let nonce = Nonce::from_slice(self.nonce.as_bytes());
        let decrypted_data = cipher.decrypt(nonce, self.encrypted_data.as_ref()).unwrap();

        String::from_utf8(decrypted_data).unwrap()
    }
}
