use crate::xml_read_loop;

use super::driver::*;
use super::XlsxError;
use quick_xml::escape;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};

use helper::const_str::*;
use structs::DefinedName;
use structs::Spreadsheet;
use structs::WorkbookProtection;
use structs::WorkbookView;
use structs::Worksheet;

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
) -> result::Result<Spreadsheet, XlsxError> {
    let r = io::BufReader::new(arv.by_name(PKG_WORKBOOK)?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);
    let mut spreadsheet = Spreadsheet::default();

    let mut defined_name_value = String::from("");
    let mut is_local_only = false;
    let mut string_value = String::from("");
    let mut defined_names: Vec<DefinedName> = Vec::new();

    xml_read_loop!(
        reader,
        Event::Empty(ref e) => {
            match e.name().into_inner() {
                b"workbookView" => {
                    let mut obj = WorkbookView::default();
                    obj.set_attributes(&mut reader, e);
                    spreadsheet.set_workbook_view(obj);
                }
                b"workbookProtection" => {
                    let mut obj = WorkbookProtection::default();
                    obj.set_attributes(&mut reader, e);
                    spreadsheet.set_workbook_protection(obj);
                }
                b"sheet" => {
                    let name_value = get_attribute(e, b"name").unwrap();
                    let sheet_id_value = get_attribute(e, b"sheetId").unwrap();
                    let r_id_value = get_attribute(e, b"r:id").unwrap();
                    let mut worksheet = Worksheet::default();
                    worksheet.set_name(escape::unescape(&name_value).unwrap());
                    worksheet.set_sheet_id(sheet_id_value);
                    worksheet.set_r_id(r_id_value);
                    spreadsheet.add_sheet(worksheet);
                }
                b"pivotCache" => {
                    let cache_id = get_attribute(e, b"cacheId").unwrap();
                    let r_id = get_attribute(e, b"r:id").unwrap();
                    spreadsheet.add_pivot_caches((r_id, cache_id, String::from("")));
                }
                _ => (),
            }
        },
        Event::Start(ref e) => {
            if e.name().into_inner() == b"definedName" {
                defined_name_value = get_attribute(e, b"name").unwrap();
                is_local_only = get_attribute(e, b"localSheetId").is_some();
            }
        },
        Event::Text(e) => string_value = e.unescape().unwrap().to_string(),
        Event::End(ref e) => {
            if e.name().into_inner() == b"definedName" {
                let mut defined_name = DefinedName::default();
                defined_name.set_name(defined_name_value);
                defined_name.set_address(string_value);
                defined_name.set_is_local_only(is_local_only);
                defined_names.push(defined_name);

                defined_name_value = String::from("");
                string_value = String::from("");
                is_local_only = false;
            }
        },
        Event::Eof => break
    );

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
