use super::driver::*;
use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;

use structs::drawing::Theme;
use structs::raw::RawWorksheet;
use structs::Color;
use structs::Columns;
use structs::Conditional;
use structs::ConditionalSet;
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
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"sheetPr" => {
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) if attr.key == b"codeName" => {
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
                            Ok(ref attr) if attr.key == b"activeCell" => {
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
                    let mut conditional_set = ConditionalSet::default();
                    let sqref = get_attribute(e, b"sqref").unwrap();
                    conditional_set
                        .get_sequence_of_references_mut()
                        .set_sqref(sqref);
                    let conditional_styles_collection =
                        get_conditional_formatting(&mut reader, stylesheet, theme);
                    conditional_set.set_conditional_collection(conditional_styles_collection);
                    worksheet.add_conditional_styles_collection(conditional_set);
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
            Ok(Event::Empty(ref e)) => match e.name() {
                b"sheetPr" => {
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) if attr.key == b"codeName" => {
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
                            Ok(ref attr) if attr.key == b"activeCell" => {
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
                    let _ = worksheet
                        .get_cell_mut(&coor.to_string())
                        .set_hyperlink(hyperlink);
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

fn get_conditional_formatting<R: std::io::BufRead>(
    reader: &mut Reader<R>,
    stylesheet: &Stylesheet,
    theme: &Theme,
) -> Vec<Conditional> {
    let mut buf = Vec::new();
    let mut conditional_vec: Vec<Conditional> = Vec::new();

    let mut conditional = Conditional::default();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => match e.name() {
                b"cfRule" => {
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) if attr.key == b"type" => {
                                conditional.set_condition_type(get_attribute_value(attr).unwrap());
                            }
                            Ok(ref attr) if attr.key == b"dxfId" => {
                                let dxf_id =
                                    get_attribute_value(attr).unwrap().parse::<usize>().unwrap();
                                let style = stylesheet.get_differential_formats().get_style(dxf_id);
                                conditional.set_style(style);
                            }
                            Ok(ref attr) if attr.key == b"priority" => {
                                conditional.set_priority(
                                    get_attribute_value(attr).unwrap().parse::<usize>().unwrap(),
                                );
                            }
                            Ok(ref attr) if attr.key == b"percent" => {
                                conditional.set_percent(
                                    get_attribute_value(attr).unwrap().parse::<usize>().unwrap(),
                                );
                            }
                            Ok(ref attr) if attr.key == b"bottom" => {
                                conditional.set_bottom(
                                    get_attribute_value(attr).unwrap().parse::<usize>().unwrap(),
                                );
                            }
                            Ok(ref attr) if attr.key == b"rank" => {
                                conditional.set_rank(
                                    get_attribute_value(attr).unwrap().parse::<usize>().unwrap(),
                                );
                            }
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                    conditional_vec.push(conditional);
                    conditional = Conditional::default();
                }
                _ => (),
            },
            Ok(Event::Start(ref e)) => match e.name() {
                b"cfRule" => {
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) if attr.key == b"type" => {
                                conditional.set_condition_type(get_attribute_value(attr).unwrap());
                            }
                            Ok(ref attr) if attr.key == b"dxfId" => {
                                let dxf_id =
                                    get_attribute_value(attr).unwrap().parse::<usize>().unwrap();
                                let style = stylesheet.get_differential_formats().get_style(dxf_id);
                                conditional.set_style(style);
                            }
                            Ok(ref attr) if attr.key == b"priority" => {
                                conditional.set_priority(
                                    get_attribute_value(attr).unwrap().parse::<usize>().unwrap(),
                                );
                            }
                            Ok(ref attr) if attr.key == b"percent" => {
                                conditional.set_percent(
                                    get_attribute_value(attr).unwrap().parse::<usize>().unwrap(),
                                );
                            }
                            Ok(ref attr) if attr.key == b"bottom" => {
                                conditional.set_bottom(
                                    get_attribute_value(attr).unwrap().parse::<usize>().unwrap(),
                                );
                            }
                            Ok(ref attr) if attr.key == b"rank" => {
                                conditional.set_rank(
                                    get_attribute_value(attr).unwrap().parse::<usize>().unwrap(),
                                );
                            }
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                }
                b"dataBar" => {
                    conditional.set_data_type("dataBar");
                    conditional.set_cfvo_collection(get_cfvo(reader, theme));
                }
                b"colorScale" => {
                    conditional.set_data_type("colorScale");
                    conditional.set_cfvo_collection(get_cfvo(reader, theme));
                }
                b"iconSet" => {
                    conditional.set_data_type("iconSet");
                    conditional.set_cfvo_collection(get_cfvo(reader, theme));
                }
                _ => (),
            },
            Ok(Event::End(ref e)) => match e.name() {
                b"conditionalFormatting" => {
                    return conditional_vec;
                }
                b"cfRule" => {
                    conditional_vec.push(conditional);
                    conditional = Conditional::default();
                }
                _ => (),
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "conditionalFormatting"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_cfvo<R: std::io::BufRead>(
    reader: &mut Reader<R>,
    theme: &Theme,
) -> Vec<(String, Option<String>, Option<Color>)> {
    let mut buf = Vec::new();
    let mut cfvo: Vec<(String, Option<String>)> = Vec::new();
    let mut result: Vec<(String, Option<String>, Option<Color>)> = Vec::new();

    let mut r#type: String = String::from("");
    let mut value: Option<String> = None;

    let mut color_count = 0;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => match e.name() {
                b"cfvo" => {
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) if attr.key == b"type" => {
                                r#type = get_attribute_value(attr).unwrap()
                            }
                            Ok(ref attr) if attr.key == b"value" => {
                                value = Some(get_attribute_value(attr).unwrap())
                            }
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                    cfvo.push((r#type, value));
                    r#type = String::from("");
                    value = None;
                }
                b"color" => {
                    let mut color = Color::default();
                    color.set_attributes(reader, e);
                    color.set_argb_by_theme(theme);

                    let (t, v) = cfvo.get(color_count).unwrap();
                    result.insert(color_count, (t.clone(), v.clone(), Some(color)));
                    color_count += 1;
                }
                _ => (),
            },
            Ok(Event::Start(ref e)) => match e.name() {
                b"cfvo" => {
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) if attr.key == b"type" => {
                                r#type = get_attribute_value(attr).unwrap()
                            }
                            Ok(ref attr) if attr.key == b"value" => {
                                value = Some(get_attribute_value(attr).unwrap())
                            }
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                    cfvo.push((r#type, value));
                    r#type = String::from("");
                    value = None;
                }
                b"color" => {
                    let mut color = Color::default();
                    color.set_attributes(reader, e);
                    color.set_argb_by_theme(theme);

                    let (t, v) = cfvo.get(color_count).unwrap();
                    result.insert(color_count, (t.clone(), v.clone(), Some(color)));
                    color_count += 1;
                }
                _ => (),
            },
            Ok(Event::End(ref e)) => match e.name() {
                b"dataBar" => return result,
                b"colorScale" => return result,
                b"iconSet" => return result,
                _ => (),
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "cfRule"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
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
