use crate::Row;

use super::driver::*;
use super::XlsxError;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::collections::HashMap;
use std::io;
use std::iter::Map;
use std::sync::Arc;
use std::sync::RwLock;
use structs::Cell;
use structs::SharedStringTable;
use structs::Stylesheet;
use structs::Worksheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    sheet_no: &i32,
    worksheet: &Worksheet,
    shared_string_table: Arc<RwLock<SharedStringTable>>,
    stylesheet: &mut Stylesheet,
    has_macros: bool,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));

    {
        // XML header
        let _ = writer.write_event(Event::Decl(BytesDecl::new(
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
                (
                    "xmlns",
                    "http://schemas.openxmlformats.org/spreadsheetml/2006/main",
                ),
                (
                    "xmlns:r",
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships",
                ),
                (
                    "xmlns:xdr",
                    "http://schemas.openxmlformats.org/drawingml/2006/spreadsheetDrawing",
                ),
                (
                    "xmlns:x14",
                    "http://schemas.microsoft.com/office/spreadsheetml/2009/9/main",
                ),
                (
                    "xmlns:mc",
                    "http://schemas.openxmlformats.org/markup-compatibility/2006",
                ),
                ("mc:Ignorable", "x14ac"),
                (
                    "xmlns:x14ac",
                    "http://schemas.microsoft.com/office/spreadsheetml/2009/9/ac",
                ),
            ],
            false,
        );

        // sheetPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match has_macros {
            true => {
                let code_name = match worksheet.has_code_name() {
                    true => worksheet.get_code_name().as_ref().unwrap(),
                    false => worksheet.get_name(),
                };
                attributes.push(("codeName", code_name));
            }
            false => {}
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
        write_start_tag(
            &mut writer,
            "sheetFormatPr",
            vec![("defaultRowHeight", "13.5"), ("x14ac:dyDescent", "0.15")],
            true,
        );

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
        let mut cells = worksheet.get_cell_collection();
        cells.sort_by(|a, b| {
            (
                a.get_coordinate().get_row_num(),
                a.get_coordinate().get_col_num(),
            )
                .cmp(&(
                    b.get_coordinate().get_row_num(),
                    b.get_coordinate().get_col_num(),
                ))
        });

        // row loop
        let cell_map = cells
            .iter()
            .fold(HashMap::<u32, Vec<&Cell>>::new(), |mut acc, cell| {
                acc.entry(cell.get_coordinate().get_row_num().to_owned())
                    .or_insert_with(|| Vec::default())
                    .push(cell);
                acc
            });

        let mut row_map = HashMap::<u32, (Option<&Row>, Option<&[&Cell]>)>::new();

        let mut cell_map_iter = cell_map.iter().peekable();
        let mut row_dimensions_iter = row_dimensions.iter().peekable();

        while cell_map_iter.peek().is_some() || row_dimensions_iter.peek().is_some() {
            if let Some((row_n, cells)) = cell_map_iter.next() {
                row_map
                    .entry(row_n.to_owned())
                    .or_insert_with(|| (Option::default(), Option::default()))
                    .1
                    .replace(cells);
            }

            if let Some(row_dim) = row_dimensions_iter.next() {
                row_map
                    .entry(row_dim.get_row_num().to_owned())
                    .or_insert_with(|| (Option::default(), Option::default()))
                    .0
                    .replace(row_dim);
            }
        }

        for (_, (row, row_cells)) in row_map {
            let default_row = &Row::default();
            let row = row.unwrap_or(default_row);
            let default_cells = &Vec::<&Cell>::new();
            let row_cells = row_cells.unwrap_or(default_cells);

            if !row_cells.is_empty() {
                let first_num = row_cells.get(0).unwrap().get_coordinate().get_col_num();
                let last_num = row_cells
                    .iter()
                    .last()
                    .unwrap()
                    .get_coordinate()
                    .get_col_num();
                let spans = format!("{}:{}", first_num, last_num);

                row.write_to(&mut writer, stylesheet, spans, false);

                // c
                for cell in row_cells {
                    cell.write_to(&mut writer, shared_string_table.clone(), stylesheet);
                }
                write_end_tag(&mut writer, "row");
            } else {
                let spans = format!("{}:{}", 0, 0);

                row.write_to(&mut writer, stylesheet, spans, false);
            }
        }

        if has_sheet_data {
            write_end_tag(&mut writer, "sheetData");
        }

        // autoFilter
        match worksheet.get_auto_filter() {
            Some(v) => {
                write_start_tag(
                    &mut writer,
                    "autoFilter",
                    vec![("ref", &v.get_range().get_range())],
                    true,
                );
            }
            None => {}
        };

        // mergeCells
        worksheet.get_merge_cells_crate().write_to(&mut writer);

        // phoneticPr
        write_start_tag(&mut writer, "phoneticPr", vec![("fontId", "1")], true);

        // conditionalFormatting
        for conditional_formatting in worksheet.get_conditional_formatting_collection() {
            conditional_formatting.write_to(&mut writer, stylesheet.get_differential_formats_mut());
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
                if hyperlink.get_location() == &true {
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

        // oleObjects
        let ole_id = 1000 + 25;
        worksheet
            .get_ole_objects()
            .write_to(&mut writer, &r_id, &ole_id);
    }

    write_end_tag(&mut writer, "worksheet");

    let target = format!("xl/worksheets/sheet{}.xml", sheet_no);
    writer_mng.add_writer(&target, writer)
}
