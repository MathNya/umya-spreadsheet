use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;
use std::collections::BTreeMap;
use std::collections::HashMap;

use super::super::structs::style::Style;
use super::super::structs::worksheet::Worksheet;
use super::super::helper::coordinate::*;
use super::driver::*;
use super::XlsxError;

const SUB_DIR: &'static str = "xl/worksheets";

pub(crate) fn write(
    worksheet: &Worksheet,
    sheet_no: &usize,
    is_selected: &bool,
    has_macros: &bool,
    conditonal_style_list: Vec<(String, Style)>,
    shared_strings: HashMap<String, usize>,
    dir: &TempDir
) -> Result<(), XlsxError> 
{
    let file_name = format!("sheet{}.xml", sheet_no);
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    // worksheet
    write_start_tag(&mut writer, "worksheet", vec![
        ("xmlns", "http://schemas.openxmlformats.org/spreadsheetml/2006/main"),
        ("xmlns:r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships"),
        ("xmlns:mc", "http://schemas.openxmlformats.org/markup-compatibility/2006"),
        ("mc:Ignorable", "x14ac"),
        ("xmlns:x14ac", "http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac"),
    
    ], false);

    // sheetPr
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    let mut code_name = worksheet.get_code_name();
    if has_macros == &true {
        if !worksheet.has_code_name() {
            code_name = worksheet.get_title();
        }
    }
    attributes.push(("codeName", code_name));
    if worksheet.get_auto_filter().range != "" {
        attributes.push(("filterMode", "1"));
    }

    // tabColor
    if worksheet.get_tab_color().is_set() {
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let theme:&str = &worksheet.get_tab_color().get_theme_index().to_string();
        if worksheet.get_tab_color().is_set_theme_index() {
            attributes.push(("theme", theme));
        } else if worksheet.get_tab_color().get_argb() != "" {
            attributes.push(("rgb", worksheet.get_tab_color().get_argb()));
        }
        let tint:&str = &worksheet.get_tab_color().get_tint().to_string();
        if worksheet.get_tab_color().get_tint() != &0.0f64 {
            attributes.push(("tint", tint));
        }
        write_start_tag(&mut writer, "sheetPr", vec![], false);
        write_start_tag(&mut writer, "tabColor", attributes, true);
        write_end_tag(&mut writer, "sheetPr");
    }

    // outlinePr
    //write_start_tag(&mut writer, "outlinePr", vec![
    //    ("summaryBelow", if worksheet.show_summary_below {"1"} else {"0"}),
    //    ("summaryRight", if worksheet.show_summary_right {"1"} else {"0"}),
    //], true);

    // pageSetUpPr
    if worksheet.get_page_setup().get_fit_to_page() == &true {
        write_start_tag(&mut writer, "pageSetUpPr", vec![
            ("fitToPage", "1"),
        ], true);
    }

    // dimension
    write_start_tag(&mut writer, "dimension", vec![
        ("ref", worksheet.calculate_worksheet_dimension().as_str())
    ], true);

    // sheetViews
    write_start_tag(&mut writer, "sheetViews", vec![
        // ("ref",  worksheet.calculate_worksheet_dimension().as_str()),
    ], false);

    // sheetView
    let has_active_cell = worksheet.get_active_cell() != "";
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    if is_selected == &true {
        attributes.push(("tabSelected", "1"));
    }
    attributes.push(("workbookViewId", "0"));
    write_start_tag(&mut writer, "sheetView", attributes, !has_active_cell);

    if has_active_cell {
        // selection
        write_start_tag(&mut writer, "selection", vec![
            ("activeCell", worksheet.get_active_cell()),
            ("sqref", worksheet.get_active_cell()),
        ], true);
        write_end_tag(&mut writer, "sheetView");
    }

    write_end_tag(&mut writer, "sheetViews");

    // sheetFormatPr
    write_start_tag(&mut writer, "sheetFormatPr", vec![
        ("defaultRowHeight", "13.5"),
        ("x14ac:dyDescent", "0.15"),
    ], true);

    if worksheet.get_column_dimensions().len() > 0 {
        // cols
        write_start_tag(&mut writer, "cols", vec![], false);

        // col
        for cols in worksheet.get_column_dimensions() {
            write_start_tag(&mut writer, "col", vec![
                ("min", column_index_from_string(cols.get_column_index()).to_string().as_str()),
                ("max", column_index_from_string(cols.get_column_index()).to_string().as_str()),
                ("width", cols.get_width().to_string().as_str()),
                ("customWidth", "1"),
            ], true);
        }

        write_end_tag(&mut writer, "cols");
    }

    let mut cells_by_row = BTreeMap::new();
    let coordinates = worksheet.get_cell_collection().get_coordinates();
    for coordinate in &coordinates {
        let csf = coordinate_from_string(coordinate.as_str());
        let row_name:usize = csf.get(1).unwrap().parse().unwrap();
        let cells = match cells_by_row.get_mut(&row_name) {
            Some(v) => { v },
            None => {
                cells_by_row.insert(row_name.clone(), Vec::new());
                cells_by_row.get_mut(&row_name).unwrap()
            }
        };
        cells.push(coordinate);
    }
    let has_sheet_data = worksheet.get_row_dimensions().len() > 0;

    // sheetData
    write_start_tag(&mut writer, "sheetData", vec![], !has_sheet_data);

    if has_sheet_data {
        for (id, row) in worksheet.get_row_dimensions() {
            // row
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            let r:&str = &id.to_string();
            let ht:&str = &row.get_height().to_string();
            let descent:&str = &row.get_descent().to_string();
            attributes.push(("r", r));

            match cells_by_row.get_mut(id) {
                None => {
                    if ht != "0" {
                        attributes.push(("ht", ht));
                    }
                    if row.get_thick_bot() == &true {
                        attributes.push(("thickBot", "1"));
                    }
                    if row.get_custom_height() == &true {
                        attributes.push(("customHeight", "1"));
                    }
                    attributes.push(("x14ac:dyDescent", descent));
        
                    write_start_tag(&mut writer, "row", attributes, true);
                },
                Some(cells) => {
                    cells.sort();
                    let first = column_index_from_string(coordinate_from_string(cells.first().unwrap())[0]) + 1;
                    let last = column_index_from_string(coordinate_from_string(cells.last().unwrap())[0]) + 1;
                    let spans = format!("{}:{}", first, last);
                    attributes.push(("spans", &spans));
                    if ht != "0" {
                        attributes.push(("ht", ht));
                    }
                    if row.get_thick_bot() == &true {
                        attributes.push(("thickBot", "1"));
                    }
                    if row.get_custom_height() == &true {
                        attributes.push(("customHeight", "1"));
                    }
                    attributes.push(("x14ac:dyDescent", descent));

                    write_start_tag(&mut writer, "row", attributes, false);
                    for coordinate in cells {
                        let cell = worksheet.get_cell_collection().get(&coordinate);
                        // c
                        let mut attributes: Vec<(&str, &str)> = Vec::new();
                        attributes.push(("r", &coordinate));
                        let s:&str = &cell.get_xf_index().to_string();
                        if cell.get_xf_index() != &0usize {
                            attributes.push(("s", s));
                        }
                        if cell.get_data_type() != "" {
                            attributes.push(("t", cell.get_data_type()));
                        }
                        if cell.get_value() == "" {
                            write_start_tag(&mut writer, "c", attributes, true);
    
                        } else {
                            write_start_tag(&mut writer, "c", attributes, false);

                            // f
                            if cell.get_formula_attributes() != "" {
                                write_start_tag(&mut writer, "f", vec![], false);
                                write_text_node(&mut writer, cell.get_formula_attributes());
                                write_end_tag(&mut writer, "f");
                            }

                            // v
                            write_start_tag(&mut writer, "v", vec![], false);

                            match cell.get_data_type() {
                                "s" => {
                                    let val_index = shared_strings.get(cell.get_value()).unwrap().to_string();
                                    write_text_node(&mut writer, val_index);
                                },
                                "b" => write_text_node(&mut writer, cell.get_value()),
                                "" => write_text_node(&mut writer, cell.get_value()),
                                _ => println!("something else"),
                            }
                            write_end_tag(&mut writer, "v");
                            write_end_tag(&mut writer, "c");
                        }
                    }
                    write_end_tag(&mut writer, "row");
                },
            }
        }

        write_end_tag(&mut writer, "sheetData");
    }

    if worksheet.get_merge_cells().len() > 0 {
        // mergeCells
        write_start_tag(&mut writer, "mergeCells", vec![
            ("count", worksheet.get_merge_cells().len().to_string().as_str()),
        ], false);

        // mergeCell
        for merge_cell in worksheet.get_merge_cells() {
            write_start_tag(&mut writer, "mergeCell", vec![
                ("ref", merge_cell),
            ], true);
        }

        write_end_tag(&mut writer, "mergeCells");
    }

    // phoneticPr
    //write_start_tag(&mut writer, "phoneticPr", vec![
    //    ("fontId", "1"),
    //], true);

    // conditionalFormatting
    for (coordinates, conditional_formatting) in worksheet.get_conditional_styles_collection() {
        write_start_tag(&mut writer, "conditionalFormatting", vec![
            ("sqref", coordinates),
        ], false);

        // cfRule
        for conditional in conditional_formatting {
            let with_data_type = conditional.get_data_type() != "";
            let dxf_id_str: String;
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            attributes.push(("type", conditional.get_condition_type()));
            match conditional.get_style() {
                Some(v) => {
                    let mut dxf_id:usize = 0;
                    for (hash, _) in &conditonal_style_list {
                        if &v.get_hash_code() == hash {
                            break;
                        }
                        dxf_id += 1;
                    }
                    dxf_id_str = dxf_id.clone().to_string();
                    attributes.push(("dxfId", &dxf_id_str));
                },
                None => {}
            }
            let priority = conditional.get_priority();
            let priority_str:&str = &priority.to_string();
            if priority > &0 {
                attributes.push(("priority", priority_str));
            }
            let percent = conditional.get_percent();
            let percent_str:&str = &percent.to_string();
            if percent > &0 {
                attributes.push(("percent", percent_str));
            }
            let bottom = conditional.get_bottom();
            let bottom_str:&str = &bottom.to_string();
            if bottom > &0 {
                attributes.push(("bottom", bottom_str));
           }
            let rank = conditional.get_rank();
            let rank_str:&str = &rank.to_string();
            if rank > &0 {
                attributes.push(("rank", rank_str));
            }
            write_start_tag(&mut writer, "cfRule", attributes, !with_data_type);

            // Data Type (colorScale || )
            if with_data_type {
                write_start_tag(&mut writer, conditional.get_data_type(), vec![], false);

                // cfvo
                for (r#type, value, _) in conditional.get_cfvo_collection() {
                    let mut attributes: Vec<(&str, &str)> = Vec::new();
                    attributes.push(("type", r#type));
                    match value {
                        Some(v) => {
                            attributes.push(("value", v));
                        },
                        None => {}
                    }
                    write_start_tag(&mut writer, "cfvo", attributes, true);
                }

                // color
                for (_, _, color) in conditional.get_cfvo_collection() {
                    match color {
                        Some(v) => {
                            write_start_tag(&mut writer, "color", vec![
                                ("rgb", v.get_argb()),
                            ], true);
                        },
                        None => {}
                    }
                }

                write_end_tag(&mut writer, conditional.get_data_type());

                write_end_tag(&mut writer, "cfRule");
            }
        }

        write_end_tag(&mut writer, "conditionalFormatting");
    }

    // pageMargins
    write_start_tag(&mut writer, "pageMargins", vec![
        ("left", "0.7"),
        ("right", "0.7"),
        ("top", "0.75"),
        ("bottom", "0.75"),
        ("header", "0.3"),
        ("footer", "0.3"),
    ], true);

    // pageSetup
    //write_start_tag(&mut writer, "pageSetup", vec![
    //    ("paperSize", "9"),
    //    ("orientation", "portrait"),
    //    ("horizontalDpi", "0"),
    //    ("verticalDpi", "0"),
    //    ("r:id", "rId1"),
    //], true);

    if worksheet.has_drawing_object() {
        // drawing
        write_start_tag(&mut writer, "drawing", vec![
            ("r:id", "rId1"),
        ], true);
    }

    write_end_tag(&mut writer, "worksheet");
    let _ = make_file_from_writer(format!("{}/{}", SUB_DIR, file_name).as_str(), dir, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}
