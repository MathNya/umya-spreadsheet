use std::io;

use quick_xml::{
    events::{BytesDecl, Event},
    Writer,
};
use structs::{Worksheet, WriterManager};

use super::{driver::*, XlsxError};

pub(crate) fn write<W: io::Seek + io::Write>(worksheet: &Worksheet,
                                             writer_mng: &mut WriterManager<W>)
                                             -> Result<Vec<String>, XlsxError>
{
    let mut table_no_list = Vec::<String>::new();
    for table in worksheet.get_tables().iter()
    {
        let mut writer = Writer::new(io::Cursor::new(Vec::new()));

        // XML header
        let _ = writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), Some("yes"))));
        write_new_line(&mut writer);

        // table area coordinates
        let area_coords = table.get_area();
        let area = format!("{}:{}",
                           area_coords.0.get_coordinate(),
                           area_coords.1.get_coordinate());

        // table start
        let table_no = writer_mng.next_table_no();
        write_start_tag(&mut writer,
                        "table",
                        vec![("xmlns",
                              "http://schemas.openxmlformats.org/spreadsheetml/2006/main"),
                             ("id", &table_no.to_string()),
                             ("name", table.get_name()),
                             ("displayName", table.get_display_name()),
                             ("ref", &area),
                             ("totalsRowShown", "0"),],
                        false);

        // autoFilter
        write_start_tag(&mut writer, "autoFilter", vec![("ref", &area)], true);

        // tableColumns
        let cols = table.get_columns();
        write_start_tag(&mut writer,
                        "tableColumns",
                        vec![("count", &cols.len().to_string())],
                        false);
        let mut col_id = 1;
        for col in cols.iter()
        {
            write_start_tag(&mut writer,
                            "tableColumn",
                            vec![("id", &col_id.to_string()), ("name", col.get_name())],
                            true);
            col_id += 1;
        }
        write_end_tag(&mut writer, "tableColumns");

        // tableStyleInfo
        if table.has_style_info()
        {
            let style_info = table.get_style_info().unwrap();
            write_start_tag(&mut writer,
                            "tableStyleInfo",
                            vec![("name", style_info.get_name()),
                                 ("showFirstColumn",
                                  &(style_info.is_show_first_col() as i32).to_string()),
                                 ("showLastColumn",
                                  &(style_info.is_show_last_col() as i32).to_string()),
                                 ("showRowStripes",
                                  &(style_info.is_show_row_stripes() as i32).to_string()),
                                 ("showColumnStripes",
                                  &(style_info.is_show_col_stripes() as i32).to_string()),],
                            true);
        }

        write_end_tag(&mut writer, "table");

        writer_mng.add_file_at_table(writer, table_no)?;
        table_no_list.push(table_no.to_string());
    }
    Ok(table_no_list)
}
