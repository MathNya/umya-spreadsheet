use std::io;

use super::{driver::*, XlsxError};
use crate::structs::{Worksheet, WriterManager};
use quick_xml::{
    events::{BytesDecl, Event},
    Writer,
};

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<Vec<String>, XlsxError> {
    let mut pivot_cache_no_list = Vec::<String>::new();
    for pivot_table in worksheet.get_pivot_tables().iter() {
        let mut writer = Writer::new(io::Cursor::new(Vec::new()));

        // XML header
        writer.write_event(Event::Decl(BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            Some("yes"),
        )));
        write_new_line(&mut writer);

        // Write pivot cache definition
        pivot_table
            .get_pivot_cache_definition()
            .write_to(&mut writer);

        let pivot_cache_no = writer_mng.next_pivot_cache_no();
        writer_mng.add_file_at_pivot_cache(writer, pivot_cache_no)?;
        pivot_cache_no_list.push(pivot_cache_no.to_string());
    }
    Ok(pivot_cache_no_list)
}
