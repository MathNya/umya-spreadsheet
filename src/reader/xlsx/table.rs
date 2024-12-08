use super::driver::*;
use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::result;
use crate::structs::raw::RawFile;
use crate::structs::Comment;
use crate::structs::Worksheet;
use crate::structs::{Table, TableColumn, TableStyleInfo};

pub(crate) fn read(
    worksheet: &mut Worksheet,
    table_file: &RawFile,
) -> result::Result<(), XlsxError> {
    let data = std::io::Cursor::new(table_file.get_file_data());
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(false);
    let mut buf = Vec::new();
    let mut table = Table::default();
    let mut table_column = TableColumn::default();
    let mut string_value = String::from("");
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                b"tableColumn" => {
                    table_column = TableColumn::default();
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) => match attr.key.0 {
                                b"name" => {
                                    let attr_val = get_attribute_value(attr)?;
                                    table_column.set_name(attr_val);
                                }
                                b"totalsRowLabel" => {
                                    let attr_val = get_attribute_value(attr)?;
                                    table_column.set_totals_row_label_str(&attr_val);
                                }
                                b"totalsRowFunction" => {
                                    let attr_val = get_attribute_value(attr)?;
                                    table_column.set_totals_row_function_str(&attr_val);
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                    // add column to table (if it has a name)
                    if !table_column.get_name().is_empty() {
                        table.add_column(table_column);
                        table_column = TableColumn::default();
                    }
                }
                b"tableStyleInfo" => {
                    let mut name = String::new();
                    let mut show_first_col = false;
                    let mut show_last_col = false;
                    let mut show_row_stripes = false;
                    let mut show_col_stripes = false;
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) => {
                                let attr_val = get_attribute_value(attr)?;
                                match attr.key.0 {
                                    b"name" => {
                                        name = attr_val;
                                    }
                                    b"showFirstColumn" => {
                                        show_first_col = attr_val == "1";
                                    }
                                    b"showLastColumn" => {
                                        show_last_col = attr_val == "1";
                                    }
                                    b"showRowStripes" => {
                                        show_row_stripes = attr_val == "1";
                                    }
                                    b"showColumnStripes" => {
                                        show_col_stripes = attr_val == "1";
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    if !name.is_empty() {
                        table.set_style_info(Some(TableStyleInfo::new(
                            &name,
                            show_first_col,
                            show_last_col,
                            show_row_stripes,
                            show_col_stripes,
                        )));
                    }
                }
                _ => (),
            },
            Ok(Event::Text(e)) => string_value = e.unescape().unwrap().to_string(),
            Ok(Event::Start(ref e)) => match e.name().into_inner() {
                b"table" => {
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) => {
                                let attr_val = get_attribute_value(attr)?;
                                match attr.key.0 {
                                    b"displayName" => {
                                        table.set_display_name(&attr_val);
                                    }
                                    b"name" => {
                                        table.set_name(&attr_val);
                                    }
                                    b"ref" => {
                                        let area_coords: Vec<&str> = attr_val.split(':').collect();
                                        if area_coords.len() == 2 {
                                            table.set_area((area_coords[0], area_coords[1]));
                                        }
                                    }
                                    b"totalsRowShown" => {
                                        table.set_totals_row_shown_str(&attr_val);
                                    }
                                    b"totalsRowCount" => {
                                        table.set_totals_row_count_str(&attr_val);
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
                b"tableColumn" => {
                    table_column = TableColumn::default();
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) => match attr.key.0 {
                                b"name" => {
                                    let attr_val = get_attribute_value(attr)?;
                                    table_column.set_name(attr_val);
                                }
                                b"totalsRowLabel" => {
                                    let attr_val = get_attribute_value(attr)?;
                                    table_column.set_totals_row_label_str(&attr_val);
                                }
                                b"totalsRowFunction" => {
                                    let attr_val = get_attribute_value(attr)?;
                                    table_column.set_totals_row_function_str(&attr_val);
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
                _ => (),
            },
            Ok(Event::End(ref e)) => match e.name().into_inner() {
                b"calculatedColumnFormula" => {
                    table_column.set_calculated_column_formula(string_value);
                    string_value = String::from("");
                }
                b"tableColumn" => {
                    // add column to table (if it has a name)
                    if !table_column.get_name().is_empty() {
                        table.add_column(table_column);
                        table_column = TableColumn::default();
                    }
                }
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    // add the table to the sheet (if a few sanity checks pass)
    if table.is_ok() {
        worksheet.add_table(table);
    }
    Ok(())
}
