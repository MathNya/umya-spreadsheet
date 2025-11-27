//! Cryptographic constants and parameters for Excel document encryption.
//!
//! This module defines the standard constants used in the Office Open XML (OOXML)
//! encryption process, including:
//!
//! - Encryption format version identifiers
//! - Block keys for various encryption stages
//! - Package encryption parameters
//! - Key derivation constants
//!
//! # Implementation Details
//!
//! - Uses AES-256 for encryption
//! - Implements CBC mode for block chaining
//! - SHA-512 for hashing operations
//! - 100,000 iterations for key derivation
//! - 4096-byte chunks for package encryption
//!
//! # Standards Compliance
//!
//! These constants comply with the OOXML standard for document encryption
//! and are compatible with Microsoft Office encryption implementations.
//!
//! # Note
//!
//! These constants are essential for maintaining compatibility with the
//! Office Open XML encryption standard and should not be modified unless
//! the standard changes.

/// Constants used in the encryption process
pub const ENCRYPTION_INFO_PREFIX: [u8; 8] = [0x04, 0x00, 0x04, 0x00, 0x40, 0x00, 0x00, 0x00]; // Version and reserved bytes

pub const PACKAGE_ENCRYPTION_CHUNK_SIZE: usize = 4096;
pub const PACKAGE_OFFSET: usize = 8; // First 8 bytes are the size of the stream

// Block keys used in various stages of encryption
pub const BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY: [u8; 8] =
    [0x5f, 0xb2, 0xad, 0x01, 0x0c, 0xb9, 0xe1, 0xf6];

pub const BLOCK_KEYS_DATA_INTEGRITY_HMAC_VALUE: [u8; 8] =
    [0xa0, 0x67, 0x7f, 0x02, 0xb2, 0x2c, 0x84, 0x33];

pub const BLOCK_KEYS_KEY: [u8; 8] = [0x14, 0x6e, 0x0b, 0xe7, 0xab, 0xac, 0xd0, 0xd6];

pub const BLOCK_VERIFIER_HASH_INPUT: [u8; 8] = [0xfe, 0xa7, 0xd2, 0x76, 0x3b, 0x4b, 0x9e, 0x79];

pub const BLOCK_VERIFIER_HASH_VALUE: [u8; 8] = [0xd7, 0xaa, 0x0f, 0x6d, 0x30, 0x61, 0x34, 0x4e];

// Package parameters
pub const PACKAGE_BLOCK_SIZE: usize = 16;
pub const PACKAGE_CIPHER_ALGORITHM: &str = "AES";
pub const PACKAGE_CIPHER_CHAINING: &str = "ChainingModeCBC";
pub const PACKAGE_HASH_ALGORITHM: &str = "SHA512";
pub const PACKAGE_HASH_SIZE: usize = 64;
pub const PACKAGE_KEY_BITS: usize = PACKAGE_KEY_LENGTH * 8;
pub const PACKAGE_KEY_LENGTH: usize = 32;

// Key parameters
pub const KEY_BITLENGTH: usize = 256;
pub const KEY_BLOCK_SIZE: usize = 16;
pub const KEY_CIPHER_ALGORITHM: &str = "AES";
pub const KEY_CIPHER_CHAINING: &str = "ChainingModeCBC";
pub const KEY_HASH_ALGORITHM: &str = "SHA-512";
pub const KEY_HASH_SIZE: usize = 64;
pub const KEY_SPIN_COUNT: usize = 100_000;

// Ths file is ignored by rustfmt.
