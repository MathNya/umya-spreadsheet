use super::driver::*;
use super::XlsxError;
use helper::const_str::*;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;
use std::result;
use std::sync::Arc;
use std::sync::RwLock;
use structs::SharedStringTable;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    shared_string_table: Arc<RwLock<SharedStringTable>>,
    writer_mng: &mut WriterManager<W>,
) -> result::Result<(), XlsxError> {
    if shared_string_table
        .read()
        .unwrap()
        .get_shared_string_item()
        .is_empty()
    {
        return Ok(());
    }

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    shared_string_table.write().unwrap().write_to(&mut writer);

    writer_mng.add_writer(PKG_SHARED_STRINGS, writer)
}
