use std::result;
use std::collections::HashMap;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use super::driver::*;

use super::super::structs::theme::Theme;
use super::super::structs::number_format::NumberFormat;
use super::super::structs::spreadsheet::Spreadsheet;
use super::super::structs::borders::Borders;
use super::super::structs::border::Border;
use super::super::structs::fill::Fill;
use super::super::structs::font::Font;
use super::super::structs::style::Style;
use super::super::structs::color::Color;
use super::super::structs::alignment::Alignment;
use super::super::structs::cell_style::CellStyle;

const FILE_PATH: &'static str = "xl/styles.xml";

pub fn read(dir: &TempDir, spreadsheet:&mut Spreadsheet, theme:&Theme) -> result::Result<(Vec<Style>, Vec<Style>), XlsxError>
{
    let path = dir.path().join(FILE_PATH);
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut num_fmt_vec: HashMap<usize, NumberFormat> = HashMap::new();
    let mut font_vec: Vec<Font> = Vec::new();
    let mut fill_vec: Vec<Fill> = Vec::new();
    let mut borders_vec: Vec<Borders> = Vec::new();
    let mut cell_xfs: Vec<Style> = Vec::new();
    let mut dxf_vec: Vec<Style> = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"numFmts" => {
                        num_fmt_vec = get_num_fmts(&mut reader);
                    },
                    b"fonts" => {
                        font_vec = get_fonts(&mut reader, theme);
                    },
                    b"fills" => {
                        fill_vec = get_fills(&mut reader, theme);
                    },
                    b"borders" => {
                        borders_vec = get_borders(&mut reader, theme);
                    },
                    b"cellStyleXfs" => {
                        let style_vec = get_cell_style_xfs(&mut reader, &font_vec, &fill_vec, &borders_vec);
                        for style in style_vec {
                            let mut cell_style: CellStyle = CellStyle::default();
                            cell_style.set_style(style);
                            spreadsheet.add_cell_style_collection(cell_style);
                        }
                    },
                    b"cellXfs" => {
                        cell_xfs = get_cell_xfs(&mut reader, &num_fmt_vec, &font_vec, &fill_vec, &borders_vec, spreadsheet.get_cell_style_collection());
                    },
                    b"dxfs" => {
                        dxf_vec = get_dxfs(&mut reader, theme);
                    }
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"cellStyle" => {
                        let name = get_attribute(e, b"name").unwrap();
                        let xf_id = get_attribute(e, b"xfId").unwrap().parse::<usize>().unwrap();
                        let builtin_id = get_attribute(e, b"builtinId").unwrap().parse::<usize>().unwrap();
                        spreadsheet.get_cell_style_collection_mut().get_mut(xf_id).unwrap().set_name(name);
                        spreadsheet.get_cell_style_collection_mut().get_mut(xf_id).unwrap().set_builtin_id(builtin_id);
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok((cell_xfs, dxf_vec))
}

fn get_num_fmts(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>
)->HashMap<usize, NumberFormat>
{
    let mut buf = Vec::new();
    let mut num_fmt_vec: HashMap<usize, NumberFormat> = HashMap::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"numFmt" => {
                        let mut num_fmt = NumberFormat::default();
                        let id = get_attribute(e, b"numFmtId").unwrap().parse::<usize>().unwrap();
                        let value = condvert_character_reference(get_attribute(e, b"formatCode").unwrap().as_str());
                        num_fmt.set_format_code(value);
                        num_fmt_vec.insert(id, num_fmt);
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"numFmts" => {
                        return num_fmt_vec;
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "numFmts"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_dxfs(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    theme:&Theme
)->Vec<Style>
{
    let mut buf = Vec::new();
    let mut style_vec: Vec<Style> = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"dxf" => {
                        style_vec.push(get_dxf(reader, theme));
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"dxfs" => {
                        return style_vec;
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "dxfs"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_dxf(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    theme:&Theme
)->Style
{
    let mut buf = Vec::new();
    let mut style: Style = Style::default();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"font" => {
                        style.set_font(get_font(reader, theme));
                    },
                    b"fill" => {
                        style.set_fill(get_fill(reader, theme));
                    },
                    b"get_border" => {
                        match get_border(reader, theme){
                            Some(v) => {style.set_borders(v)},
                            None => {}
                        }
                        
                    }
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"dxf" => {
                        return style;
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "dxf"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_fonts(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    theme:&Theme
)->Vec<Font>
{
    let mut buf = Vec::new();
    let mut font_vec: Vec<Font> = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"font" => {
                        font_vec.push(get_font(reader, theme));
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"fonts" => {
                        return font_vec;
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "fonts"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_font(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    theme:&Theme
)->Font
{
    let mut buf = Vec::new();
    let mut font = Font::default();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"i" => {
                        font.set_italic(true);
                    },
                    b"strike"=> {
                        font.set_strikethrough(true);
                    },
                    b"sz" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"val" => {
                                    let value = get_attribute_value(attr).unwrap();
                                    font.set_size(value.parse::<usize>().unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    b"color" => {
                        get_attribute_color(e, font.get_color_mut(), theme);
                    },
                    b"name" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"val" => {
                                    font.set_name(get_attribute_value(attr).unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    b"family" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"val" => {
                                    let value = get_attribute_value(attr).unwrap();
                                    font.set_family(value.parse::<usize>().unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    b"charset" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"val" => {
                                    let value = get_attribute_value(attr).unwrap();
                                    font.set_charset(value.parse::<usize>().unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    b"scheme" => {
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"val" => {
                                    font.set_scheme(get_attribute_value(attr).unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"font" => {
                        return font;
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "font"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_fills(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    theme:&Theme
)->Vec<Fill>
{
    let mut buf = Vec::new();
    let mut fill_vec: Vec<Fill> = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"fill" => {
                        fill_vec.push(get_fill(reader, theme));
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"fills" => {
                        return fill_vec;
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "fills"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_fill(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    theme:&Theme
)->Fill
{
    let mut buf = Vec::new();
    let mut fill = Fill::default();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"patternFill" => {
                        get_attribute_pattern_fill(e, &mut fill);
                    },
                    _ => (),
                }
            }
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"patternFill" => {
                        get_attribute_pattern_fill(e, &mut fill);
                    },
                    b"fgColor" => {
                        get_attribute_color(e, fill.get_start_color_mut(), theme);
                    },
                    b"bgColor" => {
                        get_attribute_color(e, fill.get_end_color_mut(), theme);
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"fill" => {
                        return fill;
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "fill"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_borders(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>, 
    theme:&Theme
)->Vec<Borders>
{
    let mut borders_vec: Vec<Borders> = Vec::new();
    loop {
        match get_border(reader, theme) {
            Some(v) => {
                borders_vec.push(v);
            },
            None => {
                return borders_vec;
            }
        }
    }
}

fn get_border(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    theme:&Theme
)-> Option<Borders>
{
    let mut buf = Vec::new();
    let mut borders = Borders::default();
    let mut border = Border::default();

    // start
    match reader.read_event(&mut buf) {
        Ok(Event::Start(ref e)) => {
            match e.name() {
                b"border" => {
                    let mut is_diagonal_up: bool = false;
                    let mut is_diagonal_down: bool = false;
                    for a in e.attributes().with_checks(false) {
                        match a {
                            Ok(ref attr) if attr.key == b"diagonalUp" => {
                                is_diagonal_up = get_attribute_value(attr).unwrap() == "1";
                            },
                            Ok(ref attr) if attr.key == b"diagonalDown" => {
                                is_diagonal_down = get_attribute_value(attr).unwrap() == "1";
                            },
                            Ok(_) => {},
                            Err(_) => {},
                        }
                    }
                    if !is_diagonal_up && !is_diagonal_down {
                        borders.set_diagonal_direction(Borders::DIAGONAL_NONE);
                    } else if is_diagonal_up && !is_diagonal_down {
                        borders.set_diagonal_direction(Borders::DIAGONAL_UP);
                    } else if !is_diagonal_up && is_diagonal_down {
                        borders.set_diagonal_direction(Borders::DIAGONAL_DOWN);
                    } else {
                        borders.set_diagonal_direction(Borders::DIAGONAL_BOTH);
                    }
                },
                _ => {
                    return None;
                },
            }
        },
        Ok(Event::Eof) => panic!("Error not find {} end element", "border"),
        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        _ => {
            return None;
        },
    }
    buf.clear();
    
    // property
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"left" => {
                        border = Border::default();
                        get_attribute_pattern_border(e, &mut border);
                    },
                    b"right" => {
                        border = Border::default();
                        get_attribute_pattern_border(e, &mut border);
                    },
                    b"top" => {
                        border = Border::default();
                        get_attribute_pattern_border(e, &mut border);
                    },
                    b"bottom" => {
                        border = Border::default();
                        get_attribute_pattern_border(e, &mut border);
                    },
                    b"diagonal" => {
                        border = Border::default();
                        get_attribute_pattern_border(e, &mut border);
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"left" => {
                        border = Border::default();
                        border.set_border_style(Border::BORDER_NONE.to_string());
                        borders.set_left(border);
                        border = Border::default();
                    },
                    b"right" => {
                        border = Border::default();
                        border.set_border_style(Border::BORDER_NONE.to_string());
                        borders.set_right(border);
                        border = Border::default();
                    },
                    b"top" => {
                        border = Border::default();
                        border.set_border_style(Border::BORDER_NONE.to_string());
                        borders.set_top(border);
                        border = Border::default();
                    },
                    b"bottom" => {
                        border = Border::default();
                        border.set_border_style(Border::BORDER_NONE.to_string());
                        borders.set_bottom(border);
                        border = Border::default();
                    },
                    b"diagonal" => {
                        border = Border::default();
                        border.set_border_style(Border::BORDER_NONE.to_string());
                        borders.set_diagonal(border);
                        border = Border::default();
                    },
                    b"color" => {
                        get_attribute_color(e, border.get_color_mut(), theme);
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"left" => {
                        borders.set_left(border);
                        border = Border::default();
                    }
                    b"right" => {
                        borders.set_right(border);
                        border = Border::default();
                    }
                    b"top" => {
                        borders.set_top(border);
                        border = Border::default();
                    }
                    b"bottom" => {
                        borders.set_bottom(border);
                        border = Border::default();
                    }
                    b"diagonal" => {
                        borders.set_diagonal(border);
                        border = Border::default();
                    }
                    b"border" => {
                        return Some(borders);
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "border"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_cell_style_xfs(
    reader: &mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    font_vec: &Vec<Font>,
    fill_vec: &Vec<Fill>,
    borders_vec: &Vec<Borders>
)->Vec<Style>
{
    let mut cel_vec: Vec<Style> = Vec::new();
    let num_fmt_vec: HashMap<usize, NumberFormat> = HashMap::new();
    let cel_style_vec: Vec<CellStyle> = Vec::new();
    loop {
        match get_xf(reader, true, &num_fmt_vec, font_vec, fill_vec, borders_vec, &cel_style_vec) {
            Some(v) => {
                cel_vec.push(v);
            },
            None => {
                return cel_vec;
            }
        }
    }
}

fn get_cell_xfs(
    reader: &mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    num_fmt_vec: &HashMap<usize, NumberFormat>,
    font_vec: &Vec<Font>,
    fill_vec: &Vec<Fill>,
    borders_vec: &Vec<Borders>,
    cel_style_vec: &Vec<CellStyle>
)->Vec<Style>
{
    let mut cel_vec: Vec<Style> = Vec::new();
    loop {
        match get_xf(reader, false, num_fmt_vec, font_vec, fill_vec, borders_vec, cel_style_vec) {
            Some(v) => {
                cel_vec.push(v);
            },
            None => {
                return cel_vec;
            }
        }
    }
}

fn get_xf(
    reader: &mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    apply_defalut: bool,
    num_fmt_vec: &HashMap<usize, NumberFormat>,
    font_vec: &Vec<Font>,
    fill_vec: &Vec<Fill>,
    borders_vec: &Vec<Borders>,
    cel_style_vec: &Vec<CellStyle>
)-> Option<Style>
{
    let mut buf = Vec::new();
    let mut style = Style::default();

    // start
    match reader.read_event(&mut buf) {
        Ok(Event::Start(ref e)) => {
            match e.name() {
                b"xf" => {
                    get_attribute_pattern_xf(e, &mut style, apply_defalut, num_fmt_vec, font_vec, fill_vec, borders_vec, cel_style_vec);
                },
                _ => {
                    return None;
                },
            }
        },
        Ok(Event::Empty(ref e)) => {
            match e.name() {
                b"xf" => {
                    get_attribute_pattern_xf(e, &mut style, apply_defalut, num_fmt_vec, font_vec, fill_vec, borders_vec, cel_style_vec);
                    return Some(style);
                },
                _ => {
                    return None;
                },
            }
        },
        Ok(Event::End(ref e)) => {
            match e.name() {
                _ => {
                    return None;
                },
            }
        },
        Ok(Event::Eof) => panic!("Error not find {} end element", "xf"),
        Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        _ => (),
    }
    buf.clear();

    // property
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"alignment" => {
                        let mut alignment = Alignment::default();
                        for a in e.attributes().with_checks(false) {
                            match a {
                                Ok(ref attr) if attr.key == b"horizontal" => {
                                    alignment.set_horizontal(get_attribute_value(attr).unwrap());
                                },
                                Ok(ref attr) if attr.key == b"vertical" => {
                                    alignment.set_vertical(get_attribute_value(attr).unwrap());
                                },
                                Ok(_) => {},
                                Err(_) => {},
                            }
                        }
                        style.set_alignment(alignment);
                    }
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"xf" => {
                        return Some(style);
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "alignment"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn get_attribute_color(
    e:&quick_xml::events::BytesStart<'_>, 
    color:&mut Color, 
    theme:&Theme
)
{
    for a in e.attributes().with_checks(false) {
        match a {
            Ok(ref attr) if attr.key == b"indexed" => {
                let value = get_attribute_value(attr).unwrap();
                color.set_indexed(value.parse::<usize>().unwrap());
            },
            Ok(ref attr) if attr.key == b"theme" => {
                let theme_color_map = theme.get_color_map();
                let value = get_attribute_value(attr).unwrap();
                color.set_theme_index(value.parse::<usize>().unwrap(), theme_color_map);
            },
            Ok(ref attr) if attr.key == b"rgb" => {
                color.set_argb(get_attribute_value(attr).unwrap());
            },
            Ok(ref attr) if attr.key == b"tint" => {
                let value = get_attribute_value(attr).unwrap();
                color.set_tint(value.parse::<f64>().unwrap());
            },
            Ok(_) => {},
            Err(_) => {},
        }
    }
}

fn get_attribute_pattern_fill(e:&quick_xml::events::BytesStart<'_>, fill:&mut Fill)
{
    for a in e.attributes().with_checks(false) {
        match a {
            Ok(ref attr) if attr.key == b"patternType" => {
                fill.set_fill_type(get_attribute_value(attr).unwrap());
            },
            Ok(_) => {},
            Err(_) => {},
        }
    }
}

fn get_attribute_pattern_border(e:&quick_xml::events::BytesStart<'_>, border:&mut Border)
{
    for a in e.attributes().with_checks(false) {
        match a {
            Ok(ref attr) if attr.key == b"style" => {
                border.set_border_style(get_attribute_value(attr).unwrap());
            },
            Ok(_) => {},
            Err(_) => {},
        }
    }
}

fn get_attribute_pattern_xf(
    e:&quick_xml::events::BytesStart<'_>,
    style:&mut Style,
    apply_defalut: bool,
    num_fmt_vec: &HashMap<usize, NumberFormat>,
    font_vec: &Vec<Font>,
    fill_vec: &Vec<Fill>,
    borders_vec: &Vec<Borders>,
    cel_style_vec: &Vec<CellStyle>
)
{
    // xfId
    match get_attribute(e, b"xfId") {
        Some(v) => {
            let id = v.parse::<usize>().unwrap();
            style.set_xf_id(id);
            match cel_style_vec.get(id).unwrap().get_style().get_number_format() {
                Some(v) => style.set_number_format(v.clone()),
                None => {}
            }
            match cel_style_vec.get(id).unwrap().get_style().get_font() {
                Some(v) => style.set_font(v.clone()),
                None => {}
            }
            match cel_style_vec.get(id).unwrap().get_style().get_fill() {
                Some(v) => style.set_fill(v.clone()),
                None => {}
            }
            match cel_style_vec.get(id).unwrap().get_style().get_borders() {
                Some(v) => style.set_borders(v.clone()),
                None => {}
            }
            match cel_style_vec.get(id).unwrap().get_style().get_alignment() {
                Some(v) => style.set_alignment(v.clone()),
                None => {}
            }
        },
        None => {}
    }

    // NumberFormat
    let apply_number_format = match get_attribute(e, b"applyNumberFormat") {
        Some(v) if v == "1" => true,
        Some(_) => false,
        None => apply_defalut.clone()
    };
    if apply_number_format {
        let id = get_attribute(e, b"numFmtId").unwrap().parse::<usize>().unwrap();
        match num_fmt_vec.get(&id) {
            Some(v) => {
                style.set_number_format(v.clone());
            },
            None => {
                match super::super::structs::number_format::FILL_BUILT_IN_FORMAT_CODES.get(&id){
                    Some(v) => {
                        let mut num_fmt = NumberFormat::default();
                        num_fmt.set_format_code(v);
                        style.set_number_format(num_fmt);
                    },
                    None => {}
                }
            }
        }
    }

    // Font
    let apply_font = match get_attribute(e, b"applyFont") {
        Some(v) if v == "1" => true,
        Some(_) => false,
        None => apply_defalut.clone()
    };
    if apply_font {
        let id = get_attribute(e, b"fontId").unwrap().parse::<usize>().unwrap();
        if id == 0usize {
            style.set_font(Font::get_defalut_value());
        } else {
            style.set_font((*font_vec.get(id).unwrap()).clone());
        }
    }

    // Fill
    let apply_fill = match get_attribute(e, b"applyFill") {
        Some(v) if v == "1" => true,
        Some(_) => false,
        None => apply_defalut.clone()
    };
    if apply_fill {
        let id = get_attribute(e, b"fillId").unwrap().parse::<usize>().unwrap();
        if id == 0usize {
            style.set_fill(Fill::get_defalut_value());
        } else {
            style.set_fill((*fill_vec.get(id).unwrap()).clone());
        }
    }

    // Border
    let apply_border = match get_attribute(e, b"applyBorder") {
        Some(v) if v == "1" => true,
        Some(_) => false,
        None => apply_defalut.clone()
    };
    if apply_border {
        let id = get_attribute(e, b"borderId").unwrap().parse::<usize>().unwrap();
        if id == 0usize {
            style.set_borders(Borders::get_defalut_value());
        } else {
            style.set_borders((*borders_vec.get(id).unwrap()).clone());
        }
    }

}
