use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use super::XlsxError;
use helper::const_str::*;
use structs::Spreadsheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    // workbook
    write_start_tag(
        &mut writer,
        "workbook",
        vec![("xmlns", SHEET_MAIN_NS), ("xmlns:r", REL_OFC_NS)],
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
        attributes.push(("codeName", &spreadsheet.get_code_name().unwrap_or("ThisWorkbook")));
    }
    write_start_tag(&mut writer, "workbookPr", attributes, true);

    // workbookProtection
    if let Some(v) = spreadsheet.get_workbook_protection() {
        v.write_to(&mut writer);
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
        let attributes: Vec<(&str, &str)> = vec![
            ("name", worksheet.get_name()),
            ("sheetId", &id),
            ("r:id", &r_id),
        ];

        // sheet
        write_start_tag(&mut writer, "sheet", attributes, true);

        index += 1;
    }
    write_end_tag(&mut writer, "sheets");

    // definedNames
    if spreadsheet.has_defined_names() {
        write_start_tag(&mut writer, "definedNames", vec![], false);

        for defined_name in spreadsheet.get_defined_names() {
            defined_name.write_to(&mut writer);
        }
        for sheet in spreadsheet.get_sheet_collection_no_check() {
            for defined_name in sheet.get_defined_names() {
                defined_name.write_to(&mut writer);
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

    writer_mng.add_writer(PKG_WORKBOOK, writer)
}
