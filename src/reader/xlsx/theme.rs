use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use super::driver::*;

use super::super::structs::theme::Theme;

const FILE_PATH: &'static str = "xl/theme/theme1.xml";

pub fn read(dir: &TempDir) -> result::Result<Theme, XlsxError> {
    let path = dir.path().join(FILE_PATH);
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut theme: Theme = Theme::default();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"a:theme " => {
                        theme.set_theme_name(get_attribute(e, b"name").unwrap());
                    },
                    b"a:clrScheme" => {
                        theme.set_color_scheme_name(get_attribute(e, b"name").unwrap());
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"a:sysClr" => {
                        theme.add_color_map(get_attribute(e, b"lastClr").unwrap());
                    },
                    b"a:srgbClr" => {
                        theme.add_color_map(get_attribute(e, b"val").unwrap());
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

    //spreadsheet.set_theme(theme);

    Ok(theme)
}
