use super::driver::*;
use super::XlsxError;
use crate::helper::const_str::*;
use crate::structs::Cell;
use crate::structs::SharedStringTable;
use crate::structs::Stylesheet;
use crate::structs::Worksheet;
use crate::structs::WriterManager;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use std::sync::RwLock;

pub(crate) fn write<W: io::Seek + io::Write>(
    sheet_no: &i32,
    worksheet: &Worksheet,
    shared_string_table: Arc<RwLock<SharedStringTable>>,
    stylesheet: &mut Stylesheet,
    has_macros: bool,
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

    // worksheet
    write_start_tag(
        &mut writer,
        "worksheet",
        vec![
            ("xmlns", SHEET_MAIN_NS),
            ("xmlns:r", REL_OFC_NS),
            ("xmlns:xdr", SHEET_DRAWING_NS),
            ("xmlns:x14", SHEET_MS_MAIN_NS),
            ("xmlns:mc", MC_NS),
            ("mc:Ignorable", "x14ac"),
            ("xmlns:x14ac", SHEETML_AC_NS),
        ],
        false,
    );

    // sheetPr
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    if has_macros {
        let code_name = match worksheet.has_code_name() {
            true => worksheet.get_code_name().as_ref().unwrap(),
            false => worksheet.get_name(),
        };
        attributes.push(("codeName", code_name));
    }

    // tabColor
    match worksheet.get_tab_color() {
        Some(v) => {
            write_start_tag(&mut writer, "sheetPr", attributes, false);
            v.write_to_tab_color(&mut writer);
            write_end_tag(&mut writer, "sheetPr");
        }
        None => {
            if !attributes.is_empty() {
                write_start_tag(&mut writer, "sheetPr", attributes, true);
            }
        }
    }

    // outlinePr
    //write_start_tag(&mut writer, "outlinePr", vec![
    //    ("summaryBelow", if worksheet.show_summary_below {"1"} else {"0"}),
    //    ("summaryRight", if worksheet.show_summary_right {"1"} else {"0"}),
    //], true);

    // dimension
    write_start_tag(
        &mut writer,
        "dimension",
        vec![("ref", worksheet.calculate_worksheet_dimension().as_str())],
        true,
    );

    // sheetViews
    worksheet.get_sheets_views().write_to(&mut writer);

    // sheetFormatPr
    worksheet
        .get_sheet_format_properties()
        .write_to(&mut writer);

    // cols
    let mut column_dimensions = worksheet.get_column_dimensions_crate().clone();
    column_dimensions.calculation_auto_width(
        worksheet.get_cell_collection_crate(),
        worksheet.get_merge_cells_crate(),
    );
    column_dimensions.write_to(&mut writer, stylesheet);

    // sheetData
    let has_sheet_data = worksheet.has_sheet_data();
    write_start_tag(&mut writer, "sheetData", vec![], !has_sheet_data);

    // row dimensions sort.
    let mut row_dimensions = worksheet.get_row_dimensions();
    row_dimensions.sort_by(|a, b| a.get_row_num().cmp(b.get_row_num()));

    // it's faster than get cell collection by row.
    // cells sort.
    let mut cells = worksheet.get_cell_collection_sorted();

    // make formula shared list
    let mut formula_shared_list: HashMap<&u32, (String, Option<String>)> = HashMap::new();
    for cell in &cells {
        if let Some(si) = cell.get_formula_shared_index() {
            match formula_shared_list.get(si) {
                Some((start_cell, _)) => {
                    formula_shared_list.insert(
                        si,
                        (
                            start_cell.clone(),
                            Some(cell.get_coordinate().get_coordinate()),
                        ),
                    );
                }
                None => {
                    formula_shared_list.insert(si, (cell.get_coordinate().get_coordinate(), None));
                }
            }
        }
    }

    let mut cells_iter = cells.iter().peekable();

    // row loop
    for row in &row_dimensions {
        let mut cells_in_row: Vec<&Cell> = Vec::new();

        while let Some(cell) = cells_iter.peek() {
            if row.get_row_num() != cell.get_coordinate().get_row_num() {
                break;
            }

            cells_in_row.push(cells_iter.next().unwrap());
        }

        // row
        if cells_in_row.is_empty() {
            let spans = "0:0".to_string();
            row.write_to(&mut writer, stylesheet, spans, true);
        } else {
            let (first_num, last_num) = (
                cells_in_row.first().unwrap().get_coordinate().get_col_num(),
                cells_in_row.last().unwrap().get_coordinate().get_col_num(),
            );
            let spans = format!("{first_num}:{last_num}");

            row.write_to(&mut writer, stylesheet, spans, false);
            // c
            for cell in cells_in_row {
                cell.write_to(
                    &mut writer,
                    &shared_string_table,
                    stylesheet,
                    &formula_shared_list,
                );
            }

            write_end_tag(&mut writer, "row");
        }
    }

    if has_sheet_data {
        write_end_tag(&mut writer, "sheetData");
    }

    // sheetProtection
    if let Some(v) = worksheet.get_sheet_protection() {
        v.write_to(&mut writer);
    }

    // autoFilter
    if let Some(v) = worksheet.get_auto_filter() {
        write_start_tag(
            &mut writer,
            "autoFilter",
            vec![("ref", &v.get_range().get_range())],
            true,
        );
    }

    // mergeCells
    worksheet.get_merge_cells_crate().write_to(&mut writer);

    // phoneticPr
    write_start_tag(&mut writer, "phoneticPr", vec![("fontId", "1")], true);

    // conditionalFormatting
    for conditional_formatting in worksheet.get_conditional_formatting_collection() {
        conditional_formatting.write_to(&mut writer, stylesheet.get_differential_formats_mut());
    }

    // dataValidations
    if let Some(v) = worksheet.get_data_validations() {
        v.write_to(&mut writer);
    }

    let mut r_id = 1;

    // hyperlinks
    if worksheet.has_hyperlink() {
        write_start_tag(&mut writer, "hyperlinks", vec![], false);

        // hyperlink
        for (coordition, hyperlink) in worksheet.get_hyperlink_collection_to_hashmap() {
            let r_id_str = format!("rId{}", &r_id);
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            attributes.push(("ref", &coordition));
            if *hyperlink.get_location() {
                attributes.push(("location", hyperlink.get_url()));
            } else {
                attributes.push(("r:id", r_id_str.as_str()));
                r_id += 1;
            }
            write_start_tag(&mut writer, "hyperlink", attributes, true);
        }

        write_end_tag(&mut writer, "hyperlinks");
    }

    // printOptions
    worksheet.get_print_options().write_to(&mut writer);

    // pageMargins
    worksheet.get_page_margins().write_to(&mut writer);

    // pageSetup
    if worksheet.get_page_setup().has_param() {
        worksheet.get_page_setup().write_to(&mut writer, &mut r_id);
    }

    // headerFooter
    worksheet.get_header_footer().write_to(&mut writer);

    // rowBreaks
    worksheet.get_row_breaks().write_to(&mut writer);

    // colBreaks
    worksheet.get_column_breaks().write_to(&mut writer);

    if worksheet.has_drawing_object() {
        // drawing
        let r_id_str = format!("rId{}", &r_id);
        write_start_tag(
            &mut writer,
            "drawing",
            vec![("r:id", r_id_str.as_str())],
            true,
        );
        r_id += 1;
    }

    // legacyDrawing
    if worksheet.has_legacy_drawing() {
        let r_id_str = format!("rId{}", &r_id);
        write_start_tag(
            &mut writer,
            "legacyDrawing",
            vec![("r:id", r_id_str.as_str())],
            true,
        );
        r_id += 1;
    }

    // tableParts
    if worksheet.has_table() {
        let tables = worksheet.get_tables();
        write_start_tag(
            &mut writer,
            "tableParts",
            vec![("count", &tables.len().to_string())],
            false,
        );
        for table in worksheet.get_tables().iter() {
            let r_id_str = format!("rId{}", &r_id);
            write_start_tag(
                &mut writer,
                "tablePart",
                vec![("r:id", r_id_str.as_str())],
                true,
            );
            r_id += 1;
        }
        write_end_tag(&mut writer, "tableParts");
    }

    // oleObjects
    let ole_id = 1000 + 25;
    worksheet
        .get_ole_objects()
        .write_to(&mut writer, &r_id, &ole_id);

    // extLst
    if worksheet.get_data_validations_2010().is_some() {
        write_start_tag(&mut writer, "extLst", vec![], false);
        match worksheet.get_data_validations_2010() {
            Some(v) => {
                v.write_to(&mut writer);
            }
            None => {}
        }
        write_end_tag(&mut writer, "extLst");
    }

    write_end_tag(&mut writer, "worksheet");

    let target = format!("{PKG_SHEET}{}.xml", sheet_no);
    writer_mng.add_writer(&target, writer)
}
