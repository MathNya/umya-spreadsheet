use std::io;

use structs::Worksheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> String {
    let writer = worksheet
        .get_page_setup()
        .get_object_data()
        .as_ref()
        .unwrap();

    let file_no = writer_mng.add_file_at_printer_settings(writer);
    file_no.to_string()
}
