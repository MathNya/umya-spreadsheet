use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use super::driver::*;

use super::super::structs::worksheet::Worksheet;
use super::super::structs::color::Color;
use super::super::structs::theme::Theme;
use super::super::structs::row_dimension::RowDimension;
use super::super::structs::column_dimension::ColumnDimension;
use super::super::structs::conditional::Conditional;
use super::super::structs::drawing::Drawing;
use super::super::structs::style::Style;
use super::super::structs::page_margins::PageMargins;
use super::super::structs::rich_text::RichText;
use super::super::structs::hyperlink::Hyperlink;

use super::super::helper::coordinate::*;

pub(crate) fn read(
    dir: &TempDir,
    target: &String,
    worksheet: &mut Worksheet,
    theme: &Theme,
    shared_strings: &Vec<(String, Option<RichText>)>,
    cell_xfs_vec: &Vec<Style>,
    dxf_vec: &Vec<Style>
) -> Result<(bool, Option<String>, Option<String>, Vec<(String, String)>), XlsxError> {
    let path = dir.path().join(format!("xl/{}", target));
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    // result
    let mut is_active_sheet = false;
    let mut drawing:Option<String> = None;
    let mut legacy_drawing:Option<String> = None;
    let mut hyperlink_vec: Vec<(String, String)> = Vec::new();

    let mut coordinate: String = String::from("");
    let mut type_value: String = String::from("");
    let mut string_value: String = String::from("");
    let mut style_index: Option<usize> = None;

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
                        let row_index = get_attribute_row(e, &mut row);
                        worksheet.set_row_dimension(row_index, row);
                    },
                    b"c" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"r" => {
                                    coordinate = get_attribute_value(attr)?;
                                },
                                Ok(ref attr) if attr.key == b"s" => {
                                    let value = get_attribute_value(attr)?;
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
                                let mut style = cell_xfs_vec.get(v).unwrap().clone();
                                let coordinate_upper = coordinate.to_uppercase();
                                let split = index_from_coordinate(&coordinate_upper);
                                let col = split[0];
                                let row = split[1];
                                style.set_col_num(&col);
                                style.set_row_num(&row);
                                worksheet.add_style(style);
                            },
                            None => {}
                        }
                        style_index = None;
                    },
                    b"conditionalFormatting" => {
                        let sqref = get_attribute(e, b"sqref").unwrap();
                        let conditional_styles_collection = get_conditional_formatting(&mut reader, dxf_vec, theme);
                        worksheet.add_conditional_styles_collection(sqref, conditional_styles_collection);
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
                        get_attribute_color(e, worksheet, theme);
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
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"min" => {
                                    let value = get_attribute_value(attr).unwrap();
                                    let column_index = value.parse::<usize>().unwrap();
                                    column_dimension.set_column_index(string_from_column_index(&column_index));
                                },
                                Ok(ref attr) if attr.key == b"width" => {
                                    let value = get_attribute_value(attr).unwrap();
                                    column_dimension.set_width(value.parse::<f32>().unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                        worksheet.set_column_dimensions(column_dimension);
                    },
                    b"row" => {
                        let mut row = RowDimension::default();
                        let row_index = get_attribute_row(e, &mut row);
                        worksheet.set_row_dimension(row_index, row);
                    },
                    b"c" => {
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
                                let mut style = cell_xfs_vec.get(v).unwrap().clone();
                                let coordinate_upper = coordinate.to_uppercase();
                                let split = index_from_coordinate(&coordinate_upper);
                                let col = split[0];
                                let row = split[1];
                                style.set_col_num(&col);
                                style.set_row_num(&row);
                                worksheet.add_style(style);
                            },
                            None => {}
                        }
                        style_index = None;
                    },
                    b"autoFilter" => {
                        worksheet.get_auto_filter_mut().set_range(get_attribute(e, b"ref").unwrap());
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
                        worksheet.add_drawing(Drawing::default());
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
                    b"f" => worksheet.get_cell_mut(&coordinate.to_string()).set_formula_attributes(string_value.clone()),
                    b"v" => {
                        if type_value == "s" {
                            let index = string_value.parse::<usize>().unwrap();
                            let (value, rich_text) = shared_strings.get(index).unwrap();
                            let _ = worksheet.get_cell_mut(&coordinate.to_string()).set_all_param(value, rich_text.clone(), &type_value, &"".into());
                        } else if type_value == "b" {
                            let prm = if &string_value == "1" {"TRUE"} else {"FALSE"};
                            let _ = worksheet.get_cell_mut(&coordinate.to_string()).set_value_and_data_type(prm, &type_value);
                        } else if type_value == "" {
                            let _ = worksheet.get_cell_mut(&coordinate.to_string()).set_value_crate(&string_value);
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
    dxf_vec: &Vec<Style>,
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
                                Ok(ref attr) if attr.key == b"type" => conditional.set_condition_type(get_attribute_value(attr).unwrap()),
                                Ok(ref attr) if attr.key == b"dxfId" => {
                                    let dxf_id = get_attribute_value(attr).unwrap().parse::<usize>().unwrap();
                                    conditional.set_style(dxf_vec.get(dxf_id).unwrap().clone());
                                },
                                Ok(ref attr) if attr.key == b"priority" => conditional.set_priority(get_attribute_value(attr).unwrap().parse::<usize>().unwrap()),
                                Ok(ref attr) if attr.key == b"percent" => conditional.set_percent(get_attribute_value(attr).unwrap().parse::<usize>().unwrap()),
                                Ok(ref attr) if attr.key == b"bottom" => conditional.set_bottom(get_attribute_value(attr).unwrap().parse::<usize>().unwrap()),
                                Ok(ref attr) if attr.key == b"rank" => conditional.set_rank(get_attribute_value(attr).unwrap().parse::<usize>().unwrap()),
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
                                Ok(ref attr) if attr.key == b"type" => conditional.set_condition_type(get_attribute_value(attr).unwrap()),
                                Ok(ref attr) if attr.key == b"dxfId" => {
                                    let dxf_id = get_attribute_value(attr).unwrap().parse::<usize>().unwrap();
                                    conditional.set_style(dxf_vec.get(dxf_id).unwrap().clone());
                                },
                                Ok(ref attr) if attr.key == b"priority" => conditional.set_priority(get_attribute_value(attr).unwrap().parse::<usize>().unwrap()),
                                Ok(ref attr) if attr.key == b"percent" => conditional.set_percent(get_attribute_value(attr).unwrap().parse::<usize>().unwrap()),
                                Ok(ref attr) if attr.key == b"bottom" => conditional.set_bottom(get_attribute_value(attr).unwrap().parse::<usize>().unwrap()),
                                Ok(ref attr) if attr.key == b"rank" => conditional.set_rank(get_attribute_value(attr).unwrap().parse::<usize>().unwrap()),
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
    let color_map = theme.get_color_map();

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
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"theme" => {
                                    let value = get_attribute_value(attr).unwrap();
                                    let _ = color.set_theme_index(value.parse::<usize>().unwrap(), color_map);
                                },
                                Ok(ref attr) if attr.key == b"rgb" => {
                                    let _ = color.set_argb(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"tint" => {
                                    let value = get_attribute_value(attr).unwrap();
                                    let _ = color.set_tint(value.parse::<f64>().unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
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
)-> usize {
    let mut row_index:usize = 0;
    for a in e.attributes().with_checks(false) {
        match a {
            Ok(ref attr) if attr.key == b"r" => row_index = get_attribute_value(attr).unwrap().parse::<usize>().unwrap(),
            Ok(ref attr) if attr.key == b"ht" => row.set_height(get_attribute_value(attr).unwrap().parse::<f32>().unwrap()),
            Ok(ref attr) if attr.key == b"thickBot" => row.set_thick_bot(get_attribute_value(attr).unwrap() == "1"),
            Ok(ref attr) if attr.key == b"customHeight" => row.set_custom_height(get_attribute_value(attr).unwrap() == "1"),
            Ok(ref attr) if attr.key == b"x14ac:dyDescent" => row.set_descent(get_attribute_value(attr).unwrap().parse::<f32>().unwrap()),
            Ok(_) => {},
            Err(_) => {},
        }
    }
    row_index
}

fn get_attribute_color(
    e:&quick_xml::events::BytesStart<'_>,
    worksheet: &mut Worksheet,
    theme:&Theme
) {
    let theme_color_map:Vec<String> = theme.get_color_map().clone();

    for a in e.attributes().with_checks(false) {
        match a {
            Ok(ref attr) if attr.key == b"theme" => {
                let value = get_attribute_value(attr).unwrap();
                let _ = worksheet.get_tab_color_mut().set_theme_index(value.parse::<usize>().unwrap(), &theme_color_map);
            },
            Ok(ref attr) if attr.key == b"rgb" => {
                let _ = worksheet.get_tab_color_mut().set_argb(get_attribute_value(attr).unwrap());
            },
            Ok(ref attr) if attr.key == b"tint" => {
                let value = get_attribute_value(attr).unwrap();
                let _ = worksheet.get_tab_color_mut().set_tint(value.parse::<f64>().unwrap());
            },
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
