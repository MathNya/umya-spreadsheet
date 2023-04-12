use super::driver::*;
use super::XlsxError;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;
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
            let include_cell = !cells_in_row.is_empty();
            if include_cell {
                let first_num = cells_in_row.first().unwrap().get_coordinate().get_col_num();
                let last_num = cells_in_row.last().unwrap().get_coordinate().get_col_num();
                let spans = format!("{first_num}:{last_num}");

                row.write_to(&mut writer, stylesheet, spans, false);
                // c
                for cell in cells_in_row {
                    cell.write_to(&mut writer, shared_string_table.clone(), stylesheet);
                }

                write_end_tag(&mut writer, "row");
            } else {
                let spans = "0:0".to_string();
                row.write_to(&mut writer, stylesheet, spans, true);
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

        // dataValidations
        match worksheet.get_data_validations() {
            Some(v) => {
                v.write_to(&mut writer);
            }
            None => {}
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
