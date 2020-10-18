use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::BTreeSet;

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
    all_cell_xf_list: Vec<(String, Style)>,
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
    match has_macros {
        &true => {
            let code_name = match worksheet.has_code_name() {
                true => worksheet.get_code_name().as_ref().unwrap(),
                false => worksheet.get_title()
            };
            attributes.push(("codeName", code_name));
        },
        &false => {}
    }

    // tabColor
    match worksheet.get_tab_color() {
        Some(v) => {
            write_start_tag(&mut writer, "sheetPr", attributes, false);
            write_color(&mut writer, v, "tabColor");
            write_end_tag(&mut writer, "sheetPr");
        },
        None => {
            if attributes.len() > 0 {
                write_start_tag(&mut writer, "sheetPr", attributes, true);
            }
        }
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
    let coordinates = worksheet.get_coordinates();
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

    // sheetData
    let has_sheet_data = worksheet.get_row_dimensions().len() > 0;
    write_start_tag(&mut writer, "sheetData", vec![], !has_sheet_data);

    for (row_num, row) in worksheet.get_row_dimensions() {
        // cells and styles
        let cells = worksheet.get_collection_by_row(row_num);
        let styles = worksheet.get_style_collection_by_row(row_num);
        let mut col_num_list:BTreeSet<usize> = BTreeSet::new();
        for (col_num, _) in &cells {
            col_num_list.insert(col_num.clone());
        }
        for (col_num, _) in &styles {
            col_num_list.insert(col_num.clone());
        }

        // row
        let include_cell = col_num_list.len() > 0;
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let r_string = row_num.to_string();
        attributes.push(("r", &r_string));
        let fist_num = match col_num_list.iter().next() {
            Some(col_num) => col_num,
            None => &0usize
        };
        let last_num = match col_num_list.iter().last() {
            Some(col_num) => col_num,
            None => &0usize
        };
        let spans = format!("{}:{}", fist_num, last_num);
        if include_cell {
            attributes.push(("spans", &spans));
        }
        let ht = row.get_height().to_string();
        if row.get_height() != &0f32 {
            attributes.push(("ht", &ht));
        }
        if row.get_thick_bot() == &true {
            attributes.push(("thickBot", "1"));
        }
        if row.get_custom_height() == &true {
            attributes.push(("customHeight", "1"));
        }
        let dy_descent = row.get_descent().to_string();
        attributes.push(("x14ac:dyDescent", &dy_descent));
        write_start_tag(&mut writer, "row", attributes, !include_cell);

        for col_num in col_num_list {
            let cell = &cells.get(&col_num);
            let style = &styles.get(&col_num);

            let coordinate = coordinate_from_index(&col_num, row_num);
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            attributes.push(("r", &coordinate));

            let mut xf_index:usize = 0;
            match style {
                Some(v) => {
                    for (hash_code, _) in &all_cell_xf_list {
                        if v.get_hash_code().as_str() == hash_code {
                            break;
                        }
                        xf_index += 1;
                    }
                },
                None => {}
            }
            let xf_index_str:&str = &xf_index.to_string();
            if xf_index != 0usize {
                attributes.push(("s", xf_index_str));
            }

            match cell {
                Some(c) => {
                    if c.get_data_type() == "s" || c.get_data_type() == "b" {
                        attributes.push(("t", c.get_data_type()));
                    }
                    write_start_tag(&mut writer, "c", attributes, false);

                    // f
                    if c.get_formula_attributes() != "" {
                        write_start_tag(&mut writer, "f", vec![], false);
                        write_text_node(&mut writer, c.get_formula_attributes());
                        write_end_tag(&mut writer, "f");
                    }

                    // v
                    write_start_tag(&mut writer, "v", vec![], false);

                    match c.get_data_type() {
                        "s" => {
                            let val_index = shared_strings.get(&c.get_hash_code_by_value()).unwrap().to_string();
                            write_text_node(&mut writer, val_index);
                        },
                        "b" => {
                            let upper_value = c.get_value().to_uppercase();
                            let prm = if upper_value == "TRUE" {"1"} else {"0"};
                            write_text_node(&mut writer, prm);
                        },
                        _ => write_text_node(&mut writer, c.get_value()),
                    }
                    write_end_tag(&mut writer, "v");
                    write_end_tag(&mut writer, "c");
                },
                None => {
                    write_start_tag(&mut writer, "c", attributes, true);
                }
            }
        }
        if include_cell {
            write_end_tag(&mut writer, "row");
        }
    }

    if has_sheet_data {
        write_end_tag(&mut writer, "sheetData");
    }

    // autoFilter
    match worksheet.get_auto_filter() {
        Some(v) => {
            write_start_tag(&mut writer, "autoFilter", vec![
                ("ref", v.get_range()),
            ], true);
        },
        None => {}
    };

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

    // hyperlinks
    if worksheet.get_hyperlink_collection().len() > 0 {
        write_start_tag(&mut writer, "hyperlinks", vec![], false);

        // hyperlink
        let mut i = first_hyperlink_id(worksheet);
        for (coordition, hyperlink) in worksheet.get_hyperlink_collection() {
            let rid = format!("rId{}", &i);
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            attributes.push(("ref", &coordition));
            if hyperlink.get_location() == &true {
                attributes.push(("location", hyperlink.get_url()));
            } else {
                attributes.push(("r:id", rid.as_str()));
                i += 1;
            }
            write_start_tag(&mut writer, "hyperlink", attributes, true);
        }

        write_end_tag(&mut writer, "hyperlinks");
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

    // comment
    if worksheet.get_comments().len() > 0 {
        let i = first_legacy_drawing_id(worksheet);
        let rid = format!("rId{}", &i);
        // legacyDrawing
        write_start_tag(&mut writer, "legacyDrawing", vec![
            ("r:id", rid.as_str()),
        ], true);
    }

    write_end_tag(&mut writer, "worksheet");
    let _ = make_file_from_writer(format!("{}/{}", SUB_DIR, file_name).as_str(), dir, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}

fn first_hyperlink_id(worksheet: &Worksheet) -> usize {
    let mut result = 1;
    if worksheet.has_drawing_object() {
        result += 1;
    }
    result
}

fn first_legacy_drawing_id(worksheet: &Worksheet) -> usize {
    let mut result = 1;
    if worksheet.has_drawing_object() {
        result += 1;
    }
    for (_, hyperlink) in worksheet.get_hyperlink_collection() {
        if hyperlink.get_location() == &false {
            result += 1;
        }
    }
    result
}