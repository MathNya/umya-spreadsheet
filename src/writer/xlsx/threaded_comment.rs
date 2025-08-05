use super::driver::*;
use super::XlsxError;
use crate::helper::const_str::*;
use crate::structs::Worksheet;
use crate::structs::WriterManager;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::collections::HashSet;
use std::io;

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<String, XlsxError> {
    if !worksheet.has_threaded_comments() {
        return Ok(String::new());
    }

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    // comments
    write_start_tag(
        &mut writer,
        "ThreadedComments",
        vec![("xmlns", THREADED_COMMENTS_NS), (" xmlns:x", SHEET_MAIN_NS)],
        false,
    );

    for threaded_comment in worksheet.get_threaded_comments() {
        // threaded comment
        threaded_comment.write_to(&mut writer);
    }
    write_end_tag(&mut writer, "ThreadedComments");

    let file_no = writer_mng.add_file_at_threaded_comment(writer)?;
    Ok(file_no.to_string())
}
