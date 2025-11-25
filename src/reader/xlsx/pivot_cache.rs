use std::io;

use quick_xml::{
    Reader,
    events::Event,
};

use super::XlsxError;
use crate::structs::{
    PivotCacheDefinition,
    Workbook,
};

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    wb: &mut Workbook,
    target: &str,
    cache_id: &str,
) -> Result<(), XlsxError> {
    let r = io::BufReader::new(arv.by_name(&format!("xl/{target}"))?);
    let mut reader = Reader::from_reader(r);
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

    // Associate the cache definition with pivot tables that use this cache_id
    if let Ok(cache_id_num) = cache_id.parse::<u32>() {
        for sheet_index in 0..wb.sheet_count() {
            if let Ok(sheet) = wb.sheet_mut(sheet_index) {
                for pivot_table in sheet.pivot_tables_mut() {
                    if pivot_table.pivot_table_definition().cache_id() == cache_id_num {
                        pivot_table.set_pivot_cache_definition(pivot_cache_def.clone());
                    }
                }
            }
        }
    }

    Ok(())
}
