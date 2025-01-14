use std::io;

use quick_xml::{
    Writer,
    events::{BytesDecl, Event},
};

use super::{XlsxError, driver::write_new_line};
use crate::{
    helper::const_str::PKG_STYLES,
    structs::{Stylesheet, WriterManager},
};

pub(crate) fn write<W: io::Seek + io::Write>(
    stylesheet: &Stylesheet,
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

    stylesheet.write_to(&mut writer);

    writer_mng.add_writer(PKG_STYLES, writer)
}
