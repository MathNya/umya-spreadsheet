//! Cryptographic utilities for Excel workbook and worksheet protection.
//!
//! This module provides functionality for encrypting and managing protection
//! settings in Excel workbooks and worksheets. It includes methods for:
//! - Password-based encryption
//! - Hash generation
//! - Salt management
//! - Protection settings configuration
//!
//! The encryption process follows Microsoft's Office Open XML standards for
//! document protection, using standardized algorithms and key derivation
//! functions.
//!
//! # Examples
//!
//! ```
//! use crate::{
//!     helper::crypt,
//!     structs::SheetProtection,
//! };
//!
//! let mut protection = SheetProtection::new();
//! crypt::encrypt_sheet_protection("mypassword", &mut protection);
//! ```
//!
//! # Security
//!
//! The module implements industry-standard cryptographic practices:
//! - Random salt generation for each encryption
//! - Configurable key spin count for key derivation
//! - Base64 encoding for binary data storage
//! - Secure password hashing

use std::{io::Write, path::Path};

use rand::Rng;

use crate::structs::{SheetProtection, WorkbookProtection};

pub(crate) mod algo;
pub(crate) mod constants;
pub(crate) mod key;
pub(crate) mod utils;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use utils::generate_random_bytes;

/// Encrypts the sheet protection using the provided password.
///
/// Updates the `sheet_protection` object with the algorithm name, salt value,
/// spin count, and hash value.
///
/// # Parameters
///
/// - `password`: The password to use for encryption.
/// - `sheet_protection`: The `SheetProtection` object to update with encryption
///   details.
#[allow(clippy::cast_possible_truncation)]
pub fn encrypt_sheet_protection(password: &str, sheet_protection: &mut SheetProtection) {
    generate_random_bytes!(salt, 16);

    // Convert the password into a hash
    let key = key::convert_password_to_hash(password, &salt, constants::KEY_SPIN_COUNT);

    // Encode the salt and hash value in base64
    let salt_value_str = STANDARD.encode(salt);
    let hash_value_str = STANDARD.encode(&key);

    // Update the sheet_protection object
    sheet_protection.set_algorithm_name(constants::KEY_HASH_ALGORITHM);
    sheet_protection.set_salt_value(salt_value_str);
    sheet_protection.set_spin_count(constants::KEY_SPIN_COUNT as u32);
    sheet_protection.set_hash_value(hash_value_str);
    sheet_protection.remove_password_raw();
}

/// Encrypts the workbook protection using the provided password.
///
/// Updates the `workbook_protection` object with the algorithm name, salt
/// value, spin count, and hash value.
///
/// # Parameters
///
/// - `password`: The password to use for encryption.
/// - `workbook_protection`: The `WorkbookProtection` object to update with
///   encryption details.
#[allow(clippy::cast_possible_truncation)]
pub fn encrypt_workbook_protection(password: &str, workbook_protection: &mut WorkbookProtection) {
    generate_random_bytes!(salt, 16);

    // Convert the password into a hash
    let key = key::convert_password_to_hash(password, &salt, constants::KEY_SPIN_COUNT);

    // Encode the salt and hash value in base64
    let salt_value_str = STANDARD.encode(salt);
    let hash_value_str = STANDARD.encode(&key);

    // Update the workbook_protection object
    workbook_protection.set_workbook_algorithm_name(constants::KEY_HASH_ALGORITHM);
    workbook_protection.set_workbook_salt_value(salt_value_str);
    workbook_protection.set_workbook_spin_count(constants::KEY_SPIN_COUNT as u32);
    workbook_protection.set_workbook_hash_value(hash_value_str);
    workbook_protection.remove_workbook_password_raw();
}

/// Encrypts the revisions protection using the provided password.
///
/// This function generates a random salt, derives a key from the password using
/// SHA-512, and updates the workbook protection settings with the encrypted
/// values.
///
/// # Arguments
///
/// * `password` - The password string to use for encryption
/// * `workbook_protection` - Mutable reference to the `WorkbookProtection`
///   object to update
#[allow(clippy::cast_possible_truncation)]
pub fn encrypt_revisions_protection(password: &str, workbook_protection: &mut WorkbookProtection) {
    generate_random_bytes!(salt, 16);

    // Convert the password into a hash
    let key = key::convert_password_to_hash(password, &salt, constants::KEY_SPIN_COUNT);

    // Encode the salt and hash value in base64
    let salt_value_str = STANDARD.encode(salt);
    let hash_value_str = STANDARD.encode(&key);

    // Update the workbook_protection object
    workbook_protection.set_revisions_algorithm_name(constants::KEY_HASH_ALGORITHM);
    workbook_protection.set_revisions_salt_value(salt_value_str);
    workbook_protection.set_revisions_spin_count(constants::KEY_SPIN_COUNT as u32);
    workbook_protection.set_revisions_hash_value(hash_value_str);
    workbook_protection.remove_revisions_password_raw();
}

/// Encrypts the given data and writes it to a specified file.
///
/// This function performs the following steps:
/// 1. Generates random bytes for various cryptographic keys and salts.
/// 2. Encrypts the data using a specified package cipher algorithm (AES).
/// 3. Generates and encrypts an HMAC key and its corresponding value for data
///    integrity.
/// 4. Converts the provided password into a key using a key derivation
///    function.
/// 5. Encrypts the derived key and generates a verifier hash input and value,
///    both of which are also encrypted.
/// 6. Constructs an XML structure containing encryption information.
/// 7. Creates a compound file and writes the encrypted data and metadata to it.
///
/// # Parameters
///
/// - `filepath`: A reference to the path where the encrypted data will be
///   saved. This can be any type that implements the `AsRef<Path>` trait.
/// - `data`: A byte slice containing the data to be encrypted.
/// - `password`: A string slice representing the password used for key
///   derivation.
///
/// # Errors
///
/// This function may return errors related to file I/O or cryptographic
/// operations. Ensure to handle these errors appropriately in your application.
///
/// # Panics
///
/// This function may panic if:
/// - The underlying file operations fail (e.g., if the file cannot be created
///   or written to).
/// - Any cryptographic operation fails, such as key generation or encryption,
///   due to invalid parameters or internal errors.
///
/// # Example
///
/// ```
/// let data = b"Sensitive data to encrypt";
/// let password = "securepassword";
/// let filepath = "encrypted_data.bin";
///
/// encrypt(&filepath, data, password);
/// ```
///
/// # Note
///
/// The encryption process involves multiple cryptographic operations, including
/// key generation, HMAC creation, and data encryption. Ensure that the password
/// used is strong and kept secure.
pub fn encrypt<P: AsRef<Path>>(filepath: &P, data: &[u8], password: &str) {
    generate_random_bytes!(hmac_key, 64);
    generate_random_bytes!(key_salt, 16);
    generate_random_bytes!(package_key, 32);
    generate_random_bytes!(package_salt, 16);
    generate_random_bytes!(verifier_hash_input, 16);

    // Encrypt the package
    let encrypted_package = algo::crypt_package(
        true,
        constants::PACKAGE_BLOCK_SIZE,
        &package_salt,
        &package_key,
        data,
    );

    // Generate HMAC key and encrypt it
    let hmac_key_iv = key::create_iv(
        &package_salt,
        constants::PACKAGE_BLOCK_SIZE,
        &constants::BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY,
    );
    let encrypted_hmac_key = algo::crypt(true, &package_key, &hmac_key_iv, &hmac_key).unwrap();

    // Generate HMAC value and encrypt it
    let hmac_value = key::hmac(&hmac_key, &[&encrypted_package]);
    let hmac_value_iv = key::create_iv(
        &package_salt,
        constants::PACKAGE_BLOCK_SIZE,
        &constants::BLOCK_KEYS_DATA_INTEGRITY_HMAC_VALUE,
    );
    let encrypted_hmac_value =
        algo::crypt(true, &package_key, &hmac_value_iv, &hmac_value).unwrap();

    // Convert the password to a key
    let key = key::convert_password_to_key(
        password,
        &key_salt,
        constants::KEY_SPIN_COUNT,
        constants::KEY_BITLENGTH,
        &constants::BLOCK_KEYS_KEY,
    );
    let encrypted_key_value = algo::crypt(true, &key, &key_salt, &package_key).unwrap();

    // Generate verifier hash input and encrypt it
    let verifier_hash_input_key = key::convert_password_to_key(
        password,
        &key_salt,
        constants::KEY_SPIN_COUNT,
        constants::KEY_BITLENGTH,
        &constants::BLOCK_VERIFIER_HASH_INPUT,
    );
    let encrypted_verifier_hash_input = algo::crypt(
        true,
        &verifier_hash_input_key,
        &key_salt,
        &verifier_hash_input,
    )
    .unwrap();

    // Generate verifier hash value and encrypt it
    let verifier_hash_value = utils::hash_concatenated(&[&verifier_hash_input]);
    let verifier_hash_value_key = key::convert_password_to_key(
        password,
        &key_salt,
        constants::KEY_SPIN_COUNT,
        constants::KEY_BITLENGTH,
        &constants::BLOCK_VERIFIER_HASH_VALUE,
    );
    let encrypted_verifier_hash_value = algo::crypt(
        true,
        &verifier_hash_value_key,
        &key_salt,
        &verifier_hash_value,
    )
    .unwrap();

    // Build the encryption info XML data
    let encryption_info_buffer = algo::build_encryption_info(
        &package_salt,
        &encrypted_hmac_key,
        &encrypted_hmac_value,
        &key_salt,
        &encrypted_verifier_hash_input,
        &encrypted_verifier_hash_value,
        &encrypted_key_value,
    );

    // Create compound file and write streams
    let mut comp = cfb::create(filepath).unwrap();
    {
        let mut stream_info = comp.create_stream("EncryptionInfo").unwrap();
        stream_info.write_all(&encryption_info_buffer).unwrap();
    }
    {
        let mut stream_package = comp.create_stream("EncryptedPackage").unwrap();
        stream_package.write_all(&encrypted_package).unwrap();
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use std::fs;

    use hex_literal::hex;
    use sha2::{Digest, Sha256};

    use super::*;
    use crate::helper::utils::{assert_sha256, print_hex, print_sha256_hex};

    #[test]
    fn test_encrypt_package() {
        let data = fs::read("./tests/test_files/aaa.xlsx").unwrap();

        // Package parameters
        let package_key = hex!("cdf9defae2480933c503350e16334453d1cb8348bb2fea585db7f9e1f78fe9bf");
        let package_salt = hex!("4c251b321d85cecfcb6d952ba6d81846");

        // Encrypted package
        let encrypted_package = algo::crypt_package(
            true,
            constants::PACKAGE_BLOCK_SIZE,
            &package_salt,
            &package_key,
            &data,
        );

        assert_sha256!(
            &encrypted_package,
            "c2f7aa6ef36f5389aee63255887e103d7e9d9388c6dcaaa821fb62119ebdd697"
        );
    }

    #[test]
    fn test_encrypt_hmac_key() {
        // Package parameters
        let package_key = hex!("cdf9defae2480933c503350e16334453d1cb8348bb2fea585db7f9e1f78fe9bf");
        let package_salt = hex!("4c251b321d85cecfcb6d952ba6d81846");

        // HMAC key
        let hmac_key = hex!(
            "4c6e4db6d9a60e5d41c3ca639a682aaa71da7437202fe92ec5d814bd1e9e4e6a"
            "831aee889eae3bc18bc1bebedae1f73393fddfffd0a0b6c557485fefcdb5e98b"
        );

        let hmac_key_iv = key::create_iv(
            &package_salt,
            constants::PACKAGE_BLOCK_SIZE,
            &constants::BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY,
        );
        assert_eq!(hmac_key_iv, hex!("ba1bf00eed82b07ee65e574eb1f46043"));

        let encrypted_hmac_key = algo::crypt(true, &package_key, &hmac_key_iv, &hmac_key).unwrap();
        assert_eq!(
            encrypted_hmac_key,
            hex!(
                "b32b1cdc4ac1af244377c1eb57efd31a819f555a7204adcc0cfe364b394bbdb0
                 86a8daef4f4c512d52e3db6a54b1d45e1dd1dbfa3ddacc29fe35449ba5225dc7"
            )
        );
    }

    #[test]
    fn test_encrypt_hmac_value() {
        let hmac_key = hex!(
            "4c6e4db6d9a60e5d41c3ca639a682aaa71da7437202fe92ec5d814bd1e9e4e6a"
            "831aee889eae3bc18bc1bebedae1f73393fddfffd0a0b6c557485fefcdb5e98b"
        );

        // Use the data specific from "test_encrypt"
        let data = fs::read("./tests/test_files/aaa.xlsx").unwrap();

        let encrypted_package = algo::crypt_package(
            true,
            constants::PACKAGE_BLOCK_SIZE,
            &hex!("4c251b321d85cecfcb6d952ba6d81846"), // package_salt
            &hex!("cdf9defae2480933c503350e16334453d1cb8348bb2fea585db7f9e1f78fe9bf"), /* package_key */
            &data,
        );

        let hmac_value = key::hmac(&hmac_key, &[&encrypted_package]);
        assert_eq!(
            hmac_value,
            hex!(
                "48ed7b8718e55f7f6e19e1cae6a447afacff5e5e3fe92ac836908e6cdeeb68a2"
                "5fa1cbaaa4d7f7c0acabc2f7f22ad87bf11eaf74169fb6e6f78ab033e038928c"
            )
        );

        let hmac_value_iv = key::create_iv(
            &hex!("4c251b321d85cecfcb6d952ba6d81846"),
            constants::PACKAGE_BLOCK_SIZE,
            &constants::BLOCK_KEYS_DATA_INTEGRITY_HMAC_VALUE,
        );
        assert_eq!(hmac_value_iv, hex!("088385b871292e7ed8414f173c5b6622"));

        let encrypted_hmac_value = algo::crypt(
            true,
            &hex!("cdf9defae2480933c503350e16334453d1cb8348bb2fea585db7f9e1f78fe9bf"),
            &hmac_value_iv,
            &hmac_value,
        )
        .unwrap();
        assert_eq!(
            encrypted_hmac_value,
            hex!(
                "f75c7f3c44fadf9b4bbf2ff693586710c52e043d8db69e3e538be5f10d36f86d"
                "24dd5f4b9f71a2ce928abbbfe46e791a6c683703bcb30d5214997e60bbd547f6"
            )
        );
    }

    #[test]
    fn test_convert_password_to_key() {
        let key_salt = hex!("3aa973eec73c98c4710021730ef5b513");
        let password = "password";

        let key = key::convert_password_to_key(
            password,
            &key_salt,
            constants::KEY_SPIN_COUNT,
            constants::KEY_BITLENGTH,
            &constants::BLOCK_KEYS_KEY,
        );
        assert_eq!(
            key,
            hex!("8d5869311b1c1fdb59a1de6fe1e6f2ce7dccd4deb198a6dfb1f7fb55bc03487d")
        );

        let encrypted_key_value = algo::crypt(
            true,
            &key,
            &key_salt,
            &hex!("cdf9defae2480933c503350e16334453d1cb8348bb2fea585db7f9e1f78fe9bf"),
        )
        .unwrap();
        assert_eq!(
            encrypted_key_value,
            hex!("5017ddc6146e56dfbf76734b3e99b80f36a4c9a2e9eb21fe77695f73850cc452")
        );
    }

    #[test]
    fn test_encrypt_verifier_hash() {
        let key_salt = hex!("3aa973eec73c98c4710021730ef5b513");
        let password = "password";

        // Verifier hash input
        let verifier_hash_input = hex!("8f54777cba87efa55ea2db8399873815");
        let verifier_hash_input_key = key::convert_password_to_key(
            password,
            &key_salt,
            constants::KEY_SPIN_COUNT,
            constants::KEY_BITLENGTH,
            &constants::BLOCK_VERIFIER_HASH_INPUT,
        );
        assert_eq!(
            verifier_hash_input_key,
            hex!("44e4b664c512b08e7577aa3fc7e11ad603e0877a476931fad5aa79e203304aff")
        );

        let encrypted_verifier_hash_input = algo::crypt(
            true,
            &verifier_hash_input_key,
            &key_salt,
            &verifier_hash_input,
        )
        .unwrap();
        assert_eq!(
            encrypted_verifier_hash_input,
            hex!("2fb9eea58e227ffa549449e941f1199e")
        );

        // Verifier hash value
        let verifier_hash_value = utils::hash_concatenated(&[&verifier_hash_input]);
        assert_eq!(
            verifier_hash_value,
            hex!(
                "920b1de74f38d9cb3ccb3394119ed37e958404fdc47560b1bf647d3c49c22549"
                "625fe4a0bd36798bd68a0d98ae64f6ab64a330c9890c62bb740aa492c226ae1f"
            )
        );

        let verifier_hash_value_key = key::convert_password_to_key(
            password,
            &key_salt,
            constants::KEY_SPIN_COUNT,
            constants::KEY_BITLENGTH,
            &constants::BLOCK_VERIFIER_HASH_VALUE,
        );
        assert_eq!(
            verifier_hash_value_key,
            hex!("d5515a6062e3e99551b80b92db1fe646483884cdb63e1e7595a9f2cca7532884")
        );

        let encrypted_verifier_hash_value = algo::crypt(
            true,
            &verifier_hash_value_key,
            &key_salt,
            &verifier_hash_value,
        )
        .unwrap();
        assert_eq!(
            encrypted_verifier_hash_value,
            hex!(
                "0d9c888111b40b630b739c95a5f5b6be67c8f96acdd1bee185bd808b507f6527"
                "60a2e77f63a6ad0c46f985f2bb8dab4fcf9b86d6a40d9c21299bb4ddf788b250"
            )
        );
    }

    #[test]
    fn test_build_encryption_info() {
        let encryption_info = algo::build_encryption_info(
            &hex!("4c251b321d85cecfcb6d952ba6d81846"),
            &hex!(
                "b32b1cdc4ac1af244377c1eb57efd31a819f555a7204adcc0cfe364b394bbdb0"
                "86a8daef4f4c512d52e3db6a54b1d45e1dd1dbfa3ddacc29fe35449ba5225dc7"
            ),
            &hex!(
                "f75c7f3c44fadf9b4bbf2ff693586710c52e043d8db69e3e538be5f10d36f86d"
                "24dd5f4b9f71a2ce928abbbfe46e791a6c683703bcb30d5214997e60bbd547f6"
            ),
            &hex!("3aa973eec73c98c4710021730ef5b513"),
            &hex!("2fb9eea58e227ffa549449e941f1199e"),
            &hex!(
                "0d9c888111b40b630b739c95a5f5b6be67c8f96acdd1bee185bd808b507f6527
                 60a2e77f63a6ad0c46f985f2bb8dab4fcf9b86d6a40d9c21299bb4ddf788b250"
            ),
            &hex!("5017ddc6146e56dfbf76734b3e99b80f36a4c9a2e9eb21fe77695f73850cc452"),
        );

        assert_sha256!(
            encryption_info,
            "31f0b53f3d92aa053607641946534b96488e9df8abe64268cc1f337b5e4de8b8"
        );
    }

    #[test]
    fn test_hash() {
        let package_salt = hex!("4c251b321d85cecfcb6d952ba6d81846");
        let result = utils::hash_concatenated(&[
            &package_salt,
            &constants::BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY,
        ]);

        assert_eq!(
            result,
            hex!(
                "ba1bf00eed82b07ee65e574eb1f460435d2a1405e81904fd01d5ed5adf43fdcf
                 d8e9aeebad0c08065e0db20cdc8e4552744b61ad1b3cf9a3c5aad5b2a047e76b"
            )
        );
    }
}
