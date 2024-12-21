use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::write_new_line;
use super::XlsxError;
use crate::helper::const_str::PKG_THEME;
use crate::structs::drawing::Theme;
use crate::structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    theme: &Theme,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
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

    // a:theme
    theme.write_to(&mut writer);

    writer_mng.add_writer(PKG_THEME, writer)
}
