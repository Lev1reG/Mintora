use hex;
use rand::Rng;
use sha2::{Digest, Sha256};
use crate::error::AppError;

const API_KEY_LENGTH: usize = 32; // 32 bytes = 64 hex characters
const PREFIX_LIVE: &str = "mnt_live_";
const PREFIX_TEST: &str = "mnt_test_";

pub struct GeneratedApiKey {
    pub key: String,          // Full key shown once to user
    pub key_prefix: String,   // First 8-16 chars for identification
    pub key_hash: String,     // SHA-256 hash stored in DB
}

/// Generate a new API key
pub fn generate_api_key(is_live: bool) -> Result<GeneratedApiKey, AppError> {
    let prefix = if is_live { PREFIX_LIVE } else { PREFIX_TEST };

    // Generate random bytes
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..API_KEY_LENGTH).map(|_| rng.gen()).collect();
    let random_part = hex::encode(random_bytes);

    // Construct full key: prefix + random_part
    let full_key = format!("{}{}", prefix, random_part);

    // Hash the full key for storage
    let key_hash = hash_api_key(&full_key);

    // Extract prefix for display (first 16 chars)
    let key_prefix = full_key.chars().take(16).collect();

    Ok(GeneratedApiKey {
        key: full_key,
        key_prefix,
        key_hash,
    })
}

/// Hash an API key using SHA-256
pub fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    hex::encode(hasher.finalize())
}

/// Verify an API key against its hash
pub fn verify_api_key(key: &str, hash: &str) -> bool {
    let computed_hash = hash_api_key(key);
    computed_hash == hash
}

/// Validate API key format
pub fn validate_api_key_format(key: &str) -> bool {
    (key.starts_with(PREFIX_LIVE) || key.starts_with(PREFIX_TEST)) && key.len() >= 40
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_key_generation() {
        let generated = generate_api_key(true).unwrap();

        assert!(generated.key.starts_with("mnt_live_"));
        assert!(verify_api_key(&generated.key, &generated.key_hash));
        assert!(!verify_api_key("wrong_key", &generated.key_hash));
    }

    #[test]
    fn test_api_key_format_validation() {
        assert!(validate_api_key_format(
            "mnt_live_1234567890abcdef1234567890abcdef"
        ));
        assert!(validate_api_key_format(
            "mnt_test_1234567890abcdef1234567890abcdef"
        ));
        assert!(!validate_api_key_format("invalid_key"));
        assert!(!validate_api_key_format("mnt_live_short"));
    }
}
