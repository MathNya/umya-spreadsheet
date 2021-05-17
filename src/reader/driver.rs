use quick_xml::events::attributes::Attribute;
use quick_xml::events::{Event};
use tempdir::TempDir;
use std::fs;
use std::fs::File;
use std::io;
use std::string::FromUtf8Error;

use super::super::structs::theme::Theme;
use super::super::structs::font::Font;
use super::super::structs::color::Color;
use super::super::structs::text_element::TextElement;

pub(crate) fn unzip(zip_file: &File, dir: &TempDir) -> Result<(), zip::result::ZipError> {
    let mut zip = zip::ZipArchive::new(zip_file)?;
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let path = dir.path().join(file.name());
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?
        }
        if (&*file.name()).ends_with("/") {
            fs::create_dir_all(path)?
        } else {
            let mut archived_file = File::create(path)?;
            let _ = io::copy(&mut file, &mut archived_file);
        }
    }
    Ok(())
}

pub(crate) fn get_attribute(
    e:&quick_xml::events::BytesStart<'_>,
    key:&[u8]
) -> Option<String>
{
    for a in e.attributes().with_checks(false) {
        match a {
            Ok(ref attr) if attr.key == key => {
                return Some(get_attribute_value(attr).unwrap());
            },
            Ok(_) => {},
            Err(_) => {},
        }
    }
    None
}
pub(crate) fn get_attribute_value(attr: &Attribute) -> Result<String, FromUtf8Error>
{
    let value = (&attr.value).clone().into_owned();
    String::from_utf8(value)
}

pub(crate) fn condvert_character_reference(src: &str) -> String
{
    src.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

pub(crate) fn get_font(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    theme:&Theme
)->Font {
    let mut buf = Vec::new();
    let mut font = Font::default();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"i" => {font.set_italic(true);},
                    b"b" => {font.set_bold(true);},
                    b"u" => {
                        let val = match get_attribute(e, b"val") {
                            Some(v) => v,
                            None => Font::UNDERLINE_SINGLE.to_string()
                        };
                        font.set_underline(val);
                    },
                    b"strike"=> {font.set_strikethrough(true);},
                    b"sz" => {font.set_size(get_attribute(e, b"val").unwrap().parse::<usize>().unwrap());},
                    b"color" => {get_attribute_color(e, font.get_color_mut(), theme);},
                    b"name" => {font.set_name(get_attribute(e, b"val").unwrap());},
                    b"rFont" => {font.set_name(get_attribute(e, b"val").unwrap());},
                    b"family" => {font.set_family(get_attribute(e, b"val").unwrap().parse::<usize>().unwrap());},
                    b"charset" => {font.set_charset(get_attribute(e, b"val").unwrap().parse::<usize>().unwrap());},
                    b"scheme" => {font.set_scheme(get_attribute(e, b"val").unwrap());},
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"font" => return font,
                    b"rPr" => return font,
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

pub(crate) fn get_attribute_color(
    e:&quick_xml::events::BytesStart<'_>, 
    color:&mut Color, 
    theme:&Theme
) {
    for a in e.attributes().with_checks(false) {
        match a {
            Ok(ref attr) if attr.key == b"indexed" => {
                let value = get_attribute_value(attr).unwrap();
                let _ = color.set_indexed(value.parse::<usize>().unwrap());
            },
            Ok(ref attr) if attr.key == b"theme" => {
                let theme_color_map = theme.get_color_map();
                let value = get_attribute_value(attr).unwrap();
                let _ = color.set_theme_index(value.parse::<usize>().unwrap(), theme_color_map);
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
}

pub(crate) fn get_text_element(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    theme:&Theme
)->TextElement {
    let mut buf = Vec::new();
    let mut text_element = TextElement::default();
    let mut with_first_space = false;
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"rPr" => text_element.set_font(get_font(reader, theme)),
                    b"t" => {
                        match get_attribute(e, b"xml:space") {
                            Some(v) => {
                                if v == "preserve" {
                                    with_first_space = true;
                                }
                            },
                            None => {}
                        }
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                let mut value = e.unescape_and_decode(&reader).unwrap();
                if with_first_space {
                    value = format!("\r\n{}", value);
                }
                text_element.set_text(value);
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"r" => return text_element,
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
