use super::driver::*;
use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};

use structs::DefinedName;
use structs::Spreadsheet;
use structs::WorkbookView;
use structs::Worksheet;

const FILE_PATH: &str = "xl/workbook.xml";

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
) -> result::Result<Spreadsheet, XlsxError> {
    let r = io::BufReader::new(arv.by_name(FILE_PATH)?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut spreadsheet = Spreadsheet::default();

    let mut defined_name_value = String::from("");
    let mut is_local_only = false;
    let mut string_value = String::from("");
    let mut defined_names: Vec<DefinedName> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                b"workbookView" => {
                    let mut obj = WorkbookView::default();
                    obj.set_attributes(&mut reader, e);
                    spreadsheet.set_workbook_view(obj);
                }
                b"sheet" => {
                    let name_value = get_attribute(e, b"name").unwrap();
                    let sheet_id_value = get_attribute(e, b"sheetId").unwrap();
                    let r_id_value = get_attribute(e, b"r:id").unwrap();
                    let mut worksheet = Worksheet::default();
                    worksheet.set_name(name_value);
                    worksheet.set_sheet_id(sheet_id_value);
                    worksheet.set_r_id(r_id_value);
                    let _ = spreadsheet.add_sheet(worksheet);
                }
                b"pivotCache" => {
                    let cache_id = get_attribute(e, b"cacheId").unwrap();
                    let r_id = get_attribute(e, b"r:id").unwrap();
                    spreadsheet.add_pivot_caches((r_id, cache_id, String::from("")));
                }
                _ => (),
            },
            Ok(Event::Start(ref e)) => match e.name().into_inner() {
                b"definedName" => {
                    defined_name_value = get_attribute(e, b"name").unwrap();
                    is_local_only = match get_attribute(e, b"localSheetId") {
                        Some(_) => true,
                        None => false,
                    };
                }
                _ => (),
            },
            Ok(Event::Text(e)) => string_value = e.unescape().unwrap().to_string(),
            Ok(Event::End(ref e)) => match e.name().into_inner() {
                b"definedName" => {
                    let mut defined_name = DefinedName::default();
                    defined_name.set_name(defined_name_value);
                    defined_name.set_address(string_value);
                    defined_name.set_is_local_only(is_local_only);
                    defined_names.push(defined_name);

                    defined_name_value = String::from("");
                    string_value = String::from("");
                    is_local_only = false;
                }
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    for sheet in spreadsheet.get_sheet_collection_mut() {
        for defined_name in &defined_names {
            let def_sheet_name = defined_name.get_address_obj().get_sheet_name();
            if sheet.get_name() == def_sheet_name {
                sheet.add_defined_names(defined_name.clone());
            }
        }
    }
    Ok(spreadsheet)
}
