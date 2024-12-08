use super::driver::*;
use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::result;
use crate::structs::raw::RawFile;
use crate::structs::PivotTable;
use crate::structs::PivotTableDefinition;
use crate::structs::Worksheet;

pub(crate) fn read(
    worksheet: &mut Worksheet,
    pivot_table_file: &RawFile,
) -> result::Result<(), XlsxError> {
    let data = std::io::Cursor::new(pivot_table_file.get_file_data());
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(false);
    let mut buf = Vec::new();
    let mut pivot_table = PivotTable::default();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name().into_inner() {
                b"pivotTableDefinition" => {
                    let mut obj = PivotTableDefinition::default();
                    obj.set_attributes(&mut reader, e);
                    pivot_table.set_pivot_table_definition(obj);
                }
                _ => (),
            },
            Ok(Event::End(ref e)) => match e.name().into_inner() {
                b"pivotTableDefinition" => {
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
    worksheet.add_pivot_table(pivot_table);
    Ok(())
}
