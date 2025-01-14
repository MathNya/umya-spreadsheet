use quick_xml::{Reader, events::Event};

use crate::structs::{PivotTable, PivotTableDefinition, Worksheet, raw::RawFile};

#[allow(dead_code)]
pub(crate) fn read(worksheet: &mut Worksheet, pivot_table_file: &RawFile) {
    let data = std::io::Cursor::new(pivot_table_file.get_file_data());
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
    worksheet.add_pivot_table(pivot_table);
}
