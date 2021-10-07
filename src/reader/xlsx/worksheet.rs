use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use super::driver::*;

use ::structs::Spreadsheet;
use ::structs::Color;
use ::structs::Theme;
use ::structs::RowDimension;
use ::structs::ColumnDimension;
use ::structs::Conditional;
use ::structs::Stylesheet;
use ::structs::PageMargins;
use ::structs::Hyperlink;
use ::structs::ConditionalSet;

pub(crate) fn read(
    dir: &TempDir,
    target: &String,
    spreadsheet: &mut Spreadsheet,
    sheets_sheet_id: &str,
    sheets_name: &str,
) -> Result<(bool, Option<String>, Option<String>, Vec<(String, String)>), XlsxError> {
    
    let path = dir.path().join(format!("xl/{}", target));
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    let theme = spreadsheet.get_theme_mut().clone();
    let shared_string_table = spreadsheet.get_shared_string_table().clone();
    let stylesheet = spreadsheet.get_stylesheet().clone();

    let worksheet = spreadsheet.new_sheet_crate(sheets_sheet_id, sheets_name);

    // result
    let mut is_active_sheet = false;
    let mut drawing:Option<String> = None;
    let mut legacy_drawing:Option<String> = None;
    let mut hyperlink_vec: Vec<(String, String)> = Vec::new();

    let mut coordinate: String = String::from("");
    let mut type_value: String = String::from("");
    let mut string_value: String = String::from("");

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"sheetPr" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"codeName" => {
                                    worksheet.set_code_name(get_attribute_value(attr)?);
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    b"sheetView" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"tabSelected" => {
                                    is_active_sheet = true;
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    b"selection" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"activeCell" => {
                                    worksheet.set_active_cell(get_attribute_value(attr)?);
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    b"row" => {
                        let mut row = RowDimension::default();
                        get_attribute_row(e, &mut row);
                        worksheet.set_row_dimension(row);
                    },
                    b"c" => {
                        let mut style_index: Option<usize> = None;
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"r" => {
                                    coordinate = get_attribute_value(attr)?;
                                },
                                Ok(ref attr) if attr.key == b"s" => {
                                    let value = get_attribute_value(attr).unwrap();
                                    style_index = Some(value.parse::<usize>().unwrap());
                                },
                                Ok(ref attr) if attr.key == b"t" => {
                                    type_value = get_attribute_value(attr)?;
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                        match style_index {
                            Some(v) => {
                                let mut style = stylesheet.get_style(v);
                                style.get_coordinate_mut().set_coordinate(&coordinate);
                                worksheet.add_style(style);
                            },
                            None => {}
                        }
                    },
                    b"conditionalFormatting" => {
                        let mut conditional_set = ConditionalSet::default();
                        let sqref = get_attribute(e, b"sqref").unwrap();
                        conditional_set.set_sqref(sqref);
                        let conditional_styles_collection = get_conditional_formatting(&mut reader, &stylesheet, &theme);
                        conditional_set.set_conditional_collection(conditional_styles_collection);
                        worksheet.add_conditional_styles_collection(conditional_set);
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"sheetPr" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"codeName" => {
                                    worksheet.set_code_name(get_attribute_value(attr)?);
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    b"tabColor" => {
                        worksheet.get_tab_color_mut().set_attributes(&mut reader, e);
                        worksheet.get_tab_color_mut().set_argb_by_theme(&theme);
                    },
                    b"selection" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"activeCell" => {
                                    worksheet.set_active_cell(get_attribute_value(attr)?);
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    b"col" => {
                        let mut column_dimension = ColumnDimension::default();
                        let min = get_attribute(e, b"min").unwrap().parse::<usize>().unwrap();
                        column_dimension.set_col_num_start(min);

                        let max = get_attribute(e, b"max").unwrap().parse::<usize>().unwrap();
                        column_dimension.set_col_num_end(max);

                        let width = get_attribute(e, b"width").unwrap().parse::<f32>().unwrap();
                        column_dimension.set_width(width);

                        let best_fit =  match get_attribute(e, b"bestFit") {
                            Some(v) => { if v.as_str() == "1" { true } else { false } },
                            None => { false }
                        };
                        column_dimension.set_best_fit(best_fit);
                        worksheet.set_column_dimensions(column_dimension);
                    },
                    b"row" => {
                        let mut row = RowDimension::default();
                        get_attribute_row(e, &mut row);
                        worksheet.set_row_dimension(row);
                    },
                    b"c" => {
                        let mut style_index: Option<usize> = None;
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"r" => {
                                    coordinate = get_attribute_value(attr)?;
                                },
                                Ok(ref attr) if attr.key == b"s" => {
                                    let value = get_attribute_value(attr)?;
                                    style_index = Some(value.parse::<usize>().unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                        match style_index {
                            Some(v) => {
                                let mut style = stylesheet.get_style(v);
                                style.get_coordinate_mut().set_coordinate(&coordinate);
                                worksheet.add_style(style);
                            },
                            None => {}
                        }
                    },
                    b"autoFilter" => {
                        worksheet.set_auto_filter(get_attribute(e, b"ref").unwrap());
                    },
                    b"mergeCell" => {
                        worksheet.add_merge_cells_crate(get_attribute(e, b"ref").unwrap());
                    },
                    b"pageMargins" => {
                        let mut page_margins:PageMargins = PageMargins::default();
                        let left = get_attribute(e, b"left").unwrap();
                        let right = get_attribute(e, b"right").unwrap();
                        let top = get_attribute(e, b"top").unwrap();
                        let bottom = get_attribute(e, b"bottom").unwrap();
                        let header = get_attribute(e, b"header").unwrap();
                        let footer = get_attribute(e, b"footer").unwrap();
                        page_margins.set_left(left.parse::<f32>().unwrap());
                        page_margins.set_right(right.parse::<f32>().unwrap());
                        page_margins.set_top(top.parse::<f32>().unwrap());
                        page_margins.set_bottom(bottom.parse::<f32>().unwrap());
                        page_margins.set_header(header.parse::<f32>().unwrap());
                        page_margins.set_footer(footer.parse::<f32>().unwrap());
                        worksheet.set_page_margins(page_margins);
                    },
                    b"drawing" => {
                        drawing = Some(get_attribute(e, b"r:id").unwrap());
                    },
                    b"legacyDrawing" => {
                        legacy_drawing = Some(get_attribute(e, b"r:id").unwrap());
                    },
                    b"hyperlink" => {
                        let (coor, rid, hyperlink) = get_hyperlink(e);
                        let _ = worksheet.get_cell_mut(&coor.to_string()).set_hyperlink(hyperlink);
                        if &rid != "" {
                            hyperlink_vec.push((coor, rid));
                        }
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => string_value = e.unescape_and_decode(&reader).unwrap(),
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"f" => {
                        worksheet.get_cell_mut(&coordinate.to_string()).set_formula(string_value.clone());
                    },
                    b"v" => {
                        if type_value == "s" {
                            let index = string_value.parse::<usize>().unwrap();
                            let shared_string_item = shared_string_table.get_shared_string_item().get(index).unwrap();
                            worksheet.get_cell_mut(&coordinate.to_string()).set_shared_string_item(shared_string_item.clone());
                        } else if type_value == "b" {
                            let prm = if &string_value == "1" {true} else {false};
                            let _ = worksheet.get_cell_mut(&coordinate.to_string()).set_value_from_bool(prm);
                        } else if type_value == "" || type_value == "n" {
                            let _ = worksheet.get_cell_mut(&coordinate.to_string()).set_value(&string_value);
                        };
                    },
                    b"c" => type_value = String::from(""),
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok((is_active_sheet, drawing, legacy_drawing, hyperlink_vec))
}

fn get_conditional_formatting(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    stylesheet: &Stylesheet,
    theme: &Theme
) -> Vec<Conditional>
{
    let mut buf = Vec::new();
    let mut conditional_vec: Vec<Conditional> = Vec::new();

    let mut conditional = Conditional::default();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"cfRule" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"type" => {conditional.set_condition_type(get_attribute_value(attr).unwrap());},
                                Ok(ref attr) if attr.key == b"dxfId" => {
                                    let dxf_id = get_attribute_value(attr).unwrap().parse::<usize>().unwrap();
                                    let style = stylesheet.get_differential_formats().get_style(dxf_id);
                                    conditional.set_style(style);
                                },
                                Ok(ref attr) if attr.key == b"priority" => {conditional.set_priority(get_attribute_value(attr).unwrap().parse::<usize>().unwrap());},
                                Ok(ref attr) if attr.key == b"percent" => {conditional.set_percent(get_attribute_value(attr).unwrap().parse::<usize>().unwrap());},
                                Ok(ref attr) if attr.key == b"bottom" => {conditional.set_bottom(get_attribute_value(attr).unwrap().parse::<usize>().unwrap());},
                                Ok(ref attr) if attr.key == b"rank" => {conditional.set_rank(get_attribute_value(attr).unwrap().parse::<usize>().unwrap());},
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                        conditional_vec.push(conditional);
                        conditional = Conditional::default();
                    },
                    _ => (),
                }
            },
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"cfRule" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"type" => {conditional.set_condition_type(get_attribute_value(attr).unwrap());},
                                Ok(ref attr) if attr.key == b"dxfId" => {
                                    let dxf_id = get_attribute_value(attr).unwrap().parse::<usize>().unwrap();
                                    let style = stylesheet.get_differential_formats().get_style(dxf_id);
                                    conditional.set_style(style);
                                },
                                Ok(ref attr) if attr.key == b"priority" => {conditional.set_priority(get_attribute_value(attr).unwrap().parse::<usize>().unwrap());},
                                Ok(ref attr) if attr.key == b"percent" => {conditional.set_percent(get_attribute_value(attr).unwrap().parse::<usize>().unwrap());},
                                Ok(ref attr) if attr.key == b"bottom" => {conditional.set_bottom(get_attribute_value(attr).unwrap().parse::<usize>().unwrap());},
                                Ok(ref attr) if attr.key == b"rank" => {conditional.set_rank(get_attribute_value(attr).unwrap().parse::<usize>().unwrap());},
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    b"dataBar" => {
                        conditional.set_data_type("dataBar");
                        conditional.set_cfvo_collection(get_cfvo(reader, theme));
                    },
                    b"colorScale" => {
                        conditional.set_data_type("colorScale");
                        conditional.set_cfvo_collection(get_cfvo(reader, theme));
                    },
                    b"iconSet" => {
                        conditional.set_data_type("iconSet");
                        conditional.set_cfvo_collection(get_cfvo(reader, theme));
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"conditionalFormatting" => {
                        return conditional_vec;
                    },
                    b"cfRule" => {
                        conditional_vec.push(conditional);
                        conditional = Conditional::default();
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "conditionalFormatting"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_cfvo(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    theme: &Theme
)->Vec<(String, Option<String>, Option<Color>)>
{
    let mut buf = Vec::new();
    let mut cfvo: Vec<(String, Option<String>)> = Vec::new();
    let mut result: Vec<(String, Option<String>, Option<Color>)> = Vec::new();

    let mut r#type: String = String::from("");
    let mut value: Option<String> = None;
    
    let mut color_count = 0;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"cfvo" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"type" => r#type = get_attribute_value(attr).unwrap(),
                                Ok(ref attr) if attr.key == b"value" => value = Some(get_attribute_value(attr).unwrap()),
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                        cfvo.push((r#type, value));
                        r#type = String::from("");
                        value = None;
                    },
                    b"color" => {
                        let mut color = Color::default();
                        color.set_attributes(reader, e);
                        color.set_argb_by_theme(theme);
                        
                        let (t, v) = cfvo.get(color_count).unwrap();
                        result.insert(color_count, (t.clone(), v.clone(), Some(color)));
                        color_count += 1;
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"dataBar" => return result,
                    b"colorScale" => return result,
                    b"iconSet" => return result,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "cfRule"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_attribute_row(
    e:&quick_xml::events::BytesStart<'_>, 
    row:&mut RowDimension
) {
    for a in e.attributes().with_checks(false) {
        match a {
            Ok(ref attr) if attr.key == b"r" => row.set_row_num(&get_attribute_value(attr).unwrap().parse::<usize>().unwrap()),
            Ok(ref attr) if attr.key == b"ht" => row.set_height(get_attribute_value(attr).unwrap().parse::<f32>().unwrap()),
            Ok(ref attr) if attr.key == b"thickBot" => row.set_thick_bot(get_attribute_value(attr).unwrap() == "1"),
            Ok(ref attr) if attr.key == b"customHeight" => row.set_custom_height(get_attribute_value(attr).unwrap() == "1"),
            Ok(ref attr) if attr.key == b"x14ac:dyDescent" => row.set_descent(get_attribute_value(attr).unwrap().parse::<f32>().unwrap()),
            Ok(_) => {},
            Err(_) => {},
        }
    }
}

fn get_hyperlink(e:&quick_xml::events::BytesStart<'_>,)->(String, String, Hyperlink) {

    let mut hyperlink = Hyperlink::default();
    let mut rid = String::from("");

    let coordition = get_attribute(e, b"ref").unwrap();
    match get_attribute(e, b"location") {
        Some(v) => {
            let _ = hyperlink.set_url(v);
            let _ = hyperlink.set_location(true);
        },
        None => {
            rid = get_attribute(e, b"r:id").unwrap();
        }
    }
    (coordition, rid, hyperlink)
}
