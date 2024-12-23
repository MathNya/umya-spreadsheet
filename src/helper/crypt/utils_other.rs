use sha2::{
    Digest,
    Sha512,
};

/// A macro that generates an array of random bytes.
///
/// This macro initializes a variable with the specified name and fills it
/// with random bytes of the given size using the `rand` crate.
///
/// # Parameters
///
/// - `$var_name`: The name of the variable to hold the generated random bytes.
/// - `$size`: The size of the byte array to be generated.
///
/// # Example
///
/// ```
/// generate_random_bytes!(random_bytes, 16);
/// // `random_bytes` now contains 16 random bytes.
/// ```
macro_rules! generate_random_bytes {
    ($var_name:ident, $size:expr) => {
        let mut $var_name = [0u8; $size];
        rand::thread_rng().fill(&mut $var_name[..]);
    };
}

/// Concatenates multiple byte slices into a single `Vec<u8>`.
///
/// This function takes a slice of byte slices and computes the total length
/// of the resulting vector. It preallocates the vector with the total length
/// and extends it with each input buffer.
///
/// # Parameters
///
/// - `buffers`: A slice of byte slices to be concatenated.
///
/// # Returns
///
/// A `Vec<u8>` containing all the concatenated byte slices.
///
/// # Example
///
/// ```
/// let buffer1 = b"Hello, ";
/// let buffer2 = b"world!";
/// let result = buffer_concat(&[buffer1, buffer2]);
/// assert_eq!(result, b"Hello, world!");
/// ```
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

/// Computes the SHA-512 hash of concatenated byte slices.
///
/// This function takes a slice of byte slices, concatenates them, and computes
/// their SHA-512 hash. The resulting hash is returned as a vector of bytes.
///
/// # Parameters
///
/// - `buffers`: A slice of byte slices to be hashed.
///
/// # Returns
///
/// A `Vec<u8>` containing the SHA-512 hash of the concatenated input buffers.
///
/// # Example
///
/// ```
/// let buffer1 = b"Hello, ";
/// let buffer2 = b"world!";
/// let hash = hash_concatenated(&[buffer1, buffer2]);
/// // `hash` now contains the SHA-512 hash of "Hello, world!".
/// ```
pub(crate) fn hash_concatenated(buffers: &[&[u8]]) -> Vec<u8> {
    let mut hasher = Sha512::new();

    for buffer in buffers {
        hasher.update(buffer);
    }

    hasher.finalize().to_vec()
}

/// Re-exports the `generate_random_bytes` macro for use in other modules.
pub(crate) use generate_random_bytes;
