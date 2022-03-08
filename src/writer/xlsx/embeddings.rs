use std::io;

use super::XlsxError;
use structs::Worksheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(Vec<String>, Vec<String>), XlsxError> {
    let mut ole_object_no_list: Vec<String> = Vec::new();
    let mut excel_no_list: Vec<String> = Vec::new();
    for ole_object in worksheet.get_ole_objects().get_ole_object() {
        if ole_object.is_bin() {
            let writer = ole_object.get_object_data().as_ref().unwrap();
            let file_no = writer_mng.add_file_at_ole_object(writer)?;
            ole_object_no_list.push(file_no.to_string());
        }
        if ole_object.is_xlsx() {
            let writer = ole_object.get_object_data().as_ref().unwrap();
            let file_no = writer_mng.add_file_at_excel(writer)?;
            excel_no_list.push(file_no.to_string());
        }
    }

    Ok((ole_object_no_list, excel_no_list))
}
