use super::driver::*;
use super::XlsxError;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;
use structs::Stylesheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    stylesheet: &Stylesheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    stylesheet.write_to(&mut writer);

    let target = "xl/styles.xml";
    writer_mng.add_writer(target, writer)
}
