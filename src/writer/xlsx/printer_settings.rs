use std::io;

use super::driver::*;
use super::XlsxError;
use structs::Worksheet;

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    arv: &mut zip::ZipWriter<W>,
    sub_dir: &str,
    id: &mut usize,
) -> Result<(), XlsxError> {
    let file_name = format!("printerSettings{}.bin", id);
    let writer = worksheet
        .get_page_setup()
        .get_object_data()
        .as_ref()
        .unwrap();
    let _ = make_file_from_bin(&file_name, arv, writer, Some(sub_dir)).unwrap();

    Ok(())
}
