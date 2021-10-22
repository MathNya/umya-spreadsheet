use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io;
use std::collections::BTreeSet;
use ::structs::Spreadsheet;
use ::structs::Worksheet;
use ::structs::SharedStringTable;
use ::structs::Stylesheet;
use super::driver::*;
use super::XlsxError;

const SUB_DIR: &'static str = "xl/worksheets";

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    sheet_index: &usize,
    shared_string_table: &mut SharedStringTable,
    stylesheet: &mut Stylesheet,
    arv: &mut zip::ZipWriter<W>,
) -> Result<(), XlsxError> 
{
    let is_selected = spreadsheet.get_active_sheet_index() == sheet_index;
    let has_macros = spreadsheet.get_has_macros();
    let sheet_no = sheet_index + 1;

    let file_name = format!("sheet{}.xml", &sheet_no);
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));

    {
        let worksheet = spreadsheet.get_sheet(sheet_index.clone()).unwrap();

       // XML header
        let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
        write_new_line(&mut writer);

        // worksheet
        write_start_tag(&mut writer, "worksheet", vec![
            ("xmlns", "http://schemas.openxmlformats.org/spreadsheetml/2006/main"),
            ("xmlns:r", "http://schemas.openxmlformats.org/officeDocument/2006/relationships"),
            ("xmlns:xdr", "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing"),
            ("xmlns:x14", "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main"),
            ("xmlns:mc", "http://schemas.openxmlformats.org/markup-compatibility/2006"),
            ("mc:Ignorable", "x14ac"),
            ("xmlns:x14ac", "http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac"),
        ], false);

        // sheetPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match has_macros {
            true => {
                let code_name = match worksheet.has_code_name() {
                    true => worksheet.get_code_name().as_ref().unwrap(),
                    false => worksheet.get_title()
                };
                attributes.push(("codeName", code_name));
            },
            false => {}
        }

        // tabColor
        match worksheet.get_tab_color() {
            Some(v) => {
                write_start_tag(&mut writer, "sheetPr", attributes, false);
                v.write_to_tab_color(&mut writer);
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
        if is_selected == true {
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

        // cols
        worksheet.get_column_dimensions_crate().write_to(&mut writer, stylesheet);

        // sheetData
        let has_sheet_data = worksheet.get_row_dimensions().len() > 0;
        write_start_tag(&mut writer, "sheetData", vec![], !has_sheet_data);

        for (row_num, row) in worksheet.get_row_dimensions_to_b_tree_map() {
            // cells
            let cells = worksheet.get_collection_by_row(&row_num);
            let mut col_num_list:BTreeSet<u32> = BTreeSet::new();
            for (col_num, _) in &cells {
                col_num_list.insert(col_num.clone());
            }

            // row
            let include_cell = col_num_list.len() > 0;
            let fist_num = match col_num_list.iter().next() {
                Some(col_num) => col_num,
                None => &0u32
            };
            let last_num = match col_num_list.iter().last() {
                Some(col_num) => col_num,
                None => &0u32
            };
            let spans = format!("{}:{}", fist_num, last_num);
            row.write_to(&mut writer, stylesheet, spans, !include_cell);

            // c
            for (_, cell) in cells {
                cell.write_to(&mut writer, shared_string_table, stylesheet);
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
                    ("ref", &v.get_range().get_range()),
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
                    ("ref", merge_cell.get_range().as_str()),
                ], true);
            }

            write_end_tag(&mut writer, "mergeCells");
        }

        // phoneticPr
        //write_start_tag(&mut writer, "phoneticPr", vec![
        //    ("fontId", "1"),
        //], true);

        // conditionalFormatting
        for conditional_formatting in worksheet.get_conditional_styles_collection() {
            let coordinates = conditional_formatting.get_sqref();
            write_start_tag(&mut writer, "conditionalFormatting", vec![
                ("sqref", &coordinates),
            ], false);

            // cfRule
            for conditional in conditional_formatting.get_conditional_collection() {
                let with_data_type = conditional.get_data_type() != "";
                let dxf_id_str: String;
                let mut attributes: Vec<(&str, &str)> = Vec::new();
                attributes.push(("type", conditional.get_condition_type()));
                match conditional.get_style() {
                    Some(v) => {
                        let dxf_id = stylesheet.get_differential_formats_mut().set_style(v);
                        dxf_id_str = dxf_id.to_string();
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
    }
    
    write_end_tag(&mut writer, "worksheet");
    let _ = make_file_from_writer(format!("{}/{}", SUB_DIR, file_name).as_str(), arv, writer, None).unwrap();
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