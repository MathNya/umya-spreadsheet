use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;
use structs::MediaObject;

#[inline]
pub fn get_binary_data(path: &str) -> Vec<u8> {
    let path = std::path::Path::new(path);
    let mut buf = Vec::new();

    let file = File::open(path).unwrap();
    BufReader::new(file).read_to_end(&mut buf).unwrap();
    return buf;
}

#[inline]
pub fn make_media_object(path: &str) -> MediaObject {
    let name = path.split("/").last().unwrap();
    let mut obj = MediaObject::default();
    obj.set_image_data(get_binary_data(path));
    obj.set_image_name(name);
    obj.set_image_title(name.split(".").next().unwrap_or(""));
    obj
}
