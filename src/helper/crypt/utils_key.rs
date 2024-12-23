use std::cmp::Ordering;

use hmac::{
    Hmac,
    Mac,
};
use sha2::{
    Digest,
    Sha512,
};

use super::utils_other::hash_concatenated;

/// Calculates an HMAC using SHA-512 over concatenated input buffers.
///
/// # Arguments
/// * `key` - The key used for HMAC calculation
/// * `buffers` - Slice of byte slices to be concatenated and hashed
///
/// # Returns
/// A vector containing the calculated HMAC bytes
pub(crate) fn hmac(key: &[u8], buffers: &[&[u8]]) -> Vec<u8> {
    let mut mac = Hmac::<Sha512>::new_from_slice(key).unwrap();
    for buffer in buffers {
        mac.update(buffer);
    }
    mac.finalize().into_bytes().to_vec()
}

/// Creates an IV by hashing the salt value and block key together.
/// The resulting hash is adjusted to match the specified block size
/// by either padding with zeros or truncating.
///
/// # Arguments
/// * `salt_value` - Salt value to use in hash
/// * `block_size` - Target size for the IV in bytes
/// * `block_key` - Block key to use in hash
///
/// # Returns
/// A vector containing the IV adjusted to the block size
pub(crate) fn create_iv(salt_value: &[u8], block_size: usize, block_key: &[u8]) -> Vec<u8> {
    // Hash the salt value and block key together
    let mut iv = hash_concatenated(&[salt_value, block_key]);

    // Adjust the IV length to match the block size
    match iv.len().cmp(&block_size) {
        Ordering::Less => {
            // Pad with zeros if IV is shorter than block size
            iv.resize(block_size, 0);
        }
        Ordering::Greater => {
            // Truncate if IV is longer than block size
            iv.truncate(block_size);
        }
        Ordering::Equal => {}
    }

    iv
}

/// Generates a cryptographic key from a password using SHA-512 hashing.
///
/// # Arguments
/// * `password` - Password string to convert
/// * `salt` - Salt bytes for key derivation
/// * `spin_count` - Number of iterations for key strengthening
/// * `key_bits` - Desired key length in bits
/// * `block_key` - Additional key material for final hash
///
/// # Returns
/// A vector containing the derived key bytes, truncated or padded to match
/// `key_bits` length
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn convert_password_to_key(
    password: &str,
    salt: &[u8],
    spin_count: usize,
    key_bits: usize,
    block_key: &[u8],
) -> Vec<u8> {
    // Convert password to UTF-16LE bytes
    let password_bytes: Vec<u8> = password.encode_utf16().flat_map(u16::to_le_bytes).collect();

    let mut hasher = Sha512::new();

    hasher.update(salt);
    hasher.update(&password_bytes);

    let mut key = hasher.finalize();

    // Iterate spin_count times
    for i in 0..spin_count {
        let i_bytes = (i as u32).to_le_bytes();
        let mut hasher = Sha512::new();
        hasher.update(i_bytes);
        hasher.update(key);
        key = hasher.finalize();
    }

    let mut hasher = Sha512::new();
    hasher.update(key);
    hasher.update(block_key);
    let mut key = hasher.finalize().to_vec();

    // Truncate or pad the key to the desired length
    let key_bytes = key_bits / 8;
    match key.len().cmp(&key_bytes) {
        Ordering::Less => {
            // Pad with zeros
            key.resize(key_bytes, 0);
            key
        }
        Ordering::Greater => key[..key_bytes].to_vec(),
        Ordering::Equal => key,
    }
}

/// Generates a cryptographic hash of a password using SHA-512.
///
/// # Arguments
/// * `password` - The password string to hash
/// * `salt` - The salt bytes to use in hashing
/// * `spin_count` - Number of iterations for the hash function
///
/// # Returns
/// A vector containing the final hash value
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn convert_password_to_hash(password: &str, salt: &[u8], spin_count: usize) -> Vec<u8> {
    // Convert password to UTF-16LE bytes
    let password_bytes: Vec<u8> = password.encode_utf16().flat_map(u16::to_le_bytes).collect();

    let mut hasher = Sha512::new();
    hasher.update(salt);
    hasher.update(password_bytes);

    // Iterate spin_count times
    for i in 0..spin_count {
        let i_bytes = (i as u32).to_le_bytes();
        hasher.update(i_bytes);
    }

    hasher.finalize().to_vec()
}
