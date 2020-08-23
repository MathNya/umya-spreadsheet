use std::collections::HashMap;
use std::io::Cursor;
use std::result;
use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use tempdir::TempDir;
use super::XlsxError;

use super::super::structs::spreadsheet::Spreadsheet;
use super::driver::*;

const SHARED_STRINGS: &'static str = "xl/sharedStrings.xml";

pub fn write(spreadsheet: &Spreadsheet, dir: &TempDir) -> result::Result<HashMap<String, usize>, XlsxError> {
    let mut shared_strings: Vec<String> = Vec::new();
    let mut count: usize = 0;
    for sheet in spreadsheet.get_sheet_collection() {
        for (_, cell) in sheet.get_cell_collection().get_index() {
            if cell.get_data_type() == "s" {
                count = count + 1;
                let value = cell.get_value();
                if !shared_strings.contains(value) {
                    shared_strings.push(value.clone());
                }
            
            }
        }
    }
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    let _ = writer.write_event(Event::Decl(
        BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_text_node(&mut writer, "\n");
    write_start_tag(&mut writer, "sst", vec![
        ("xmlns", "http://schemas.openxmlformats.org/spreadsheetml/2006/main"),
        ("count", count.to_string().as_str()),
        ("uniqueCount", shared_strings.len().to_string().as_str()),
    ], false);
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut index = 0;
    for st in shared_strings {
         write_start_tag(&mut writer, "si", vec![], false);
         write_start_tag(&mut writer, "t", vec![], false);
         write_text_node(&mut writer, st.clone());
         write_end_tag(&mut writer, "t");
         write_start_tag(&mut writer, "phoneticPr", vec![
             ("fontId", "1"),
         ], true);
         write_end_tag(&mut writer, "si");
         map.insert(st, index);
         index = index + 1;
    }
    write_end_tag(&mut writer, "sst");
    let _ = make_file_from_writer(SHARED_STRINGS, dir, writer, Some("xl"))?;
    Ok(map)
}

