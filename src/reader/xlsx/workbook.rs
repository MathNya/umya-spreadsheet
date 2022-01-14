use std::{io, result};
use quick_xml::Reader;
use quick_xml::events::{Event};
use super::XlsxError;
use super::driver::*;

use ::structs::Spreadsheet;
use ::structs::DefinedName;
use ::structs::WorkbookView;

const FILE_PATH: &'static str = "xl/workbook.xml";

pub(crate) fn read<R: io::Read + io::Seek>(arv: &mut zip::read::ZipArchive<R>) -> result::Result<(Spreadsheet, Vec<(String, String, String)>, Vec<DefinedName>), XlsxError>
{    
    let r = io::BufReader::new(arv.by_name(FILE_PATH)?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut spreadsheet = Spreadsheet::default();
    let mut sheets: Vec<(String, String, String)> = Vec::new();

    let mut defined_name_value = String::from("");
    let mut is_local_only = false;
    let mut string_value = String::from("");
    let mut defined_names: Vec<DefinedName> = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"workbookView" => {
                        let mut obj = WorkbookView::default();
                        obj.set_attributes(&mut reader, e);
                        spreadsheet.set_workbook_view(obj);
                    },
                    b"sheet" => {
                        let name_value =  get_attribute(e, b"name").unwrap();
                        let sheet_id_value =  get_attribute(e, b"sheetId").unwrap();
                        let r_id_value =  get_attribute(e, b"r:id").unwrap();
                        sheets.push((name_value, sheet_id_value, r_id_value));
                    },
                    _ => (),
                }
            },
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"definedName" => {
                        defined_name_value =  get_attribute(e, b"name").unwrap();
                        is_local_only = match get_attribute(e, b"localSheetId") { Some(_) => true, None => false};
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => string_value = e.unescape_and_decode(&reader).unwrap(),
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"definedName" => {
                        let mut defined_name = DefinedName::default();
                        defined_name.set_name(defined_name_value);
                        defined_name.set_address(string_value);
                        defined_name.set_is_local_only(is_local_only);
                        defined_names.push(defined_name);
                        
                        defined_name_value = String::from("");
                        string_value = String::from("");
                        is_local_only = false;
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    Ok((spreadsheet, sheets, defined_names))
}
