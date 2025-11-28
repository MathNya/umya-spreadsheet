use quick_xml::{
    Reader,
    events::Event,
};

use crate::{
    PivotTable,
    structs::{
        PivotCacheDefinition,
        raw::RawFile,
    },
};

pub(crate) fn read(raw_file: &RawFile, pivot_table: &mut PivotTable) {
    let data = std::io::Cursor::new(raw_file.file_data());
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(false);
    let mut buf = Vec::new();
    let mut pivot_cache_def = PivotCacheDefinition::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name().into_inner() == b"pivotCacheDefinition" {
                    pivot_cache_def.set_attributes(&mut reader, e);
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name().into_inner() == b"pivotCacheDefinition" {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    pivot_table.set_pivot_cache_definition(pivot_cache_def);
}
