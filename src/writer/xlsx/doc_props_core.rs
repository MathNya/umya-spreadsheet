use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::write_new_line;
use super::XlsxError;
use crate::helper::const_str::ARC_CORE;
use crate::structs::Spreadsheet;
use crate::structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
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

    // cp:coreProperties
    spreadsheet.get_properties().write_to_core(&mut writer);

    writer_mng.add_writer(ARC_CORE, writer)
}
