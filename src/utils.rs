use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns the current timestamp as an i64.
pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

/// Returns the SHA-256 hash of the input data.
pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
