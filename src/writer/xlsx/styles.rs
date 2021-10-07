use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;
use ::structs::Stylesheet;
use super::driver::*;
use super::XlsxError;

const SUB_DIR: &'static str = "xl";
const FILE_NAME: &'static str = "styles.xml";

pub(crate) fn write(stylesheet: &Stylesheet, dir: &TempDir) -> Result<(), XlsxError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    stylesheet.write_to(&mut writer);

    let _ = make_file_from_writer(format!("{}/{}", SUB_DIR, FILE_NAME).as_str(), dir, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}
