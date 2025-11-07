use super::XlsxError;
use crate::structs::PivotCacheDefinition;
use crate::structs::Spreadsheet;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io;
use std::result;

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
    target: &str,
    cache_id: &str,
) -> result::Result<(), XlsxError> {
    let r = io::BufReader::new(arv.by_name(&format!("xl/{}", target))?);
    let mut reader = Reader::from_reader(r);
    reader.config_mut().trim_text(false);
    let mut buf = Vec::new();
    let mut pivot_cache_def = PivotCacheDefinition::default();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                b"pivotTableDefinition" | b"pivotCacheDefinition" => {
                    pivot_cache_def.set_attributes(&mut reader, e);
                }
                _ => (),
            },
            Ok(Event::End(ref e)) => match e.name().into_inner() {
                b"pivotTableDefinition" | b"pivotCacheDefinition" => {
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
        for sheet_index in 0..spreadsheet.get_sheet_count() {
            if let Some(sheet) = spreadsheet.get_sheet_mut(&sheet_index) {
                for pivot_table in sheet.get_pivot_tables_mut() {
                    if *pivot_table.get_pivot_table_definition().get_cache_id() == cache_id_num {
                        pivot_table.set_pivot_cache_definition(pivot_cache_def.clone());
                    }
                }
            }
        }
    }

    Ok(())
}
