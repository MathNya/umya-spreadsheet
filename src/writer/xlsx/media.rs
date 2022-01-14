use std::io;

use structs::Image;
use super::driver::*;
use super::XlsxError;

pub(crate) fn write<W: io::Seek + io::Write>(images: Vec<&Image>, arv: &mut zip::ZipWriter<W>, sub_dir: &str) -> Result<(), XlsxError> {
    for image in images {
        let file_name = image.get_image_name();
        let writer = image.get_image_data().as_ref();
        let _ = make_file_from_bin(file_name, arv, writer, Some(sub_dir)).unwrap();
    }

    Ok(())
}
