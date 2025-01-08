/// The version of the crate, as specified in the Cargo.toml file.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(umya_spreadsheet::VERSION, "3.0.0");
/// ```
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The major version of the crate, as specified in the Cargo.toml file.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(umya_spreadsheet::VERSION_MAJOR, "3");
/// ```
pub const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");

/// The minor version of the crate, as specified in the Cargo.toml file.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(umya_spreadsheet::VERSION_MINOR, "0");
/// ```
pub const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");

/// The patch version of the crate, as specified in the Cargo.toml file.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(umya_spreadsheet::VERSION_PATCH, "0");
/// ```
pub const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

/// The pre-release flag of the crate, if any, as specified in the Cargo.toml
/// file.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(umya_spreadsheet::VERSION_PRE, "alpha1");
/// ```
pub const VERSION_PRE: &str = env!("CARGO_PKG_VERSION_PRE");
