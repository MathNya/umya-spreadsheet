use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use super::XlsxError;
use structs::Worksheet;

const SUB_DIR: &'static str = "xl/drawings";

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    drawing_id: &usize,
    arv: &mut zip::ZipWriter<W>,
) -> Result<(), XlsxError> {
    if worksheet.has_drawing_object() == false {
        return Ok(());
    }

    let file_name = format!("drawing{}.xml", drawing_id);

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(
        b"1.0",
        Some(b"UTF-8"),
        Some(b"yes"),
    )));
    write_new_line(&mut writer);

    worksheet.get_worksheet_drawing().write_to(
        &mut writer,
        drawing_id,
        worksheet.get_ole_objects(),
    );

    let _ = make_file_from_writer(&file_name, arv, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}
