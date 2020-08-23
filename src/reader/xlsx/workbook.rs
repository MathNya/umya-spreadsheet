use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use super::driver::*;

use super::super::structs::spreadsheet::Spreadsheet;
use super::super::structs::defined_name::DefinedName;

const FILE_PATH: &'static str = "xl/workbook.xml";

pub(crate) fn read(dir: &TempDir) -> result::Result<(Spreadsheet, Vec<(String, String, String)>), XlsxError>
{
    let path = dir.path().join(FILE_PATH);
    dbg!(path.clone());
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut spreadsheet = Spreadsheet::default();
    let mut sheets: Vec<(String, String, String)> = Vec::new();

    let mut defined_name = DefinedName::default();
    let mut string_value = String::from("");

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"sheet" => {
                        let name_value =  get_attribute(e, b"name").unwrap();
                        let sheet_id_value =  get_attribute(e, b"sheetId").unwrap();
                        let r_id_value =  get_attribute(e, b"r:id").unwrap();
                        sheets.push((name_value, sheet_id_value, r_id_value));
                    },
                    b"definedName" => {
                        let name_value =  get_attribute(e, b"name").unwrap();
                        defined_name.set_name(name_value);
                    },
                    _ => (),
                }
            }
            Ok(Event::Text(e)) => string_value = e.unescape_and_decode(&reader).unwrap(),
            Ok(Event::End(ref e)) => {
                defined_name.set_value(string_value);
                spreadsheet.add_defined_names(defined_name);
                string_value = String::from("");
                defined_name = DefinedName::default();
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    Ok((spreadsheet,sheets))
}