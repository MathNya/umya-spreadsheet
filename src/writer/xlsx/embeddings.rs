use std::io;

use structs::Worksheet;
use super::driver::*;
use super::XlsxError;

pub(crate) fn write<W: io::Seek + io::Write>(worksheet: &Worksheet, arv: &mut zip::ZipWriter<W>, sub_dir: &str) -> Result<(), XlsxError> {
    match worksheet.get_ole_objects() {
        Some(v) => {
            for alternate_content in v.get_alternate_content() {
                match alternate_content.get_alternate_content_choice().get_ole_object() {
                    Some(j) => {
                        let file_name = j.get_object_name();
                        let writer = j.get_object_data().as_ref().unwrap();
                        let _ = make_file_from_bin(&file_name, arv, writer, Some(sub_dir)).unwrap();
                    },
                    None => {}
                }
            }
        },
        None => {}
    }    

    Ok(())
}
