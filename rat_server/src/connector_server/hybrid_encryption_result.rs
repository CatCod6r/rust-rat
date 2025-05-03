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
