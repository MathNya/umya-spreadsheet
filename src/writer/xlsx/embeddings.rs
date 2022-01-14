use std::io;

use structs::Worksheet;
use super::driver::*;
use super::XlsxError;

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    arv: &mut zip::ZipWriter<W>,
    sub_dir: &str,
    ole_bin_id: &mut usize,
    ole_excel_id: &mut usize,
) -> Result<(), XlsxError> {
    for ole_object in worksheet.get_ole_objects().get_ole_object() {
        if ole_object.is_bin() {
            let file_name = format!("oleObject{}.bin", ole_bin_id);
            let writer = ole_object.get_object_data().as_ref().unwrap();
            let _ = make_file_from_bin(&file_name, arv, writer, Some(sub_dir)).unwrap();
            *ole_bin_id += 1;
        }
        if ole_object.is_xlsx() {
            let file_name = format!("Microsoft_Excel_Worksheet{}.xlsx", ole_excel_id);
            let writer = ole_object.get_object_data().as_ref().unwrap();
            let _ = make_file_from_bin(&file_name, arv, writer, Some(sub_dir)).unwrap();
            *ole_excel_id += 1;
        }
    }

    Ok(())
}
