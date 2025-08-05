use std::io;

use quick_xml::{
    Writer,
    events::{
        BytesDecl,
        Event,
    },
};

use super::{
    XlsxError,
    driver::{
        write_end_tag,
        write_new_line,
        write_start_tag,
    },
};
use crate::{
    helper::const_str::{
        SHEET_MAIN_NS,
        THREADED_COMMENTS_NS,
    },
    structs::{
        Worksheet,
        WriterManager,
    },
};

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<String, XlsxError> {
    if !worksheet.has_threaded_comments() {
        return Ok(String::new());
    }

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer
        .write_event(Event::Decl(BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            Some("yes"),
        )))
        .unwrap();
    write_new_line(&mut writer);

    // comments
    write_start_tag(
        &mut writer,
        "ThreadedComments",
        vec![
            ("xmlns", THREADED_COMMENTS_NS).into(),
            (" xmlns:x", SHEET_MAIN_NS).into(),
        ],
        false,
    );

    for threaded_comment in worksheet.threaded_comments() {
        // threaded comment
        threaded_comment.write_to(&mut writer);
    }
    write_end_tag(&mut writer, "ThreadedComments");

    let file_no = writer_mng.add_file_at_threaded_comment(writer)?;
    Ok(file_no.to_string())
}
