use std::io;

use quick_xml::{
    Reader,
    events::Event,
};

use super::{
    XlsxError,
    driver::get_attribute,
};
use crate::{
    helper::const_str::PKG_WORKBOOK,
    structs::{
        DefinedName,
        Workbook,
        WorkbookProtection,
        WorkbookView,
        Worksheet,
    },
    xml_read_loop,
};

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
) -> Result<Workbook, XlsxError> {
    let r = io::BufReader::new(super::driver::zip_by_name(arv, PKG_WORKBOOK)?);
    let mut reader = Reader::from_reader(r);
    reader.config_mut().trim_text(true);
    let mut wb = Workbook::default();

    let mut defined_names: Vec<DefinedName> = Vec::new();

    xml_read_loop!(
        reader,
        ref n @ (Event::Empty(ref e) | Event::Start(ref e)) => {
            let is_empty = matches!(n, Event::Empty(_));
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
                    let mut worksheet = Worksheet::default();
                    worksheet.set_attributes_from_wookbook(&mut reader, e, is_empty);
                    wb.add_sheet(worksheet).unwrap();
                }
                b"pivotCache" => {
                    let cache_id = get_attribute(e, b"cacheId").unwrap();
                    let r_id = get_attribute(e, b"r:id").unwrap();
                    wb.add_pivot_caches((r_id, cache_id, String::new()));
                }
                b"definedName" => {
                    let mut obj = DefinedName::default();
                    obj.set_attributes(&mut reader, e);
                    defined_names.push(obj);
                }
                _ => (),
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
                if let Ok(s) = wb.sheet_by_name_mut(v.sheet_name()) {
                    s.add_defined_names(defined_name.clone());
                    continue;
                }
            }
            wb.add_defined_names(defined_name.clone());
        }
    }

    Ok(wb)
}
