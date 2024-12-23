use std::io;

use quick_xml::{
    Writer,
    events::{
        BytesDecl,
        Event,
    },
};

use super::{
    XlsxError,
    driver::{
        write_end_tag,
        write_new_line,
        write_start_tag,
    },
};
use crate::{
    helper::const_str::{
        PKG_WORKBOOK,
        REL_OFC_NS,
        SHEET_MAIN_NS,
    },
    structs::{
        Workbook,
        WriterManager,
    },
};

pub(crate) fn write<W: io::Seek + io::Write>(
    wb: &Workbook,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer
        .write_event(Event::Decl(BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            Some("yes"),
        )))
        .unwrap();
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
    // attributes.push(("defaultThemeVersion", "124226"));
    if wb.get_has_macros() {
        attributes.push(("codeName", wb.get_code_name().unwrap_or("ThisWorkbook")));
    }
    write_start_tag(&mut writer, "workbookPr", attributes, true);

    // workbookProtection
    if let Some(v) = wb.get_workbook_protection() {
        v.write_to(&mut writer);
    }

    // bookViews
    write_start_tag(&mut writer, "bookViews", vec![], false);

    // workbookView
    wb.get_workbook_view().write_to(&mut writer);

    write_end_tag(&mut writer, "bookViews");

    // // sheets
    write_start_tag(&mut writer, "sheets", vec![], false);

    let mut index = 1;
    for worksheet in wb.get_sheet_collection_no_check() {
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let id = index.to_string();
        let r_id = format!("rId{index}");
        attributes.push(("name", worksheet.get_name()));
        attributes.push(("sheetId", &id));
        attributes.push(("r:id", &r_id));
        if worksheet.has_state() {
            attributes.push(("state", worksheet.get_state_str()));
        }

        // sheet
        write_start_tag(&mut writer, "sheet", attributes, true);

        index += 1;
    }
    write_end_tag(&mut writer, "sheets");

    // definedNames
    if wb.has_defined_names() {
        write_start_tag(&mut writer, "definedNames", vec![], false);

        for defined_name in wb.get_defined_names() {
            defined_name.write_to(&mut writer);
        }
        for sheet in wb.get_sheet_collection_no_check() {
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
    let pivot_cache_definition_collection = wb.get_pivot_caches();
    if !pivot_cache_definition_collection.is_empty() {
        write_start_tag(&mut writer, "pivotCaches", vec![], false);
        for (_, val2, _) in pivot_cache_definition_collection {
            let r_id = format!("rId{index}");
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
