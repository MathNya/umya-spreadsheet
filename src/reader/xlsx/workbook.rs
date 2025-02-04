use std::io;

use quick_xml::{Reader, escape, events::Event};

use super::{XlsxError, driver::get_attribute};
use crate::{
    helper::const_str::PKG_WORKBOOK,
    structs::{DefinedName, Workbook, WorkbookProtection, WorkbookView, Worksheet},
    xml_read_loop,
};

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
) -> Result<Workbook, XlsxError> {
    let r = io::BufReader::new(arv.by_name(PKG_WORKBOOK)?);
    let mut reader = Reader::from_reader(r);
    reader.config_mut().trim_text(true);
    let mut wb = Workbook::default();

    let mut defined_names: Vec<DefinedName> = Vec::new();

    xml_read_loop!(
        reader,
        Event::Empty(ref e) => {
            match e.name().into_inner() {
                b"workbookView" => {
                    let mut obj = WorkbookView::default();
                    obj.set_attributes(&mut reader, e);
                    wb.set_workbook_view(obj);
                }
                b"workbookProtection" => {
                    let mut obj = WorkbookProtection::default();
                    obj.set_attributes(&mut reader, e);
                    wb.set_workbook_protection(obj);
                }
                b"sheet" => {
                    let name_value = get_attribute(e, b"name").unwrap();
                    let sheet_id_value = get_attribute(e, b"sheetId").unwrap();
                    let r_id_value = get_attribute(e, b"r:id").unwrap();
                    let state = get_attribute(e, b"state");
                    let mut worksheet = Worksheet::default();
                    worksheet.set_name(escape::unescape(&name_value).unwrap());
                    worksheet.set_sheet_id(sheet_id_value);
                    worksheet.set_r_id(r_id_value);
                    if let Some(v) = state {
                        worksheet.set_state_str(&v);
                    }
                    wb.add_sheet(worksheet).unwrap();
                }
                b"pivotCache" => {
                    let cache_id = get_attribute(e, b"cacheId").unwrap();
                    let r_id = get_attribute(e, b"r:id").unwrap();
                    wb.add_pivot_caches((r_id, cache_id, String::new()));
                }
                _ => (),
            }
        },
        Event::Start(ref e) => {
            if e.name().into_inner() == b"definedName" {
                let mut obj = DefinedName::default();
                obj.set_attributes(&mut reader, e);
                defined_names.push(obj);
            }
        },
        Event::Eof => break
    );

    for defined_name in &defined_names {
        if defined_name.has_local_sheet_id() {
            let local_sheet_id = defined_name.local_sheet_id() as usize;
            wb.sheet_mut(local_sheet_id)
                .unwrap()
                .add_defined_names(defined_name.clone());
        } else {
            if let Some(v) = defined_name.address_obj().first() {
                if let Some(s) = wb.sheet_by_name_mut(v.sheet_name()) {
                    s.add_defined_names(defined_name.clone());
                    continue;
                }
            }
            wb.add_defined_names(defined_name.clone());
        }
    }

    Ok(wb)
}
