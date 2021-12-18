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
    match worksheet.get_ole_objects() {
        Some(v) => {
            for alternate_content in v.get_alternate_content() {
                match alternate_content.get_alternate_content_choice().get_ole_object() {
                    Some(j) => {
                        match j.get_embedded_object_properties() {
                            Some(a) => {
                                let file_name = a.get_image_name();
                                let writer = a.get_image_data().as_ref().unwrap();
                                let _ = make_file_from_bin(&file_name, arv, writer, Some(sub_dir)).unwrap();
                            },
                            None => {}
                        }
                    },
                    None => {}
                }
            }
        },
        None => {}
    }    

    Ok(())
}
