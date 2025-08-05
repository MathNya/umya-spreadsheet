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
        PKG_PERSON,
        SHEET_MAIN_NS,
        THREADED_COMMENTS_NS,
    },
    structs::{
        Workbook,
        WriterManager,
    },
};

pub(crate) fn write<W: io::Seek + io::Write>(
    wb: &Workbook,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    if !wb.has_threaded_comments() {
        return Ok(());
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
        "personList",
        vec![
            ("xmlns", THREADED_COMMENTS_NS).into(),
            (" xmlns:x", SHEET_MAIN_NS).into(),
        ],
        false,
    );

    write_end_tag(&mut writer, "personList");
    writer_mng.add_writer(PKG_PERSON, writer)
}
