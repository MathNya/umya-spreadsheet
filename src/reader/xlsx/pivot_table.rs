use quick_xml::{
    Reader,
    events::Event,
};

use crate::{
    helper::const_str::{
        PIVOT_CACHE_DEF_NS,
        PIVOT_CACHE_REC_NS,
    },
    raw::RawRelationships,
    reader::xlsx::pivot_cache,
    structs::{
        PivotTable,
        PivotTableDefinition,
        Worksheet,
        raw::RawFile,
    },
};

#[allow(clippy::redundant_closure_for_method_calls)]
pub(crate) fn read(
    worksheet: &mut Worksheet,
    pivot_table_file: &RawFile,
    pivot_table_relationships: Option<&RawRelationships>,
    pivot_cache_relationships: Option<&RawRelationships>,
) {
    let data = std::io::Cursor::new(pivot_table_file.file_data());
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(false);
    let mut buf = Vec::new();
    let mut pivot_table = PivotTable::default();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name().into_inner() == b"pivotTableDefinition" {
                    let mut obj = PivotTableDefinition::default();
                    obj.set_attributes(&mut reader, e);
                    pivot_table.set_pivot_table_definition(obj);
                }
            }
            Ok(Event::End(ref e)) => {
                if e.name().into_inner() == b"pivotTableDefinition" {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    if let Some(rrs_list) = pivot_table_relationships {
        // Find cache records from the pivot cache definition's own relationships
        let records_file = pivot_cache_relationships.and_then(|cache_rels| {
            cache_rels
                .relationship_list()
                .iter()
                .find(|r| r.get_type() == PIVOT_CACHE_REC_NS)
                .map(|r| r.raw_file())
        });

        pivot_cache::read(
            rrs_list.relationship_by_type(PIVOT_CACHE_DEF_NS).raw_file(),
            &mut pivot_table,
            records_file,
        );
    }

    worksheet.add_pivot_table(pivot_table);
}
