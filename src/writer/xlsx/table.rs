use std::io;

use super::{driver::*, XlsxError};
use crate::helper::const_str::*;
use crate::structs::{Worksheet, WriterManager};
use quick_xml::{
    events::{BytesDecl, Event},
    Writer,
};

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<Vec<String>, XlsxError> {
    let mut table_no_list = Vec::<String>::new();
    for table in worksheet.get_tables().iter() {
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

        // table area coordinates
        let area_coords = table.get_area();
        let area = format!("{}:{}", area_coords.0, area_coords.1);

        // table start
        let table_no = writer_mng.next_table_no();
        let table_no_str = table_no.to_string();
        let mut attributes = vec![
            ("xmlns", SHEET_MAIN_NS),
            ("id", &table_no_str),
            ("name", table.get_name()),
            ("displayName", table.get_display_name()),
            ("ref", &area),
        ];

        if table.has_totals_row_shown() {
            attributes.push(("totalsRowShown", table.get_totals_row_shown_str()));
        }
        let totals_row_count_str = table.get_totals_row_count_str();
        if table.has_totals_row_count() {
            attributes.push(("totalsRowCount", &totals_row_count_str));
        }
        write_start_tag(&mut writer, "table", attributes, false);

        // autoFilter
        write_start_tag(&mut writer, "autoFilter", vec![("ref", &area)], true);

        // tableColumns
        let cols = table.get_columns();
        write_start_tag(
            &mut writer,
            "tableColumns",
            vec![("count", &cols.len().to_string())],
            false,
        );
        let mut col_id = 1;
        for col in cols.iter() {
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            let col_id_str = col_id.to_string();
            attributes.push(("id", &col_id_str));
            attributes.push(("name", col.get_name()));
            attributes.push(("totalsRowLabel", col.get_totals_row_label_str()));
            attributes.push(("totalsRowFunction", col.get_totals_row_function_str()));
            match col.get_calculated_column_formula() {
                Some(v) => {
                    write_start_tag(&mut writer, "tableColumn", attributes, false);
                    write_start_tag(&mut writer, "calculatedColumnFormula", vec![], false);
                    write_text_node(&mut writer, v);
                    write_end_tag(&mut writer, "calculatedColumnFormula");
                    write_end_tag(&mut writer, "tableColumn");
                }
                None => {
                    write_start_tag(&mut writer, "tableColumn", attributes, true);
                }
            }

            col_id += 1;
        }
        write_end_tag(&mut writer, "tableColumns");

        // tableStyleInfo
        if table.has_style_info() {
            let style_info = table.get_style_info().unwrap();
            write_start_tag(
                &mut writer,
                "tableStyleInfo",
                vec![
                    ("name", style_info.get_name()),
                    (
                        "showFirstColumn",
                        &(style_info.is_show_first_col() as i32).to_string(),
                    ),
                    (
                        "showLastColumn",
                        &(style_info.is_show_last_col() as i32).to_string(),
                    ),
                    (
                        "showRowStripes",
                        &(style_info.is_show_row_stripes() as i32).to_string(),
                    ),
                    (
                        "showColumnStripes",
                        &(style_info.is_show_col_stripes() as i32).to_string(),
                    ),
                ],
                true,
            );
        }

        write_end_tag(&mut writer, "table");

        writer_mng.add_file_at_table(writer, table_no)?;
        table_no_list.push(table_no.to_string());
    }
    Ok(table_no_list)
}
