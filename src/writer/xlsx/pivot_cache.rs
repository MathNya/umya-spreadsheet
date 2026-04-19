use std::io;

use quick_xml::{
    Writer,
    events::{
        BytesDecl,
        Event,
    },
};

use super::driver::write_new_line;
use crate::structs::{
    Worksheet,
    WriterManager,
};

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Vec<String> {
    let mut pivot_cache_no_list = Vec::<String>::new();
    for pivot_table in worksheet.pivot_tables() {
        let (find, no) = writer_mng
            .get_pivot_cache_no(pivot_table.pivot_cache_definition().hash_code().as_str());
        if find {
            continue;
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

        // Write pivot cache definition
        pivot_table.pivot_cache_definition().write_to(&mut writer);

        pivot_cache_no_list.push(no.to_string());
    }
    pivot_cache_no_list
}
