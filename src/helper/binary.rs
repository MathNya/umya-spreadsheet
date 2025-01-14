use std::{fs, path::Path};

use crate::structs::MediaObject;

/// Creates a `MediaObject` from the file at the specified path.
///
/// # Parameters
///
/// - `path`: A reference to a path from which to create the `MediaObject`.
///
/// # Returns
///
/// Returns a `MediaObject` populated with the image data, name, and title
/// extracted from the file at the specified path.
///
/// # Panics
///
/// This function will panic if the file cannot be read, as it calls `unwrap()`
/// on the result of `get_binary_data(path)`. Ensure that the file exists and is
/// readable before calling this function.
///
/// # Example
///
/// ```
/// let media_object = make_media_object("path/to/image.png");
/// ```
#[must_use]
pub fn make_media_object<P: AsRef<Path>>(path: P) -> MediaObject {
    let path = path.as_ref();
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    let title = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("");

    let mut obj = MediaObject::default();
    obj.set_image_data(fs::read(path).unwrap());
    obj.set_image_name(file_name);
    obj.set_image_title(title);
    obj
}
