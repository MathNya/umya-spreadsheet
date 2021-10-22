use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io;

use ::structs::Spreadsheet;
use super::driver::*;
use super::XlsxError;

pub(crate) fn write<W: io::Seek + io::Write>(spreadsheet: &Spreadsheet, arv: &mut zip::ZipWriter<W>, sub_dir: &str, file_name: &str) -> Result<(), XlsxError>
{
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    // workbook
    write_start_tag(&mut writer, "workbook", vec![
        ("xmlns", "http://schemas.openxmlformats.org/spreadsheetml/2006/main"),
        ("xmlns:r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships"),
    ], false);

    // fileVersion
    write_start_tag(&mut writer, "fileVersion", vec![
        ("appName", "xl"),
        ("lastEdited", "5"),
        ("lowestEdited", "4"),
        ("rupBuild", "9302"),
    ], true);

    // workbookPr
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    attributes.push(("filterPrivacy", "1"));
    //attributes.push(("defaultThemeVersion", "124226"));
    match spreadsheet.get_has_macros() {
        &true => attributes.push(("codeName", "ThisWorkbook")),
        &false => {}
    }
    write_start_tag(&mut writer, "workbookPr", attributes, true);

    // workbookProtection
    if spreadsheet.get_security().is_security_enabled() {
        let tag_name = "workbookProtection";
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("lockRevision", if spreadsheet.get_security().get_lock_revision() == &true { "true" } else {"false"}));
        attributes.push(("lockStructure", if spreadsheet.get_security().get_lock_structure() == &true { "true" } else {"false"}));
        attributes.push(("lockWindows", if spreadsheet.get_security().get_lock_windows() == &true { "true" } else {"false"}));
        if spreadsheet.get_security().get_revisions_password() != "" {
            attributes.push(("lockWindows", spreadsheet.get_security().get_revisions_password()));
        }
        if spreadsheet.get_security().get_workbook_password() != "" {
            attributes.push(("workbookPassword", spreadsheet.get_security().get_workbook_password()));
        }
        write_start_tag(&mut writer, tag_name, attributes, true);
    }

    // bookViews
    write_start_tag(&mut writer, "bookViews", vec![
    ], false);

    // workbookView
    write_start_tag(&mut writer, "workbookView", vec![
        ("xWindow", "240"),
        ("yWindow", "105"),
        ("windowWidth", "14805"),
        ("windowHeight", "8010"),
    ], true);

    write_end_tag(&mut writer, "bookViews");

    // // sheets
    write_start_tag(&mut writer, "sheets", vec![], false);
    for i in 0..spreadsheet.get_sheet_count() {
        let id = (i+1).to_string();
        let r_id = format!("rId{}", (i+1+3).to_string());
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("name", spreadsheet.get_sheet_collection()[i].get_title()));
        attributes.push(("sheetId", id.as_str()));
        if spreadsheet.get_sheet_collection()[i].get_sheet_state() != "visible"
         && spreadsheet.get_sheet_collection()[i].get_sheet_state() != ""
        {
            attributes.push(("state", spreadsheet.get_sheet_collection()[i].get_sheet_state()));
        }
        attributes.push(("r:id",  r_id.as_str()));

        // sheet
        write_start_tag(&mut writer, "sheet", attributes, true);
    }
    write_end_tag(&mut writer, "sheets");

    // definedNames
    if spreadsheet.get_defined_names().len() > 1 {
        write_start_tag(&mut writer, "definedNames", vec![], false);

        for defined_name in spreadsheet.get_defined_names() {
            // definedName
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            attributes.push(("name", defined_name.get_name()));
            if defined_name.get_is_local_only() == &true {
                attributes.push(("localSheetId", "0"));
                attributes.push(("hidden", "1"));
            }
            write_start_tag(&mut writer, "definedName", attributes, false);
            write_text_node(&mut writer, defined_name.get_address());
            write_end_tag(&mut writer, "definedName");
        }

        write_end_tag(&mut writer, "definedNames");
    }

    // calcPr
    write_start_tag(&mut writer, "calcPr", vec![
        ("calcId", "122211"),
        //("calcId", "999999"),
        //("calcMode", "auto"),
        //("calcCompleted", if recalc_required {"1"} else {"0"}),
        //("fullCalcOnLoad", if recalc_required {"0"} else {"1"}),
        //("forceFullCalc", if recalc_required {"0"} else {"1"}),
    ], true);
    
    write_end_tag(&mut writer, "workbook");
    let _ = make_file_from_writer(&file_name, arv, writer, Some(sub_dir)).unwrap();
    Ok(())
}
