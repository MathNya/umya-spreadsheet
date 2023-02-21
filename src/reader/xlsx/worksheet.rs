use super::driver::*;
use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;

use structs::drawing::Theme;
use structs::raw::RawWorksheet;
use structs::Columns;
use structs::ConditionalFormatting;
use structs::Hyperlink;
use structs::OleObjects;
use structs::Row;
use structs::SharedStringTable;
use structs::Stylesheet;
use structs::Worksheet;

pub(crate) fn read(
    worksheet: &mut Worksheet,
    raw_data_of_worksheet: &RawWorksheet,
    theme: &Theme,
    shared_string_table: &SharedStringTable,
    stylesheet: &Stylesheet,
) -> Result<(), XlsxError> {
    let data = std::io::Cursor::new(raw_data_of_worksheet.get_worksheet_file().get_file_data());
    let mut reader = Reader::from_reader(data);
    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name().into_inner() {
                b"sheetPr" => {
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) if attr.key.0 == b"codeName" => {
                                worksheet.set_code_name(get_attribute_value(attr)?);
                            }
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                }
                b"sheetViews" => {
                    worksheet
                        .get_sheet_views_mut()
                        .set_attributes(&mut reader, e);
                }
                b"selection" => {
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) if attr.key.0 == b"activeCell" => {
                                worksheet.set_active_cell(get_attribute_value(attr)?);
                            }
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                }
                b"row" => {
                    let mut obj = Row::default();
                    obj.set_attributes(
                        &mut reader,
                        e,
                        worksheet,
                        shared_string_table,
                        stylesheet,
                        false,
                    );
                    worksheet.set_row_dimension(obj);
                }
                b"cols" => {
                    let mut obj = Columns::default();
                    obj.set_attributes(&mut reader, e, stylesheet);
                    worksheet.set_column_dimensions_crate(obj);
                }
                b"mergeCells" => {
                    worksheet
                        .get_merge_cells_crate_mut()
                        .set_attributes(&mut reader, e);
                }
                b"conditionalFormatting" => {
                    let mut obj = ConditionalFormatting::default();
                    obj.set_attributes(&mut reader, e, stylesheet.get_differential_formats());
                    worksheet.add_conditional_formatting_collection(obj);
                }
                b"oleObjects" => {
                    let mut obj = OleObjects::default();
                    obj.set_attributes(
                        &mut reader,
                        e,
                        raw_data_of_worksheet.get_worksheet_relationships().unwrap(),
                    );
                    worksheet.set_ole_objects(obj);
                }
                b"headerFooter" => {
                    worksheet
                        .get_header_footer_mut()
                        .set_attributes(&mut reader, e);
                }
                b"rowBreaks" => {
                    worksheet
                        .get_row_breaks_mut()
                        .set_attributes(&mut reader, e);
                }
                b"colBreaks" => {
                    worksheet
                        .get_column_breaks_mut()
                        .set_attributes(&mut reader, e);
                }
                _ => (),
            },
            Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                b"sheetPr" => {
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) if attr.key.0 == b"codeName" => {
                                worksheet.set_code_name(get_attribute_value(attr)?);
                            }
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                }
                b"tabColor" => {
                    worksheet.get_tab_color_mut().set_attributes(&mut reader, e);
                    worksheet.get_tab_color_mut().set_argb_by_theme(theme);
                }
                b"selection" => {
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) if attr.key.0 == b"activeCell" => {
                                worksheet.set_active_cell(get_attribute_value(attr)?);
                            }
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                }
                b"row" => {
                    let mut obj = Row::default();
                    obj.set_attributes(
                        &mut reader,
                        e,
                        worksheet,
                        shared_string_table,
                        stylesheet,
                        true,
                    );
                    worksheet.set_row_dimension(obj);
                }
                b"autoFilter" => {
                    worksheet.set_auto_filter(get_attribute(e, b"ref").unwrap());
                }
                b"pageMargins" => {
                    worksheet
                        .get_page_margins_mut()
                        .set_attributes(&mut reader, e);
                }
                b"hyperlink" => {
                    let (coor, _rid, hyperlink) = get_hyperlink(e);
                    let _ = worksheet.get_cell_mut(coor).set_hyperlink(hyperlink);
                }
                b"printOptions" => {
                    worksheet
                        .get_print_options_mut()
                        .set_attributes(&mut reader, e);
                }
                b"pageSetup" => {
                    worksheet.get_page_setup_mut().set_attributes(
                        &mut reader,
                        e,
                        raw_data_of_worksheet.get_worksheet_relationships(),
                    );
                }
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok(())
}

fn get_hyperlink(e: &quick_xml::events::BytesStart<'_>) -> (String, String, Hyperlink) {
    let mut hyperlink = Hyperlink::default();
    let mut rid = String::from("");

    let coordition = get_attribute(e, b"ref").unwrap_or_default();
    match get_attribute(e, b"location") {
        Some(v) => {
            let _ = hyperlink.set_url(v);
            let _ = hyperlink.set_location(true);
        }
        None => {
            rid = get_attribute(e, b"r:id").unwrap_or_default();
        }
    }
    (coordition, rid, hyperlink)
}
