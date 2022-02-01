use super::driver::*;
use super::XlsxError;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;
use structs::Stylesheet;

const SUB_DIR: &'static str = "xl";
const FILE_NAME: &'static str = "styles.xml";

pub(crate) fn write<W: io::Seek + io::Write>(
    stylesheet: &Stylesheet,
    arv: &mut zip::ZipWriter<W>,
) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(
        b"1.0",
        Some(b"UTF-8"),
        Some(b"yes"),
    )));
    write_new_line(&mut writer);

    stylesheet.write_to(&mut writer);

    let _ = make_file_from_writer(FILE_NAME, arv, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}
