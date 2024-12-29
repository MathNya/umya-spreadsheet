#![allow(unused_imports)]

/// A macro that implements the `From` trait for converting from one error type
/// to another.
///
/// # Usage
/// ```
/// use std::io;
///
/// use my_error::MyErrorKind;
///
/// from_err!(io::Error, MyError, Io);
///
/// let io_err = io::Error::new(io::ErrorKind::Other, "An I/O error occurred");
/// let my_err: MyError = io_err.into();
///
/// assert_eq!(my_err.kind(), MyErrorKind::Io);
/// ```
#[macro_export]
macro_rules! from_err {
    ($from:ty, $to:tt, $var:tt) => {
        impl From<$from> for $to {
            fn from(e: $from) -> $to {
                $to::$var(e)
            }
        }
    };
}

/// Asserts that the SHA-256 hash of a given input matches the expected
/// hexadecimal string.
///
/// # Arguments
///
/// * `$input` - The input data to hash.
/// * `$expected_hex` - The expected SHA-256 hash as a hexadecimal string.
///
/// # Panics
///
/// This macro will panic if the actual SHA-256 hash does not match the
/// expected hash.
///
/// # Examples
///
/// ```ignore
/// let data = b"Hello, world!";
/// assert_sha256!(
///     data,
///     "a591a6d40bf420404a011733cfb7b190d62c65bf0bcda32b57b277d9ad9f146e"
/// );
/// // This will not panic
///
/// assert_sha256!(data, "invalid_hash");
/// // This will panic with a message indicating the mismatch
/// ```
macro_rules! assert_sha256 {
    ($input:expr, $expected_hex:expr) => {{
        let hash = Sha256::digest($input).to_vec();
        let expected_bytes = hex_literal::hex!($expected_hex);
        assert_eq!(
            &hash,
            &expected_bytes,
            "SHA256({}) mismatch! Expected: {:?}, Actual: {:?}",
            stringify!($input),
            &expected_bytes
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>(),
            &hash
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        );
    }};
}

/// A macro that compiles a regular expression and caches it.
///
/// # Usage
/// ```
/// let re = compile_regex!(r"^\d+$");
///
/// assert!(re.is_match("123").unwrap());
/// assert!(!re.is_match("abc").unwrap());
/// ```
macro_rules! compile_regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<fancy_regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| fancy_regex::Regex::new($re).unwrap())
    }};
}

/// Prints a byte slice as a hex string, prefixed with the variable name.
///
/// This macro takes a reference to a byte slice (`&[u8]`) and prints both
/// the variable name and its hexadecimal representation to stdout.
macro_rules! print_hex {
    ($var:expr) => {
        println!(
            "{} = {}",
            stringify!($var),
            $var.iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        );
    };
}

/// Prints the SHA-256 hash of a given input as a hexadecimal string.
///
/// # Examples
///
/// ```ignore
/// let data = b"Hello, world!";
/// print_sha256_hex!(data);
/// // Output: SHA256(data) = 5eb63bbbe01eeed093cb22bb8f5acdc3
/// ```
macro_rules! print_sha256_hex {
    ($var:expr) => {
        let hash = Sha256::digest($var);
        println!(
            "SHA256({}) = {}",
            stringify!($var),
            hash.iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        );
    };
}

pub(crate) use assert_sha256;
pub(crate) use compile_regex;
pub(crate) use print_hex;
pub(crate) use print_sha256_hex;
