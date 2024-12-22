use super::const_str::{CERTIFICATE_NS, ENCRYPTION_NS, PASSWORD_NS};
use crate::structs::{SheetProtection, WorkbookProtection};
use crate::writer::driver::{write_end_tag, write_new_line, write_start_tag};
use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes::Aes256;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use byteorder::{ByteOrder, LittleEndian};
use cbc::{Decryptor, Encryptor};
use hmac::{Hmac, Mac};
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use rand::{thread_rng, Rng};
use sha2::{Digest, Sha512};
use std::cmp::Ordering;
use std::io::{self, Write};
use std::path::Path;

type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;

/// Constants used in the encryption process
const ENCRYPTION_INFO_PREFIX: &[u8] = &[0x04, 0x00, 0x04, 0x00, 0x40, 0x00, 0x00, 0x00]; // Version and reserved bytes
const PACKAGE_ENCRYPTION_CHUNK_SIZE: usize = 4096;
const PACKAGE_OFFSET: usize = 8; // First 8 bytes are the size of the stream

// Block keys used in various stages of encryption
const BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY: &[u8] = &[0x5f, 0xb2, 0xad, 0x01, 0x0c, 0xb9, 0xe1, 0xf6];
const BLOCK_KEYS_DATA_INTEGRITY_HMAC_VALUE: &[u8] =
    &[0xa0, 0x67, 0x7f, 0x02, 0xb2, 0x2c, 0x84, 0x33];
const BLOCK_KEYS_KEY: &[u8] = &[0x14, 0x6e, 0x0b, 0xe7, 0xab, 0xac, 0xd0, 0xd6];
const BLOCK_VERIFIER_HASH_INPUT: &[u8] = &[0xfe, 0xa7, 0xd2, 0x76, 0x3b, 0x4b, 0x9e, 0x79];
const BLOCK_VERIFIER_HASH_VALUE: &[u8] = &[0xd7, 0xaa, 0x0f, 0x6d, 0x30, 0x61, 0x34, 0x4e];

/// Encrypts the sheet protection using the provided password.
/// Updates the `sheet_protection` object with algorithm name, salt value, spin count, and hash value.
pub fn encrypt_sheet_protection(password: &str, sheet_protection: &mut SheetProtection) {
    let key_salt_value = gen_random_bytes(16);
    let key_hash_algorithm = "SHA-512";
    let key_spin_count = 100_000;

    // Convert the password into a hash
    let key = convert_password_to_hash(
        password,
        key_hash_algorithm,
        &key_salt_value,
        key_spin_count,
    );

    // Encode the salt and hash value in base64
    let salt_value_str = STANDARD.encode(&key_salt_value);
    let hash_value_str = STANDARD.encode(&key);

    // Update the sheet_protection object
    sheet_protection.set_algorithm_name(key_hash_algorithm);
    sheet_protection.set_salt_value(salt_value_str);
    sheet_protection.set_spin_count(key_spin_count as u32);
    sheet_protection.set_hash_value(hash_value_str);
    sheet_protection.remove_password_raw();
}

/// Encrypts the workbook protection using the provided password.
/// Updates the `workbook_protection` object with algorithm name, salt value, spin count, and hash value.
pub fn encrypt_workbook_protection(password: &str, workbook_protection: &mut WorkbookProtection) {
    let key_salt_value = gen_random_bytes(16);
    let key_hash_algorithm = "SHA-512";
    let key_spin_count = 100_000;

    // Convert the password into a hash
    let key = convert_password_to_hash(
        password,
        key_hash_algorithm,
        &key_salt_value,
        key_spin_count,
    );

    // Encode the salt and hash value in base64
    let salt_value_str = STANDARD.encode(&key_salt_value);
    let hash_value_str = STANDARD.encode(&key);

    // Update the workbook_protection object
    workbook_protection.set_workbook_algorithm_name(key_hash_algorithm);
    workbook_protection.set_workbook_salt_value(salt_value_str);
    workbook_protection.set_workbook_spin_count(key_spin_count as u32);
    workbook_protection.set_workbook_hash_value(hash_value_str);
    workbook_protection.remove_workbook_password_raw();
}

/// Encrypts the revisions protection using the provided password.
/// Updates the `workbook_protection` object with algorithm name, salt value, spin count, and hash value.
pub fn encrypt_revisions_protection(password: &str, workbook_protection: &mut WorkbookProtection) {
    let key_salt_value = gen_random_bytes(16);
    let key_hash_algorithm = "SHA-512";
    let key_spin_count = 100_000;

    // Convert the password into a hash
    let key = convert_password_to_hash(
        password,
        key_hash_algorithm,
        &key_salt_value,
        key_spin_count,
    );

    // Encode the salt and hash value in base64
    let salt_value_str = STANDARD.encode(&key_salt_value);
    let hash_value_str = STANDARD.encode(&key);

    // Update the workbook_protection object
    workbook_protection.set_revisions_algorithm_name(key_hash_algorithm);
    workbook_protection.set_revisions_salt_value(salt_value_str);
    workbook_protection.set_revisions_spin_count(key_spin_count as u32);
    workbook_protection.set_revisions_hash_value(hash_value_str);
    workbook_protection.remove_revisions_password_raw();
}

/// Concatenates multiple byte slices into a single `Vec<u8>`.
#[expect(dead_code)]
fn buffer_concat(buffers: &[&[u8]]) -> Vec<u8> {
    // Calculate the total length of the resulting vector.
    let total_length = buffers.iter().map(|buffer| buffer.len()).sum();
    // Preallocate the vector with the total length.
    let mut result: Vec<u8> = Vec::with_capacity(total_length);
    // Extend the vector with each buffer.
    for buffer in buffers {
        result.extend_from_slice(buffer);
    }
    result
}

/// Encrypts the provided data and writes the encrypted file to the specified filepath.
/// Uses the provided password to encrypt the data.
pub fn encrypt<P: AsRef<Path>>(filepath: &P, data: &[u8], password: &str) {
    // Package parameters
    let package_key = gen_random_bytes(32);
    let package_cipher_algorithm = "AES";
    let package_hash_algorithm = "SHA512";
    let package_hash_size = 64;
    let package_block_size = 16;
    let package_key_bits = package_key.len() * 8;
    let package_salt_value = gen_random_bytes(16);

    // Key parameters
    let key_cipher_algorithm = "AES";
    let key_hash_algorithm = "SHA512";
    let key_hash_size = 64;
    let key_block_size = 16;
    let key_spin_count = 100_000;
    let key_key_bits = 256;
    let key_salt_value = gen_random_bytes(16);

    // Encrypt the package
    let encrypted_package = crypt_package(
        true,
        package_hash_algorithm,
        package_block_size,
        &package_salt_value,
        &package_key,
        data,
    );

    // Generate HMAC key and encrypt it
    let hmac_key = gen_random_bytes(64);
    let hmac_key_iv = create_iv(
        package_hash_algorithm,
        &package_salt_value,
        package_block_size,
        BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY,
    );
    let encrypted_hmac_key = crypt(true, &package_key, &hmac_key_iv, &hmac_key).unwrap();

    // Generate HMAC value and encrypt it
    let hmac_value = hmac(package_hash_algorithm, &hmac_key, &[&encrypted_package]).unwrap();
    let hmac_value_iv = create_iv(
        package_hash_algorithm,
        &package_salt_value,
        package_block_size,
        BLOCK_KEYS_DATA_INTEGRITY_HMAC_VALUE,
    );
    let encrypted_hmac_value = crypt(true, &package_key, &hmac_value_iv, &hmac_value).unwrap();

    // Convert the password to a key
    let key = convert_password_to_key(
        password,
        key_hash_algorithm,
        &key_salt_value,
        key_spin_count,
        key_key_bits,
        BLOCK_KEYS_KEY,
    );
    let encrypted_key_value = crypt(true, &key, &key_salt_value, &package_key).unwrap();

    // Generate verifier hash input and encrypt it
    let verifier_hash_input = gen_random_bytes(16);
    let verifier_hash_input_key = convert_password_to_key(
        password,
        key_hash_algorithm,
        &key_salt_value,
        key_spin_count,
        key_key_bits,
        BLOCK_VERIFIER_HASH_INPUT,
    );
    let encrypted_verifier_hash_input = crypt(
        true,
        &verifier_hash_input_key,
        &key_salt_value,
        &verifier_hash_input,
    )
    .unwrap();

    // Generate verifier hash value and encrypt it
    let verifier_hash_value = hash(key_hash_algorithm, &[&verifier_hash_input]).unwrap();
    let verifier_hash_value_key = convert_password_to_key(
        password,
        key_hash_algorithm,
        &key_salt_value,
        key_spin_count,
        key_key_bits,
        BLOCK_VERIFIER_HASH_VALUE,
    );
    let encrypted_verifier_hash_value = crypt(
        true,
        &verifier_hash_value_key,
        &key_salt_value,
        &verifier_hash_value,
    )
    .unwrap();

    // Build the encryption info XML data
    let encryption_info_buffer = build_encryption_info(
        &package_salt_value,
        package_block_size,
        package_key_bits,
        package_hash_size,
        package_cipher_algorithm,
        "ChainingModeCBC",
        package_hash_algorithm,
        &encrypted_hmac_key,
        &encrypted_hmac_value,
        key_spin_count,
        &key_salt_value,
        key_block_size,
        key_key_bits,
        key_hash_size,
        key_cipher_algorithm,
        "ChainingModeCBC",
        key_hash_algorithm,
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

/// Encrypts or decrypts the package data.
/// The package is processed in chunks, encrypting or decrypting each chunk separately.
fn crypt_package(
    encrypt: bool,
    hash_algorithm: &str,
    block_size: usize,
    salt_value: &[u8],
    key: &[u8],
    input: &[u8],
) -> Vec<u8> {
    let mut output_chunks: Vec<Vec<u8>> = Vec::new();
    let offset = if encrypt { 0 } else { PACKAGE_OFFSET };

    // Process the package in chunks
    let mut i: usize = 0;
    let mut end = 0;
    while end < input.len() {
        let start = end;
        end = (start + PACKAGE_ENCRYPTION_CHUNK_SIZE).min(input.len());

        // Get the next chunk
        let mut input_chunk = input[start + offset..end + offset].to_vec();

        // Pad the chunk if it is not a multiple of the block size
        let remainder = input_chunk.len() % block_size;
        if remainder > 0 {
            input_chunk.extend(vec![0u8; block_size - remainder]);
        }

        // Create the initialization vector (IV) for this chunk
        let block_key_buffer = create_uint32_le_buffer(i as u32, None);
        let iv = create_iv(hash_algorithm, salt_value, block_size, &block_key_buffer);

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
        let length_buffer = create_uint32_le_buffer(input_len, Some(PACKAGE_OFFSET));
        output = [length_buffer, output].concat();
    } else {
        // Truncate the output to the size specified in the prefix
        let length = LittleEndian::read_u32(&input[0..4]) as usize;
        output.truncate(length);
    }

    output
}

/// Encrypts or decrypts data using AES-256 in CBC mode with the given key and IV.
/// The `encrypt` parameter determines whether to encrypt (true) or decrypt (false).
fn crypt(encrypt: bool, key: &[u8], iv: &[u8], input: &[u8]) -> Result<Vec<u8>, String> {
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

/// Calculates an HMAC using the specified algorithm over the concatenated buffers.
fn hmac(algorithm: &str, key: &[u8], buffers: &[&[u8]]) -> Result<Vec<u8>, String> {
    match algorithm {
        "SHA512" | "SHA-512" => {
            type HmacSha512 = Hmac<Sha512>;

            let mut mac =
                HmacSha512::new_from_slice(key).map_err(|e| format!("Error creating HMAC: {e}"))?;
            for buffer in buffers {
                mac.update(buffer);
            }
            Ok(mac.finalize().into_bytes().to_vec())
        }
        _ => Err(format!("Algorithm {algorithm} not supported!")),
    }
}

/// Creates an initialization vector (IV) by hashing the salt value and the block key.
/// The resulting hash is truncated or padded to match the block size.
fn create_iv(
    hash_algorithm: &str,
    salt_value: &[u8],
    block_size: usize,
    block_key: &[u8],
) -> Vec<u8> {
    // Hash the salt value and block key together
    let mut iv = hash(hash_algorithm, &[salt_value, block_key]).unwrap();

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
        _ => {}
    }

    iv
}

/// Calculates a hash of the concatenated buffers using the specified algorithm.
fn hash(algorithm: &str, buffers: &[&[u8]]) -> Result<Vec<u8>, String> {
    let mut hasher = match algorithm {
        "SHA512" | "SHA-512" => Sha512::new(),
        _ => {
            return Err(format!("Algorithm {algorithm} not supported!"));
        }
    };

    for buffer in buffers {
        hasher.update(buffer);
    }

    Ok(hasher.finalize().to_vec())
}

/// Converts the provided password into a cryptographic key using the specified hash algorithm,
/// salt value, spin count, key length, and block key.
fn convert_password_to_key(
    password: &str,
    hash_algorithm: &str,
    salt_value: &[u8],
    spin_count: usize,
    key_bits: usize,
    block_key: &[u8],
) -> Vec<u8> {
    // Convert password to UTF-16LE bytes
    let password_bytes: Vec<u8> = password.encode_utf16().flat_map(u16::to_le_bytes).collect();

    // Generate the initial hash
    let mut key = hash(hash_algorithm, &[salt_value, &password_bytes]).unwrap();

    // Iterate spin_count times
    for i in 0..spin_count {
        let i_bytes = (i as u32).to_le_bytes();
        key = hash(hash_algorithm, &[&i_bytes, &key]).unwrap();
    }

    // Generate the final hash
    key = hash(hash_algorithm, &[&key, block_key]).unwrap();

    // Truncate or pad the key to the desired length
    let key_bytes = key_bits / 8;
    match key.len().cmp(&key_bytes) {
        Ordering::Less => {
            // Pad with zeros
            key.resize(key_bytes, 0);
            key
        }
        Ordering::Greater => key[..key_bytes].to_vec(),
        _ => key,
    }
}

/// Converts the provided password into a hash using the specified hash algorithm,
/// salt value, and spin count.
fn convert_password_to_hash(
    password: &str,
    hash_algorithm: &str,
    salt_value: &[u8],
    spin_count: usize,
) -> Vec<u8> {
    // Convert password to UTF-16LE bytes
    let password_bytes: Vec<u8> = password.encode_utf16().flat_map(u16::to_le_bytes).collect();

    // Generate the initial hash
    let mut hash_value = hash(hash_algorithm, &[salt_value, &password_bytes]).unwrap();

    // Iterate spin_count times
    for i in 0..spin_count {
        let i_bytes = (i as u32).to_le_bytes();
        hash_value = hash(hash_algorithm, &[&hash_value, &i_bytes]).unwrap();
    }

    hash_value
}

/// Generates a vector of random bytes of the specified length using a cryptographically secure random number generator.
fn gen_random_bytes(len: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    thread_rng().fill(&mut bytes[..]);
    bytes
}

/// Creates a byte buffer containing a 32-bit unsigned integer in little-endian format.
/// If `buffer_size` is specified and greater than 4, the buffer is padded to the specified size.
fn create_uint32_le_buffer(value: u32, buffer_size: Option<usize>) -> Vec<u8> {
    let mut buffer = Vec::new();
    buffer.extend(&value.to_le_bytes());
    if let Some(size) = buffer_size {
        if size > 4 {
            // Pad the buffer with zeros
            buffer.resize(size, 0);
        }
    }
    buffer
}

/// Builds the encryption info XML data.
/// Returns a vector of bytes representing the encryption info.
fn build_encryption_info(
    package_salt_value: &[u8],
    package_block_size: usize,
    package_key_bits: usize,
    package_hash_size: usize,
    package_cipher_algorithm: &str,
    package_cipher_chaining: &str,
    package_hash_algorithm: &str,
    data_integrity_encrypted_hmac_key: &[u8],
    data_integrity_encrypted_hmac_value: &[u8],
    key_spin_count: usize,
    key_salt_value: &[u8],
    key_block_size: usize,
    key_key_bits: usize,
    key_hash_size: usize,
    key_cipher_algorithm: &str,
    key_cipher_chaining: &str,
    key_hash_algorithm: &str,
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
            ("saltSize", &package_salt_value.len().to_string()),
            ("blockSize", &package_block_size.to_string()),
            ("keyBits", &package_key_bits.to_string()),
            ("hashSize", &package_hash_size.to_string()),
            ("cipherAlgorithm", package_cipher_algorithm),
            ("cipherChaining", package_cipher_chaining),
            ("hashAlgorithm", package_hash_algorithm),
            ("saltValue", &STANDARD.encode(package_salt_value)),
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
            ("spinCount", &key_spin_count.to_string()),
            ("saltSize", &key_salt_value.len().to_string()),
            ("blockSize", &key_block_size.to_string()),
            ("keyBits", &key_key_bits.to_string()),
            ("hashSize", &key_hash_size.to_string()),
            ("cipherAlgorithm", key_cipher_algorithm),
            ("cipherChaining", key_cipher_chaining),
            ("hashAlgorithm", key_hash_algorithm),
            ("saltValue", &STANDARD.encode(key_salt_value)),
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

    // Combine the ENCRYPTION_INFO_PREFIX and the XML data
    [ENCRYPTION_INFO_PREFIX, &result].concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[expect(clippy::too_many_lines)]
    fn test_encrypt() {
        const PACKAGE_CIPHER_ALGORITHM: &str = "AES";
        const KEY_CIPHER_ALGORITHM: &str = "AES";
        const KEY_HASH_SIZE: usize = 64;
        const KEY_BLOCK_SIZE: usize = 16;

        let mut file = File::open("./tests/test_files/aaa.xlsx").unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let password = "password";

        // Package parameters
        let package_key = hex!("cdf9defae2480933c503350e16334453d1cb8348bb2fea585db7f9e1f78fe9bf");
        let package_hash_algorithm = "SHA512";
        let package_block_size = 16;
        let package_salt_value = hex!("4c251b321d85cecfcb6d952ba6d81846");

        // Key parameters
        let key_salt_value = hex!("3aa973eec73c98c4710021730ef5b513");
        let key_hash_algorithm = "SHA512";
        let key_spin_count = 100_000;
        let key_key_bits = 256;

        // Encrypted package
        let encrypted_package = crypt_package(
            true,
            package_hash_algorithm,
            package_block_size,
            &package_salt_value,
            &package_key,
            &data,
        );

        // HMAC key
        let hmac_key = hex!(
            "4c6e4db6d9a60e5d41c3ca639a682aaa71da7437202fe92ec5d814bd1e9e4e6a"
            "831aee889eae3bc18bc1bebedae1f73393fddfffd0a0b6c557485fefcdb5e98b"
        );

        let hmac_key_iv = create_iv(
            package_hash_algorithm,
            &package_salt_value,
            package_block_size,
            BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY,
        );
        assert_eq!(hmac_key_iv, hex!("ba1bf00eed82b07ee65e574eb1f46043"));

        let encrypted_hmac_key = crypt(true, &package_key, &hmac_key_iv, &hmac_key).unwrap();
        assert_eq!(
            encrypted_hmac_key,
            hex!(
                "b32b1cdc4ac1af244377c1eb57efd31a819f555a7204adcc0cfe364b394bbdb0
                 86a8daef4f4c512d52e3db6a54b1d45e1dd1dbfa3ddacc29fe35449ba5225dc7"
            )
        );

        // HMAC value
        let hmac_value = hmac(package_hash_algorithm, &hmac_key, &[&encrypted_package]).unwrap();
        // Uncomment the following lines to check the HMAC value
        // let converted = encode_hex(&hmac_value);
        // assert_eq!(&converted, "41748c1ed0bcbbc46301a0a21e00747b6fafaa52ddbe4952a77a399ed4514b40c9b7e59f1c52c4cc72881794435336cc6e42fef4498245575bb9c2343480773f");

        let hmac_value_iv = create_iv(
            package_hash_algorithm,
            &package_salt_value,
            package_block_size,
            BLOCK_KEYS_DATA_INTEGRITY_HMAC_VALUE,
        );
        assert_eq!(hmac_value_iv, hex!("088385b871292e7ed8414f173c5b6622"));

        let encrypted_hmac_value = crypt(true, &package_key, &hmac_value_iv, &hmac_value).unwrap();
        // Uncomment the following lines to check the encrypted HMAC value
        // let converted = encode_hex(&encrypted_hmac_value);
        // assert_eq!(&converted, "1f6fc2877101ac12ccee6dbb0e5ea2556cc61c2c532b89ffc701fd16c5078e7e8264034ded6dc00469039f706fce12747db817574f13b49d18e914fdf4e3e93b");

        // Key
        let key = convert_password_to_key(
            password,
            key_hash_algorithm,
            &key_salt_value,
            key_spin_count,
            key_key_bits,
            BLOCK_KEYS_KEY,
        );
        assert_eq!(
            key,
            hex!("8d5869311b1c1fdb59a1de6fe1e6f2ce7dccd4deb198a6dfb1f7fb55bc03487d")
        );

        let encrypted_key_value = crypt(true, &key, &key_salt_value, &package_key).unwrap();
        assert_eq!(
            encrypted_key_value,
            hex!("5017ddc6146e56dfbf76734b3e99b80f36a4c9a2e9eb21fe77695f73850cc452")
        );

        // Verifier hash input
        let verifier_hash_input = hex!("8f54777cba87efa55ea2db8399873815");
        let verifier_hash_input_key = convert_password_to_key(
            password,
            key_hash_algorithm,
            &key_salt_value,
            key_spin_count,
            key_key_bits,
            BLOCK_VERIFIER_HASH_INPUT,
        );
        assert_eq!(
            verifier_hash_input_key,
            hex!("44e4b664c512b08e7577aa3fc7e11ad603e0877a476931fad5aa79e203304aff")
        );

        let encrypted_verifier_hash_input = crypt(
            true,
            &verifier_hash_input_key,
            &key_salt_value,
            &verifier_hash_input,
        )
        .unwrap();
        // Uncomment the following lines to check the encrypted verifier hash input
        // let converted = encode_hex(&encrypted_verifier_hash_input);
        // assert_eq!(&converted, "2fb9eea58e227ffa549449e941f1199e");

        // Verifier hash value
        let verifier_hash_value = hash(key_hash_algorithm, &[&verifier_hash_input]).unwrap();
        // Uncomment the following lines to check the verifier hash value
        // let converted = encode_hex(&verifier_hash_value);
        // assert_eq!(&converted, "920b1de74f38d9cb3ccb3394119ed37e958404fdc47560b1bf647d3c49c22549625fe4a0bd36798bd68a0d98ae64f6ab64a330c9890c62bb740aa492c226ae1f");

        let verifier_hash_value_key = convert_password_to_key(
            password,
            key_hash_algorithm,
            &key_salt_value,
            key_spin_count,
            key_key_bits,
            BLOCK_VERIFIER_HASH_VALUE,
        );
        // Uncomment the following lines to check the verifier hash value key
        // let converted = encode_hex(&verifier_hash_value_key);
        // assert_eq!(
        //     &converted,
        //     "d5515a6062e3e99551b80b92db1fe646483884cdb63e1e7595a9f2cca7532884"
        // );

        let encrypted_verifier_hash_value = crypt(
            true,
            &verifier_hash_value_key,
            &key_salt_value,
            &verifier_hash_value,
        )
        .unwrap();
        // Uncomment the following lines to check the encrypted verifier hash value
        // let converted = encode_hex(&encrypted_verifier_hash_value);
        // assert_eq!(
        //     &converted,
        //     "0d9c888111b40b630b739c95a5f5b6be67c8f96acdd1bee185bd808b507f652760a2e77f63a6ad0c46f985f2bb8dab4fcf9b86d6a40d9c21299bb4ddf788b250"
        // );

        // Build encryption info
        let _unused = build_encryption_info(
            &package_salt_value,
            package_block_size,
            package_key.len() * 8,
            64, // package_hash_size
            PACKAGE_CIPHER_ALGORITHM,
            "ChainingModeCBC",
            package_hash_algorithm,
            &encrypted_hmac_key,
            &encrypted_hmac_value,
            key_spin_count,
            &key_salt_value,
            KEY_BLOCK_SIZE,
            key_key_bits,
            KEY_HASH_SIZE,
            KEY_CIPHER_ALGORITHM,
            "ChainingModeCBC",
            key_hash_algorithm,
            &encrypted_verifier_hash_input,
            &encrypted_verifier_hash_value,
            &encrypted_key_value,
        );
    }

    #[test]
    fn test_hash() {
        let package_salt_value = hex!("4c251b321d85cecfcb6d952ba6d81846");
        let result = hash(
            "SHA512",
            &[&package_salt_value, BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY],
        )
        .unwrap();
        assert_eq!(
            result,
            hex!(
                "ba1bf00eed82b07ee65e574eb1f460435d2a1405e81904fd01d5ed5adf43fdcf
                 d8e9aeebad0c08065e0db20cdc8e4552744b61ad1b3cf9a3c5aad5b2a047e76b"
            )
        );
    }

    #[test]
    fn test_buffer_slice() {
        // Since buffer_slice is not defined in the new code, we can replicate its functionality inline
        let buffer = hex!(
            "ba1bf00eed82b07ee65e574eb1f460435d2a1405e81904fd01d5ed5adf43fdcf
             d8e9aeebad0c08065e0db20cdc8e4552744b61ad1b3cf9a3c5aad5b2a047e76b"
        );
        let start = 0;
        let end = 16;
        let result = buffer[start..end].to_vec();
        assert_eq!(result, hex!("ba1bf00eed82b07ee65e574eb1f46043"));
    }

    #[test]
    fn test_convert_password_to_key() {
        let key_salt_value = hex!("3aa973eec73c98c4710021730ef5b513");
        let result = convert_password_to_key(
            "password",
            "SHA512",
            &key_salt_value,
            100_000,
            256,
            BLOCK_KEYS_KEY,
        );
        assert_eq!(
            result,
            hex!("8d5869311b1c1fdb59a1de6fe1e6f2ce7dccd4deb198a6dfb1f7fb55bc03487d")
        );
    }
}
