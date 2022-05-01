use super::driver::*;
use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::mem;
use std::{io, result};

use structs::drawing::Theme;

pub fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    target: &str,
) -> result::Result<Theme, XlsxError> {
    let r = io::BufReader::new(arv.by_name(&format!("xl/{}", target))?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut theme: Theme = Theme::default();
    theme.add_color_map(""); // lt1
    theme.add_color_map(""); // dk1
    theme.add_color_map(""); // lt2
    theme.add_color_map(""); // dk2
    let mut tag_name = String::from("");

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"a:theme" => {
                    theme.set_theme_name(get_attribute(e, b"name").unwrap());
                }
                b"a:clrScheme" => {
                    theme.set_color_scheme_name(get_attribute(e, b"name").unwrap());
                }
                b"a:dk1" => {
                    tag_name = "dk1".into();
                }
                b"a:lt1" => {
                    tag_name = "lt1".into();
                }
                b"a:dk2" => {
                    tag_name = "dk2".into();
                }
                b"a:lt2" => {
                    tag_name = "lt2".into();
                }
                b"a:majorFont" => {
                    theme.get_major_font_mut().set_attributes(&mut reader, e);
                }
                b"a:minorFont" => {
                    theme.get_minor_font_mut().set_attributes(&mut reader, e);
                }
                _ => (),
            },
            Ok(Event::Empty(ref e)) => match e.name() {
                b"a:sysClr" => {
                    let value = get_attribute(e, b"lastClr").unwrap();
                    set_value(&mut theme, &tag_name, &value);
                    tag_name = "".into();
                }
                b"a:srgbClr" => {
                    let value = get_attribute(e, b"val").unwrap();
                    set_value(&mut theme, &tag_name, &value);
                    tag_name = "".into();
                }
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    Ok(theme)
}

fn set_value(theme: &mut Theme, tag_name: &str, value: &str) {
    if tag_name == "lt1" {
        let _ = mem::replace(&mut theme.get_color_map_mut()[0], value.into());
    } else if tag_name == "dk1" {
        let _ = mem::replace(&mut theme.get_color_map_mut()[1], value.into());
    } else if tag_name == "lt2" {
        let _ = mem::replace(&mut theme.get_color_map_mut()[2], value.into());
    } else if tag_name == "dk2" {
        let _ = mem::replace(&mut theme.get_color_map_mut()[3], value.into());
    } else {
        theme.add_color_map(value);
    }
}
