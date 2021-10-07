use std::io::Cursor;
use std::result;
use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use tempdir::TempDir;
use super::XlsxError;
use ::structs::SharedStringTable;
use super::driver::*;

const SHARED_STRINGS: &'static str = "xl/sharedStrings.xml";

pub(crate) fn write(shared_string_table: &SharedStringTable, dir: &TempDir) -> result::Result<(), XlsxError> {

    if shared_string_table.get_shared_string_item().len() == 0 {
        return Ok(())
    }
    
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    shared_string_table.write_to(&mut writer);

    let _ = make_file_from_writer(SHARED_STRINGS, dir, writer, Some("xl"))?;
    Ok(())
}
