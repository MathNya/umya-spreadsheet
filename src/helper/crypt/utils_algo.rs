use std::io;

use aes::{
    Aes256,
    cipher::{
        BlockDecryptMut,
        BlockEncryptMut,
        KeyIvInit,
        block_padding::NoPadding,
    },
};
use base64::{
    Engine as _,
    engine::general_purpose::STANDARD,
};
use byteorder::{
    ByteOrder,
    LittleEndian,
};
use cbc::{
    Decryptor,
    Encryptor,
};
use quick_xml::{
    Writer,
    events::{
        BytesDecl,
        Event,
    },
};

type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;

use super::{
    super::const_str::{
        CERTIFICATE_NS,
        ENCRYPTION_NS,
        PASSWORD_NS,
    },
    constants,
    utils_key::create_iv,
};
use crate::writer::driver::{
    write_end_tag,
    write_new_line,
    write_start_tag,
};

/// Encrypts or decrypts package data using AES-256 in CBC mode.
///
/// # Parameters
/// - `encrypt`: Whether to encrypt (true) or decrypt (false) the data
/// - `block_size`: Size of encryption blocks in bytes
/// - `salt`: Salt value for IV generation
/// - `key`: Encryption/decryption key
/// - `input`: Data to encrypt/decrypt
///
/// # Returns
/// A vector containing the encrypted/decrypted data. For encryption, includes
/// an 8-byte length prefix. For decryption, output is truncated to the original
/// length.
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn crypt_package(
    encrypt: bool,
    block_size: usize,
    salt: &[u8],
    key: &[u8],
    input: &[u8],
) -> Vec<u8> {
    let mut output_chunks: Vec<Vec<u8>> = Vec::new();
    let offset = if encrypt {
        0
    } else {
        constants::PACKAGE_OFFSET
    };

    // Process the package in chunks
    let mut i: usize = 0;
    let mut end = 0;
    while end < input.len() {
        let start = end;
        end = (start + constants::PACKAGE_ENCRYPTION_CHUNK_SIZE).min(input.len());

        // Get the next chunk
        let mut input_chunk = input[start + offset..end + offset].to_vec();

        // Pad the chunk if it is not a multiple of the block size
        let remainder = input_chunk.len() % block_size;
        if remainder > 0 {
            input_chunk.extend(vec![0u8; block_size - remainder]);
        }

        // Create the initialization vector (IV) for this chunk
        let block_key_buffer = create_uint32_le_buffer(i as u32, None);
        let iv = create_iv(salt, block_size, &block_key_buffer);

        // Encrypt or decrypt the chunk
        let output_chunk = crypt(encrypt, key, &iv, &input_chunk).unwrap();
        output_chunks.push(output_chunk);

        i += 1;
    }

    // Concatenate all output chunks
    let mut output = output_chunks.concat();

    if encrypt {
        // Prepend the length of the package in the first 8 bytes
        let input_len = input.len() as u32;
        let length_buffer = create_uint32_le_buffer(input_len, Some(constants::PACKAGE_OFFSET));
        output = [length_buffer, output].concat();
    } else {
        // Truncate the output to the size specified in the prefix
        let length = LittleEndian::read_u32(&input[0..4]) as usize;
        output.truncate(length);
    }

    output
}

/// Performs AES-256-CBC encryption or decryption on input data.
///
/// # Arguments
/// * `encrypt` - If true, encrypts the data. If false, decrypts the data
/// * `key` - 256-bit encryption/decryption key
/// * `iv` - Initialization vector for CBC mode
/// * `input` - Data to be encrypted/decrypted
///
/// # Returns
/// * `Ok(Vec<u8>)` - Encrypted/decrypted data
/// * `Err(String)` - Error message if operation fails
pub(crate) fn crypt(encrypt: bool, key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, String> {
    match key.len() * 8 {
        256 => {
            if encrypt {
                // Encrypt the input data
                let cipher = Aes256CbcEnc::new_from_slices(key, iv)
                    .map_err(|e| format!("Error creating cipher: {e}"))?;
                let mut buffer = input.to_vec();
                cipher
                    .encrypt_padded_mut::<NoPadding>(&mut buffer, input.len())
                    .map_err(|e| format!("Encryption error: {e}"))?;
                Ok(buffer)
            } else {
                // Decrypt the input data
                let cipher = Aes256CbcDec::new_from_slices(key, iv)
                    .map_err(|e| format!("Error creating cipher: {e}"))?;
                let mut buffer = input.to_vec();
                cipher
                    .decrypt_padded_mut::<NoPadding>(&mut buffer)
                    .map_err(|e| format!("Decryption error: {e}"))?;
                Ok(buffer)
            }
        }
        _ => Err("Key size not supported!".to_string()),
    }
}

/// Creates a little-endian byte buffer from a `u32` value.
///
/// If `buffer_size` is provided and greater than 4, the buffer is padded with
/// zeros to match the specified size.
///
/// # Parameters
/// - `value`: The `u32` value to convert.
/// - `buffer_size`: Optional desired size of the buffer.
///
/// # Returns
/// A `Vec<u8>` containing the little-endian bytes of `value`, optionally padded
/// with zeros.
pub(crate) fn create_uint32_le_buffer(value: u32, buffer_size: Option<usize>) -> Vec<u8> {
    let mut buffer = value.to_le_bytes().to_vec();
    if let Some(size) = buffer_size.filter(|&s| s > 4) {
        buffer.resize(size, 0);
    }
    buffer
}

/// Constructs the encryption info XML data for document protection.
///
/// # Arguments
/// * `package_salt` - Salt value for package encryption
/// * `data_integrity_encrypted_hmac_key` - Encrypted HMAC key for data
///   integrity
/// * `data_integrity_encrypted_hmac_value` - Encrypted HMAC value for data
///   integrity
/// * `key_salt` - Salt value for key encryption
/// * `key_encrypted_verifier_hash_input` - Encrypted verifier hash input
/// * `key_encrypted_verifier_hash_value` - Encrypted verifier hash value
/// * `key_encrypted_key_value` - Encrypted key value
///
/// # Returns
/// A vector of bytes containing the encryption info XML prefixed with the
/// standard header
pub(crate) fn build_encryption_info(
    package_salt: &[u8],
    data_integrity_encrypted_hmac_key: &[u8],
    data_integrity_encrypted_hmac_value: &[u8],
    key_salt: &[u8],
    key_encrypted_verifier_hash_input: &[u8],
    key_encrypted_verifier_hash_value: &[u8],
    key_encrypted_key_value: &[u8],
) -> Vec<u8> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer
        .write_event(Event::Decl(BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            Some("yes"),
        )))
        .unwrap();
    write_new_line(&mut writer);

    // Start encryption element
    write_start_tag(
        &mut writer,
        "encryption",
        vec![
            ("xmlns", ENCRYPTION_NS),
            ("xmlns:p", PASSWORD_NS),
            ("xmlns:c", CERTIFICATE_NS),
        ],
        false,
    );

    // keyData element
    write_start_tag(
        &mut writer,
        "keyData",
        vec![
            ("saltSize", &package_salt.len().to_string()),
            ("blockSize", &constants::PACKAGE_BLOCK_SIZE.to_string()),
            ("keyBits", &constants::PACKAGE_KEY_BITS.to_string()),
            ("hashSize", &constants::PACKAGE_HASH_SIZE.to_string()),
            ("cipherAlgorithm", constants::PACKAGE_CIPHER_ALGORITHM),
            ("cipherChaining", constants::PACKAGE_CIPHER_CHAINING),
            ("hashAlgorithm", constants::PACKAGE_HASH_ALGORITHM),
            ("saltValue", &STANDARD.encode(package_salt)),
        ],
        true,
    );

    // dataIntegrity element
    write_start_tag(
        &mut writer,
        "dataIntegrity",
        vec![
            (
                "encryptedHmacKey",
                &STANDARD.encode(data_integrity_encrypted_hmac_key),
            ),
            (
                "encryptedHmacValue",
                &STANDARD.encode(data_integrity_encrypted_hmac_value),
            ),
        ],
        true,
    );

    // keyEncryptors element
    write_start_tag(&mut writer, "keyEncryptors", vec![], false);

    // keyEncryptor element
    write_start_tag(
        &mut writer,
        "keyEncryptor",
        vec![("uri", PASSWORD_NS)],
        false,
    );

    // p:encryptedKey element
    write_start_tag(
        &mut writer,
        "p:encryptedKey",
        vec![
            ("spinCount", &constants::KEY_SPIN_COUNT.to_string()),
            ("saltSize", &key_salt.len().to_string()),
            ("blockSize", &constants::KEY_BLOCK_SIZE.to_string()),
            ("keyBits", &constants::KEY_BITLENGTH.to_string()),
            ("hashSize", &constants::KEY_HASH_SIZE.to_string()),
            ("cipherAlgorithm", constants::KEY_CIPHER_ALGORITHM),
            ("cipherChaining", constants::KEY_CIPHER_CHAINING),
            ("hashAlgorithm", constants::KEY_HASH_ALGORITHM),
            ("saltValue", &STANDARD.encode(key_salt)),
            (
                "encryptedVerifierHashInput",
                &STANDARD.encode(key_encrypted_verifier_hash_input),
            ),
            (
                "encryptedVerifierHashValue",
                &STANDARD.encode(key_encrypted_verifier_hash_value),
            ),
            (
                "encryptedKeyValue",
                &STANDARD.encode(key_encrypted_key_value),
            ),
        ],
        true,
    );

    // Close tags
    write_end_tag(&mut writer, "keyEncryptor");
    write_end_tag(&mut writer, "keyEncryptors");
    write_end_tag(&mut writer, "encryption");

    let result = writer.into_inner().into_inner().clone();

    // Combine the constants::ENCRYPTION_INFO_PREFIX and the XML data
    [constants::ENCRYPTION_INFO_PREFIX.to_vec(), result].concat()
}
