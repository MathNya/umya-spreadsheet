use tempdir::TempDir;

use structs::drawing::spreadsheet::Picture;
use super::driver::*;
use super::XlsxError;

pub(crate) fn write(picture: &Picture, dir: &TempDir, sub_dir: &str) -> Result<(), XlsxError> {
    let file_name = picture.get_blip_fill().get_blip().get_image_name();
    let writer = picture.get_blip_fill().get_blip().get_image_data().as_ref().unwrap();
    let _ = make_file_from_bin(format!("{}/{}",sub_dir,file_name).as_str(), dir, writer, Some(sub_dir)).unwrap();
    Ok(())
}
