use std::io;

use structs::Worksheet;
use super::driver::*;
use super::XlsxError;

pub(crate) fn write<W: io::Seek + io::Write>(worksheet: &Worksheet, arv: &mut zip::ZipWriter<W>, sub_dir: &str) -> Result<(), XlsxError> {
    for picture in worksheet.get_worksheet_drawing().get_picture_collection(){
        let file_name = picture.get_blip_fill().get_blip().get_image_name();
        let writer = picture.get_blip_fill().get_blip().get_image_data().as_ref().unwrap();
        let _ = make_file_from_bin(&file_name, arv, writer, Some(sub_dir)).unwrap();
    }
    for get_ole_object in worksheet.get_ole_objects().get_ole_object() {
        let file_name = get_ole_object.get_embedded_object_properties().get_image_name();
        let writer = get_ole_object.get_embedded_object_properties().get_image_data().as_ref().unwrap();
        let _ = make_file_from_bin(&file_name, arv, writer, Some(sub_dir)).unwrap();
    }    

    Ok(())
}
