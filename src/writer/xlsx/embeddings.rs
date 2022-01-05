use std::io;

use structs::Worksheet;
use super::driver::*;
use super::XlsxError;

pub(crate) fn write<W: io::Seek + io::Write>(worksheet: &Worksheet, arv: &mut zip::ZipWriter<W>, sub_dir: &str) -> Result<(), XlsxError> {
    for ole_object in worksheet.get_ole_objects().get_ole_object() {
        let file_name = ole_object.get_object_name();
        let writer = ole_object.get_object_data().as_ref().unwrap();
        let _ = make_file_from_bin(&file_name, arv, writer, Some(sub_dir)).unwrap();
    }

    Ok(())
}
