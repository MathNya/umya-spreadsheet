use std::io;

use super::XlsxError;
use structs::Worksheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<String, XlsxError> {
    let writer = worksheet
        .get_page_setup()
        .get_object_data()
        .as_ref()
        .unwrap();

    let file_no = writer_mng.add_file_at_printer_settings(writer)?;
    Ok(file_no.to_string())
}
