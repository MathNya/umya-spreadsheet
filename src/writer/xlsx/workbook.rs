use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use super::XlsxError;
use structs::Spreadsheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    // workbook
    write_start_tag(
        &mut writer,
        "workbook",
        vec![
            (
                "xmlns",
                "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
            ),
            (
                "xmlns:r",
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
            ),
        ],
        false,
    );

    // fileVersion
    write_start_tag(
        &mut writer,
        "fileVersion",
        vec![
            ("appName", "xl"),
            ("lastEdited", "5"),
            ("lowestEdited", "4"),
            ("rupBuild", "9302"),
        ],
        true,
    );

    // workbookPr
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    attributes.push(("filterPrivacy", "1"));
    //attributes.push(("defaultThemeVersion", "124226"));
    if spreadsheet.get_has_macros() {
        attributes.push(("codeName", "ThisWorkbook"));
    }
    write_start_tag(&mut writer, "workbookPr", attributes, true);

    // workbookProtection
    if spreadsheet.get_security().is_security_enabled() {
        let tag_name = "workbookProtection";
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push((
            "lockRevision",
            if spreadsheet.get_security().get_lock_revision() == &true {
                "true"
            } else {
                "false"
            },
        ));
        attributes.push((
            "lockStructure",
            if spreadsheet.get_security().get_lock_structure() == &true {
                "true"
            } else {
                "false"
            },
        ));
        attributes.push((
            "lockWindows",
            if spreadsheet.get_security().get_lock_windows() == &true {
                "true"
            } else {
                "false"
            },
        ));
        if spreadsheet.get_security().get_revisions_password() != "" {
            attributes.push((
                "lockWindows",
                spreadsheet.get_security().get_revisions_password(),
            ));
        }
        if spreadsheet.get_security().get_workbook_password() != "" {
            attributes.push((
                "workbookPassword",
                spreadsheet.get_security().get_workbook_password(),
            ));
        }
        write_start_tag(&mut writer, tag_name, attributes, true);
    }

    // bookViews
    write_start_tag(&mut writer, "bookViews", vec![], false);

    // workbookView
    spreadsheet.get_workbook_view().write_to(&mut writer);

    write_end_tag(&mut writer, "bookViews");

    // // sheets
    write_start_tag(&mut writer, "sheets", vec![], false);

    let mut index = 1;
    for worksheet in spreadsheet.get_sheet_collection_no_check() {
        let id = index.to_string();
        let r_id = format!("rId{}", index);
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("name", worksheet.get_name()));
        attributes.push(("sheetId", &id));
        attributes.push(("r:id", &r_id));

        // sheet
        write_start_tag(&mut writer, "sheet", attributes, true);

        index += 1;
    }
    write_end_tag(&mut writer, "sheets");

    // definedNames
    if spreadsheet.has_defined_names() {
        write_start_tag(&mut writer, "definedNames", vec![], false);

        for sheet in spreadsheet.get_sheet_collection_no_check() {
            for defined_name in sheet.get_defined_names() {
                // definedName
                let mut attributes: Vec<(&str, &str)> = Vec::new();
                attributes.push(("name", defined_name.get_name()));
                if defined_name.get_is_local_only() == &true {
                    attributes.push(("localSheetId", "0"));
                    attributes.push(("hidden", "1"));
                }
                write_start_tag(&mut writer, "definedName", attributes, false);
                write_text_node_no_escape(&mut writer, defined_name.get_address_str());
                write_end_tag(&mut writer, "definedName");
            }
        }

        write_end_tag(&mut writer, "definedNames");
    }

    // calcPr
    write_start_tag(
        &mut writer,
        "calcPr",
        vec![
            ("calcId", "122211"),
            //("calcId", "999999"),
            //("calcMode", "auto"),
            //("calcCompleted", if recalc_required {"1"} else {"0"}),
            //("fullCalcOnLoad", if recalc_required {"0"} else {"1"}),
            //("forceFullCalc", if recalc_required {"0"} else {"1"}),
        ],
        true,
    );

    // pivotCaches
    let pivot_cache_definition_collection = spreadsheet.get_pivot_caches();
    if !pivot_cache_definition_collection.is_empty() {
        write_start_tag(&mut writer, "pivotCaches", vec![], false);
        for (_, val2, _) in pivot_cache_definition_collection {
            let r_id = format!("rId{}", index);
            write_start_tag(
                &mut writer,
                "pivotCache",
                vec![("cacheId", &val2), ("r:id", &r_id)],
                true,
            );
            index += 1;
        }
        write_end_tag(&mut writer, "pivotCaches");
    }

    write_end_tag(&mut writer, "workbook");

    let target = "xl/workbook.xml";
    writer_mng.add_writer(target, writer)
}
