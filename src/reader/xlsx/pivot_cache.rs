use quick_xml::{
    Reader,
    events::Event,
};

use crate::structs::{
    PivotCacheDefinition,
    Worksheet,
    raw::RawFile,
};

pub(crate) fn read(worksheet: &mut Worksheet, pivot_cache_file: &RawFile) {
    let data = std::io::Cursor::new(pivot_cache_file.file_data());
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(false);
    let mut buf = Vec::new();
    let mut pivot_cache_def = PivotCacheDefinition::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e) | Event::Empty(ref e)) => match e.name().into_inner() {
                // Support both element names for backwards compatibility with incorrectly written
                // files
                b"pivotCacheDefinition" | b"pivotTableDefinition" => {
                    pivot_cache_def.set_attributes(&mut reader, e);
                }
                _ => (),
            },
            Ok(Event::End(ref e)) => match e.name().into_inner() {
                b"pivotCacheDefinition" | b"pivotTableDefinition" => {
                    break;
                }
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    dbg!(pivot_cache_def.id());

    // Associate the cache definition with pivot tables that use this cache_id
    for pivot_table in worksheet.pivot_tables_mut() {
        if pivot_table.pivot_table_definition().cache_id() == 1 {
            pivot_table.set_pivot_cache_definition(pivot_cache_def.clone());
        }
    }
}
