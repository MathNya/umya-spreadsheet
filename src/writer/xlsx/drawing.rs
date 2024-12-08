use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use super::XlsxError;
use crate::structs::Worksheet;
use crate::structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(String, Vec<(String, String)>), XlsxError> {
    let mut rel_list: Vec<(String, String)> = Vec::new();

    if !worksheet.has_drawing_object() {
        return Ok((String::from(""), rel_list));
    }

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    worksheet.get_worksheet_drawing().write_to(
        &mut writer,
        worksheet.get_ole_objects(),
        &mut rel_list,
    );

    let file_no = writer_mng.add_file_at_drawing(writer)?;
    Ok((file_no.to_string(), rel_list))
}
