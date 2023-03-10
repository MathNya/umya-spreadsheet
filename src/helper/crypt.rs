use aes::cipher::{block_padding::NoPadding, BlockEncryptMut, KeyIvInit};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use byteorder::{ByteOrder, LittleEndian};
use cfb;
use hmac::{Hmac, Mac};
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use sha2::{Digest, Sha512};
use std::io;
use std::io::Write;
use std::path::Path;
use writer::driver::*;

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;

const ENCRYPTION_INFO_PREFIX: &[u8] = &[0x04, 0x00, 0x04, 0x00, 0x40, 0x00, 0x00, 0x00]; // First 4 bytes are the version number, second 4 bytes are reserved.
const PACKAGE_ENCRYPTION_CHUNK_SIZE: usize = 4096;
const PACKAGE_OFFSET: usize = 8; // First 8 bytes are the size of the stream
const BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY: &[u8] = &[0x5f, 0xb2, 0xad, 0x01, 0x0c, 0xb9, 0xe1, 0xf6];
const BLOCK_KEYS_DATA_INTEGRITY_HMAC_VALUE: &[u8] =
    &[0xa0, 0x67, 0x7f, 0x02, 0xb2, 0x2c, 0x84, 0x33];
const BLOCK_KEYS_KEY: &[u8] = &[0x14, 0x6e, 0x0b, 0xe7, 0xab, 0xac, 0xd0, 0xd6];
const BLOCK_VERIFIER_HASH_INPUT: &[u8] = &[0xfe, 0xa7, 0xd2, 0x76, 0x3b, 0x4b, 0x9e, 0x79];
const BLOCK_VERIFIER_HASH_VALUE: &[u8] = &[0xd7, 0xaa, 0x0f, 0x6d, 0x30, 0x61, 0x34, 0x4e];

pub fn encrypt<P: AsRef<Path>>(filepath: &P, data: &Vec<u8>, password: &str) {
    // package params
    let package_key = gen_random_32();
    let package_cipher_algorithm = "AES";
    let package_cipher_chaining = "ChainingModeCBC";
    let package_salt_value = gen_random_16();
    let package_hash_algorithm = "SHA512";
    let package_hash_size = 64;
    let package_block_size = 16;
    let package_key_bits = package_key.len() * 8;

    // key params
    let key_cipher_algorithm = "AES";
    let key_cipher_chaining = "ChainingModeCBC";
    let key_salt_value = gen_random_16();
    let key_hash_algorithm = "SHA512";
    let key_hash_size = 64;
    let key_block_size = 16;
    let key_spin_count = 100000;
    let key_key_bits = 256;

    // encrypted_package
    let encrypted_package = crypt_package(
        &true,
        package_cipher_algorithm,
        package_cipher_chaining,
        package_hash_algorithm,
        &package_block_size,
        &package_salt_value,
        &package_key,
        data,
    );

    // hmac key
    let hmac_key = gen_random_64();
    let hmac_key_iv = create_iv(
        package_hash_algorithm,
        &package_salt_value,
        &package_block_size,
        &BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY.to_vec(),
    );
    let encrypted_hmac_key = crypt(
        &true,
        package_cipher_algorithm,
        package_cipher_chaining,
        &package_key,
        &hmac_key_iv,
        &hmac_key,
    )
    .unwrap();

    // hmac value
    let hmac_value = hmac(package_hash_algorithm, &hmac_key, vec![&encrypted_package]).unwrap();
    let hmac_value_iv = create_iv(
        package_hash_algorithm,
        &package_salt_value,
        &package_block_size,
        &BLOCK_KEYS_DATA_INTEGRITY_HMAC_VALUE.to_vec(),
    );
    let encrypted_hmac_value = crypt(
        &true,
        package_cipher_algorithm,
        package_cipher_chaining,
        &package_key,
        &hmac_value_iv,
        &hmac_value,
    )
    .unwrap();

    // key
    let key = convert_password_to_key(
        password,
        key_hash_algorithm,
        &key_salt_value,
        &key_spin_count,
        &key_key_bits,
        &BLOCK_KEYS_KEY.to_vec(),
    );
    let encrypted_key_value = crypt(
        &true,
        key_cipher_algorithm,
        key_cipher_chaining,
        &key,
        &key_salt_value,
        &package_key,
    )
    .unwrap();

    // verifier_hash_input
    let verifier_hash_input = gen_random_16();
    let verifier_hash_input_key = convert_password_to_key(
        password,
        key_hash_algorithm,
        &key_salt_value,
        &key_spin_count,
        &key_key_bits,
        &BLOCK_VERIFIER_HASH_INPUT.to_vec(),
    );
    let encrypted_verifier_hash_input = crypt(
        &true,
        key_cipher_algorithm,
        key_cipher_chaining,
        &verifier_hash_input_key,
        &key_salt_value,
        &verifier_hash_input,
    )
    .unwrap();

    // verifier_hash_value
    let verifier_hash_value = hash(key_hash_algorithm, vec![&verifier_hash_input]).unwrap();
    let verifier_hash_value_key = convert_password_to_key(
        password,
        key_hash_algorithm,
        &key_salt_value,
        &key_spin_count,
        &key_key_bits,
        &BLOCK_VERIFIER_HASH_VALUE.to_vec(),
    );
    let encrypted_verifier_hash_value = crypt(
        &true,
        key_cipher_algorithm,
        key_cipher_chaining,
        &verifier_hash_value_key,
        &key_salt_value,
        &verifier_hash_value,
    )
    .unwrap();

    // XML
    let encryption_info_buffer = build_encryption_info(
        &package_salt_value,
        &package_block_size,
        &package_key_bits,
        &package_hash_size,
        package_cipher_algorithm,
        package_cipher_chaining,
        package_hash_algorithm,
        &encrypted_hmac_key,
        &encrypted_hmac_value,
        &key_spin_count,
        &key_salt_value,
        &key_block_size,
        &key_key_bits,
        &key_hash_size,
        key_cipher_algorithm,
        key_cipher_chaining,
        key_hash_algorithm,
        &encrypted_verifier_hash_input,
        &encrypted_verifier_hash_value,
        &encrypted_key_value,
    );

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

// Encrypt/decrypt the package
fn crypt_package(
    encrypt: &bool,
    cipher_algorithm: &str,
    cipher_chaining: &str,
    hash_algorithm: &str,
    block_size: &usize,
    salt_value: &Vec<u8>,
    key: &Vec<u8>,
    input: &Vec<u8>,
) -> Vec<u8> {
    // The first 8 bytes is supposed to be the length, but it seems like it is really the length - 4..
    let mut output_chunks: Vec<Vec<u8>> = Vec::new();
    let offset = if encrypt == &true { 0 } else { PACKAGE_OFFSET };

    // The package is encoded in chunks. Encrypt/decrypt each and concat.
    let mut i: usize = 0;
    let mut end = 0;
    while end < input.len() {
        let start = end;
        end = start + PACKAGE_ENCRYPTION_CHUNK_SIZE;
        if end > input.len() {
            end = input.len();
        };

        // Grab the next chunk
        let mut input_chunk = buffer_slice(input, start + offset, end + offset);

        // Pad the chunk if it is not an integer multiple of the block size
        let remainder = input_chunk.len() % block_size;
        if remainder > 0 {
            let buffer = buffer_alloc(0, block_size - remainder);
            input_chunk = buffer_concat(vec![&input_chunk, &buffer]);
        }

        // Create the initialization vector
        // Create the block key from the current index
        let block_key_buffer = create_uint32_le_buffer(&(i as u32), None);
        let iv = create_iv(hash_algorithm, salt_value, block_size, &block_key_buffer);

        // Encrypt/decrypt the chunk and add it to the array
        let output_chunk = crypt(
            encrypt,
            cipher_algorithm,
            cipher_chaining,
            key,
            &iv,
            &input_chunk,
        )
        .unwrap();
        output_chunks.push(output_chunk);

        i += 1;
    }

    // Concat all of the output chunks.
    let output_chunks_as: Vec<&Vec<u8>> = output_chunks.iter().map(AsRef::as_ref).collect();
    let mut output = buffer_concat(output_chunks_as);

    if encrypt == &true {
        // Put the length of the package in the first 8 bytes
        let input_len = input.len();
        output = buffer_concat(vec![
            &create_uint32_le_buffer(&(input_len as u32), Some(&PACKAGE_OFFSET)),
            &output,
        ]);
    } else {
        // Truncate the buffer to the size in the prefix
        let length = buffer_read_u_int32_le(input, &0);
        output = output[0..length as usize].to_vec();
    }

    output
}

// Create an initialization vector (IV)
fn create_iv(
    hash_algorithm: &str,
    salt_value: &Vec<u8>,
    block_size: &usize,
    block_key: &Vec<u8>,
) -> Vec<u8> {
    // Create the initialization vector by hashing the salt with the block key.
    // Truncate or pad as needed to meet the block size.
    let mut iv = hash(hash_algorithm, vec![salt_value, block_key]).unwrap();
    if &iv.len() < block_size {
        let mut tmp = buffer_alloc(0x36, *block_size);
        buffer_copy(&mut tmp, &iv);
        iv = tmp;
    } else if &iv.len() > block_size {
        iv = buffer_slice(&iv, 0, *block_size);
    }
    iv
}

// Encrypt/decrypt input
fn crypt(
    _encrypt: &bool,
    _cipher_algorithm: &str,
    _cipher_chaining: &str,
    key: &Vec<u8>,
    iv: &Vec<u8>,
    input: &Vec<u8>,
) -> Result<Vec<u8>, String> {
    let mut buf = [0u8; 4096];
    let pt_len = input.len();
    buf[..pt_len].copy_from_slice(input);
    let ct = match key.len() * 8 {
        256 => Aes256CbcEnc::new_from_slices(key, iv)
            .unwrap()
            .encrypt_padded_mut::<NoPadding>(&mut buf, pt_len)
            .unwrap(),
        _ => {
            return Err("key size not supported!".to_string());
        }
    };
    Ok(ct.to_vec())
}

fn hmac(algorithm: &str, key: &Vec<u8>, buffers: Vec<&Vec<u8>>) -> Result<Vec<u8>, String> {
    let mut mac = match algorithm {
        "SHA512" => {
            type HmacSha512 = Hmac<Sha512>;
            HmacSha512::new_from_slice(key).unwrap()
        }
        _ => {
            return Err(format!("algorithm {} not supported!", algorithm));
        }
    };
    mac.update(&buffer_concat(buffers));

    let result = mac.finalize();
    Ok(result.into_bytes()[..].to_vec())
}

fn convert_password_to_key(
    password: &str,
    hash_algorithm: &str,
    salt_value: &Vec<u8>,
    spin_count: &usize,
    key_bits: &usize,
    block_key: &Vec<u8>,
) -> Vec<u8> {
    // Password must be in unicode buffer
    let mut password_buffer: Vec<u8> = Vec::new();
    let v: Vec<u16> = password.encode_utf16().collect();
    for a in v {
        let d = a.to_le_bytes();
        password_buffer.push(d[0]);
        password_buffer.push(d[1]);
    }

    // Generate the initial hash
    let mut key = hash(hash_algorithm, vec![salt_value, &password_buffer]).unwrap();

    // Now regenerate until spin count
    for i in 0..*spin_count {
        let iterator = create_uint32_le_buffer(&(i as u32), None);
        key = hash(hash_algorithm, vec![&iterator, &key]).unwrap();
    }

    // Now generate the final hash
    key = hash(hash_algorithm, vec![&key, block_key]).unwrap();

    // Truncate or pad as needed to get to length of keyBits
    let key_bytes = key_bits / 8;
    if key.len() < key_bytes {
        let mut tmp = buffer_alloc(0x36, key_bytes);
        buffer_copy(&mut tmp, &key);
        key = tmp;
    } else if key.len() > key_bytes {
        key = buffer_slice(&key, 0, key_bytes);
    }

    key
}

// Calculate a hash of the concatenated buffers with the given algorithm.
fn hash(algorithm: &str, buffers: Vec<&Vec<u8>>) -> Result<Vec<u8>, String> {
    let mut digest = match algorithm {
        "SHA512" => Sha512::new(),
        _ => {
            return Err(format!("algorithm {} not supported!", algorithm));
        }
    };
    digest.update(&buffer_concat(buffers)[..]);
    Ok(digest.finalize().to_vec())
}

fn gen_random_16() -> Vec<u8> {
    let buf: &mut [u8] = &mut [0; 16];
    let _ = getrandom::getrandom(buf);
    buf.to_vec()
}

fn gen_random_32() -> Vec<u8> {
    let buf: &mut [u8] = &mut [0; 32];
    let _ = getrandom::getrandom(buf);
    buf.to_vec()
}

fn gen_random_64() -> Vec<u8> {
    let buf: &mut [u8] = &mut [0; 64];
    let _ = getrandom::getrandom(buf);
    buf.to_vec()
}

// Create a buffer of an integer encoded as a uint32le
fn create_uint32_le_buffer(value: &u32, buffer_size: Option<&usize>) -> Vec<u8> {
    let bs_prm = buffer_size.unwrap_or(&4);
    let mut buffer = buffer_alloc(0, *bs_prm);
    buffer_write_u_int32_le(&mut buffer, value, &0);
    buffer
}

fn build_encryption_info(
    package_salt_value: &Vec<u8>,
    package_block_size: &usize,
    package_key_bits: &usize,
    package_hash_size: &usize,
    package_cipher_algorithm: &str,
    package_cipher_chaining: &str,
    package_hash_algorithm: &str,
    data_integrity_encrypted_hmac_key: &Vec<u8>,
    data_integrity_encrypted_hmac_value: &Vec<u8>,
    key_spin_count: &usize,
    key_salt_value: &Vec<u8>,
    key_block_size: &usize,
    key_key_bits: &usize,
    key_hash_size: &usize,
    key_cipher_algorithm: &str,
    key_cipher_chaining: &str,
    key_hash_algorithm: &str,
    key_encrypted_verifier_hash_input: &Vec<u8>,
    key_encrypted_verifier_hash_value: &Vec<u8>,
    key_encrypted_key_value: &Vec<u8>,
) -> Vec<u8> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    // Map the object into the appropriate XML structure. Buffers are encoded in base 64.
    write_start_tag(
        &mut writer,
        "encryption",
        vec![
            (
                "xmlns",
                "http://schemas.microsoft.com/office/2006/encryption",
            ),
            (
                "xmlns:p",
                "http://schemas.microsoft.com/office/2006/keyEncryptor/password",
            ),
            (
                "xmlns:c",
                "http://schemas.microsoft.com/office/2006/keyEncryptor/certificate",
            ),
        ],
        false,
    );
    let str_package_salt_value_len = package_salt_value.len().to_string();
    let str_package_block_size = package_block_size.to_string();
    let str_package_key_bits = package_key_bits.to_string();
    let str_package_hash_size = package_hash_size.to_string();
    let str_package_salt_value = STANDARD.encode(package_salt_value);
    write_start_tag(
        &mut writer,
        "keyData",
        vec![
            ("saltSize", &str_package_salt_value_len),
            ("blockSize", &str_package_block_size),
            ("keyBits", &str_package_key_bits),
            ("hashSize", &str_package_hash_size),
            ("cipherAlgorithm", package_cipher_algorithm),
            ("cipherChaining", package_cipher_chaining),
            ("hashAlgorithm", package_hash_algorithm),
            ("saltValue", &str_package_salt_value),
        ],
        true,
    );
    let str_data_integrity_encrypted_hmac_key = STANDARD.encode(data_integrity_encrypted_hmac_key);
    let str_data_integrity_encrypted_hmac_value =
        STANDARD.encode(data_integrity_encrypted_hmac_value);
    write_start_tag(
        &mut writer,
        "dataIntegrity",
        vec![
            ("encryptedHmacKey", &str_data_integrity_encrypted_hmac_key),
            (
                "encryptedHmacValue",
                &str_data_integrity_encrypted_hmac_value,
            ),
        ],
        true,
    );
    write_start_tag(&mut writer, "keyEncryptors", vec![], false);
    write_start_tag(
        &mut writer,
        "keyEncryptor",
        vec![(
            "uri",
            "http://schemas.microsoft.com/office/2006/keyEncryptor/password",
        )],
        false,
    );
    let str_key_spin_count = key_spin_count.to_string();
    let str_key_salt_value_len = key_salt_value.len().to_string();
    let str_key_block_size = key_block_size.to_string();
    let str_key_key_bits = key_key_bits.to_string();
    let str_key_hash_size = key_hash_size.to_string();
    let str_key_salt_value = STANDARD.encode(key_salt_value);
    let str_key_encrypted_verifier_hash_input = STANDARD.encode(key_encrypted_verifier_hash_input);
    let str_key_encrypted_verifier_hash_value = STANDARD.encode(key_encrypted_verifier_hash_value);
    let str_key_key_encrypted_key_value = STANDARD.encode(key_encrypted_key_value);
    write_start_tag(
        &mut writer,
        "p:encryptedKey",
        vec![
            ("spinCount", &str_key_spin_count),
            ("saltSize", &str_key_salt_value_len),
            ("blockSize", &str_key_block_size),
            ("keyBits", &str_key_key_bits),
            ("hashSize", &str_key_hash_size),
            ("cipherAlgorithm", key_cipher_algorithm),
            ("cipherChaining", key_cipher_chaining),
            ("hashAlgorithm", key_hash_algorithm),
            ("saltValue", &str_key_salt_value),
            (
                "encryptedVerifierHashInput",
                &str_key_encrypted_verifier_hash_input,
            ),
            (
                "encryptedVerifierHashValue",
                &str_key_encrypted_verifier_hash_value,
            ),
            ("encryptedKeyValue", &str_key_key_encrypted_key_value),
        ],
        true,
    );

    write_end_tag(&mut writer, "keyEncryptor");
    write_end_tag(&mut writer, "keyEncryptors");
    write_end_tag(&mut writer, "encryption");

    let result = writer.into_inner().into_inner().to_vec();
    buffer_concat(vec![&ENCRYPTION_INFO_PREFIX.to_vec(), &result])
}

fn buffer_slice(buffer: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    buffer[start..end].to_vec()
}

fn buffer_alloc(alloc_char: u8, size: usize) -> Vec<u8> {
    vec![alloc_char; size]
}

fn buffer_concat(buffers: Vec<&Vec<u8>>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for buffer in buffers {
        result.extend(buffer);
    }
    result
}
fn buffer_copy(buffer1: &mut Vec<u8>, buffer2: &Vec<u8>) {
    let mut i = 0;
    for byte in buffer2 {
        let _ = std::mem::replace(&mut buffer1[i], *byte);
        i += 1;
    }
}

fn buffer_read_u_int32_le(buffer: &Vec<u8>, _cnt: &usize) -> u32 {
    LittleEndian::read_u32(buffer)
}

fn buffer_write_u_int32_le(buffer: &mut Vec<u8>, value: &u32, _cnt: &usize) {
    LittleEndian::write_u32(buffer, *value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::{fmt::Write, num::ParseIntError};

    fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }

    fn encode_hex(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            write!(&mut s, "{:02x}", b).unwrap();
        }
        s
    }

    #[test]
    fn test_encrypt() {
        let mut file = File::open("./tests/test_files/aaa.xlsx").unwrap();
        let mut data = Vec::new();
        let _ = file.read_to_end(&mut data).unwrap();

        let password = "password";

        // package params
        let package_key =
            decode_hex("cdf9defae2480933c503350e16334453d1cb8348bb2fea585db7f9e1f78fe9bf").unwrap();
        let package_cipher_algorithm = "AES";
        let package_cipher_chaining = "ChainingModeCBC";
        let package_salt_value = decode_hex("4c251b321d85cecfcb6d952ba6d81846").unwrap();
        let package_hash_algorithm = "SHA512";
        let package_hash_size = 64;
        let package_block_size = 16;
        let package_key_bits = package_key.len() * 8;

        // key params
        let key_cipher_algorithm = "AES";
        let key_cipher_chaining = "ChainingModeCBC";
        let key_salt_value = decode_hex("3aa973eec73c98c4710021730ef5b513").unwrap();
        let key_hash_algorithm = "SHA512";
        let key_hash_size = 64;
        let key_block_size = 16;
        let key_spin_count = 100000;
        let key_key_bits = 256;

        // encrypted_package
        let encrypted_package = crypt_package(
            &true,
            package_cipher_algorithm,
            package_cipher_chaining,
            package_hash_algorithm,
            &package_block_size,
            &package_salt_value,
            &package_key,
            &data,
        );

        // hmac key
        let hmac_key = decode_hex("4c6e4db6d9a60e5d41c3ca639a682aaa71da7437202fe92ec5d814bd1e9e4e6a831aee889eae3bc18bc1bebedae1f73393fddfffd0a0b6c557485fefcdb5e98b").unwrap();
        let hmac_key_iv = create_iv(
            package_hash_algorithm,
            &package_salt_value,
            &package_block_size,
            &BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY.to_vec(),
        );
        let converted = encode_hex(&hmac_key_iv);
        assert_eq!(&converted, "ba1bf00eed82b07ee65e574eb1f46043");

        let encrypted_hmac_key = crypt(
            &true,
            package_cipher_algorithm,
            package_cipher_chaining,
            &package_key,
            &hmac_key_iv,
            &hmac_key,
        )
        .unwrap();
        let converted = encode_hex(&encrypted_hmac_key);
        assert_eq!(&converted, "b32b1cdc4ac1af244377c1eb57efd31a819f555a7204adcc0cfe364b394bbdb086a8daef4f4c512d52e3db6a54b1d45e1dd1dbfa3ddacc29fe35449ba5225dc7");

        // hmac value
        let hmac_value = hmac(package_hash_algorithm, &hmac_key, vec![&encrypted_package]).unwrap();
        //let converted = encode_hex(&hmac_value);
        //assert_eq!(&converted, "41748c1ed0bcbbc46301a0a21e00747b6fafaa52ddbe4952a77a399ed4514b40c9b7e59f1c52c4cc72881794435336cc6e42fef4498245575bb9c2343480773f");

        let hmac_value_iv = create_iv(
            package_hash_algorithm,
            &package_salt_value,
            &package_block_size,
            &BLOCK_KEYS_DATA_INTEGRITY_HMAC_VALUE.to_vec(),
        );
        let converted = encode_hex(&hmac_value_iv);
        assert_eq!(&converted, "088385b871292e7ed8414f173c5b6622");

        let encrypted_hmac_value = crypt(
            &true,
            package_cipher_algorithm,
            package_cipher_chaining,
            &package_key,
            &hmac_value_iv,
            &hmac_value,
        )
        .unwrap();
        //let converted = encode_hex(&encrypted_hmac_value);
        //assert_eq!(&converted, "1f6fc2877101ac12ccee6dbb0e5ea2556cc61c2c532b89ffc701fd16c5078e7e8264034ded6dc00469039f706fce12747db817574f13b49d18e914fdf4e3e93b");

        // key
        let key = convert_password_to_key(
            password,
            key_hash_algorithm,
            &key_salt_value,
            &key_spin_count,
            &key_key_bits,
            &BLOCK_KEYS_KEY.to_vec(),
        );
        let converted = encode_hex(&key);
        assert_eq!(
            &converted,
            "8d5869311b1c1fdb59a1de6fe1e6f2ce7dccd4deb198a6dfb1f7fb55bc03487d"
        );

        let encrypted_key_value = crypt(
            &true,
            key_cipher_algorithm,
            key_cipher_chaining,
            &key,
            &key_salt_value,
            &package_key,
        )
        .unwrap();
        let converted = encode_hex(&encrypted_key_value);
        assert_eq!(
            &converted,
            "5017ddc6146e56dfbf76734b3e99b80f36a4c9a2e9eb21fe77695f73850cc452"
        );

        // verifier_hash_input
        let verifier_hash_input = "8f54777cba87efa55ea2db8399873815".as_bytes().to_vec();
        let verifier_hash_input_key = convert_password_to_key(
            password,
            key_hash_algorithm,
            &key_salt_value,
            &key_spin_count,
            &key_key_bits,
            &BLOCK_VERIFIER_HASH_INPUT.to_vec(),
        );
        let converted = encode_hex(&verifier_hash_input_key);
        assert_eq!(
            &converted,
            "44e4b664c512b08e7577aa3fc7e11ad603e0877a476931fad5aa79e203304aff"
        );

        let encrypted_verifier_hash_input = crypt(
            &true,
            key_cipher_algorithm,
            key_cipher_chaining,
            &verifier_hash_input_key,
            &key_salt_value,
            &verifier_hash_input,
        )
        .unwrap();
        //let converted = encode_hex(&encrypted_verifier_hash_input);
        //assert_eq!(&converted, "2fb9eea58e227ffa549449e941f1199e");

        // verifier_hash_value
        let verifier_hash_value = hash(key_hash_algorithm, vec![&verifier_hash_input]).unwrap();
        //let converted = encode_hex(&verifier_hash_value);
        //assert_eq!(&converted, "920b1de74f38d9cb3ccb3394119ed37e958404fdc47560b1bf647d3c49c22549625fe4a0bd36798bd68a0d98ae64f6ab64a330c9890c62bb740aa492c226ae1f");

        let verifier_hash_value_key = convert_password_to_key(
            password,
            key_hash_algorithm,
            &key_salt_value,
            &key_spin_count,
            &key_key_bits,
            &BLOCK_VERIFIER_HASH_VALUE.to_vec(),
        );
        //let converted = encode_hex(&verifier_hash_value_key);
        //assert_eq!(&converted, "d5515a6062e3e99551b80b92db1fe646483884cdb63e1e7595a9f2cca7532884");

        let encrypted_verifier_hash_value = crypt(
            &true,
            key_cipher_algorithm,
            key_cipher_chaining,
            &verifier_hash_value_key,
            &key_salt_value,
            &verifier_hash_value,
        )
        .unwrap();
        //let converted = encode_hex(&encrypted_verifier_hash_value);
        //assert_eq!(&converted, "0d9c888111b40b630b739c95a5f5b6be67c8f96acdd1bee185bd808b507f652760a2e77f63a6ad0c46f985f2bb8dab4fcf9b86d6a40d9c21299bb4ddf788b250");

        // XML
        let _ = build_encryption_info(
            &package_salt_value,
            &package_block_size,
            &package_key_bits,
            &package_hash_size,
            package_cipher_algorithm,
            package_cipher_chaining,
            package_hash_algorithm,
            &encrypted_hmac_key,
            &encrypted_hmac_value,
            &key_spin_count,
            &key_salt_value,
            &key_block_size,
            &key_key_bits,
            &key_hash_size,
            key_cipher_algorithm,
            key_cipher_chaining,
            key_hash_algorithm,
            &encrypted_verifier_hash_input,
            &encrypted_verifier_hash_value,
            &encrypted_key_value,
        );
    }

    #[test]
    fn test_hash() {
        let package_salt_value = decode_hex("4c251b321d85cecfcb6d952ba6d81846").unwrap();
        let result = hash(
            "SHA512",
            vec![
                &package_salt_value,
                &BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY.to_vec(),
            ],
        )
        .unwrap();
        let converted = encode_hex(&result);
        assert_eq!(&converted, "ba1bf00eed82b07ee65e574eb1f460435d2a1405e81904fd01d5ed5adf43fdcfd8e9aeebad0c08065e0db20cdc8e4552744b61ad1b3cf9a3c5aad5b2a047e76b");
    }

    #[test]
    fn test_buffer_slice() {
        let buffer = decode_hex("ba1bf00eed82b07ee65e574eb1f460435d2a1405e81904fd01d5ed5adf43fdcfd8e9aeebad0c08065e0db20cdc8e4552744b61ad1b3cf9a3c5aad5b2a047e76b").unwrap();
        let start = 0;
        let end = 16;
        let result = buffer_slice(&buffer, start, end);
        let converted = encode_hex(&result);
        assert_eq!(&converted, "ba1bf00eed82b07ee65e574eb1f46043");
    }

    #[test]
    fn test_convert_password_to_key() {
        let key_salt_value = decode_hex("3aa973eec73c98c4710021730ef5b513").unwrap();
        let result = convert_password_to_key(
            "password",
            "SHA512",
            &key_salt_value,
            &100000,
            &256,
            &BLOCK_KEYS_KEY.to_vec(),
        );
        let converted = encode_hex(&result);
        assert_eq!(
            &converted,
            "8d5869311b1c1fdb59a1de6fe1e6f2ce7dccd4deb198a6dfb1f7fb55bc03487d"
        );
    }
}
