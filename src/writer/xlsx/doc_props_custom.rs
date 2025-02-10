use std::io;

use quick_xml::{
    Writer,
    events::{BytesDecl, Event},
};

use super::{XlsxError, driver::write_new_line};
use crate::{
    helper::const_str::ARC_CUSTOM,
    structs::{Workbook, WriterManager},
};

pub(crate) fn write<W: io::Seek + io::Write>(
    wb: &Workbook,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    if wb
        .properties()
        .get_custom_properties()
        .custom_document_property_list()
        .is_empty()
    {
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

    // Properties
    wb.properties().write_to_custom(&mut writer);

    writer_mng.add_writer(ARC_CUSTOM, writer)
}
