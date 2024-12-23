use sha2::{
    Digest,
    Sha512,
};

macro_rules! generate_random_bytes {
    ($var_name:ident, $size:expr) => {
        let mut $var_name = [0u8; $size];
        rand::thread_rng().fill(&mut $var_name[..]);
    };
}

pub(crate) fn buffer_concat(buffers: &[&[u8]]) -> Vec<u8> {
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

/// Takes a slice of byte slices and computes their SHA-512 hash.
/// Concatenates all input buffers and processes them sequentially.
/// Returns the resulting hash as a vector of bytes.
pub(crate) fn hash_concatenated(buffers: &[&[u8]]) -> Vec<u8> {
    let mut hasher = Sha512::new();

    for buffer in buffers {
        hasher.update(buffer);
    }

    hasher.finalize().to_vec()
}

pub(crate) use generate_random_bytes;
