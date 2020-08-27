use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;

use super::driver::*;
use super::super::structs::text_element::TextElement;
use super::super::structs::theme::Theme;
use super::super::structs::rich_text::RichText;

const SHARED_STRINGS: &'static str = "xl/sharedStrings.xml";

pub(crate) fn read(dir: &TempDir, theme:&Theme) -> result::Result<Vec<(String, Option<RichText>)>, XlsxError> {
    let path = dir.path().join(SHARED_STRINGS);
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut res: Vec<(String, Option<RichText>)> = Vec::new();

    let mut value: String = String::from("");
    let mut text: String = String::from("");
    let mut text_element_vec: Vec<TextElement> = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"r" => text_element_vec.push(get_text_element(&mut reader, theme)),
                    b"rPh" => get_rubi(&mut reader),
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => value = e.unescape_and_decode(&reader).unwrap(),
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"t" => text = value.clone(),
                    b"si" => {
                        if text_element_vec.len() > 0 {
                            let mut rich_text = RichText::default();
                            rich_text.set_rich_text_elements(text_element_vec);
                            res.push((text, Some(rich_text)));
                        } else {
                            res.push((text, None));
                        }
                        text = String::from("");
                        text_element_vec = Vec::new();
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
    Ok(res)
}

fn get_text_element(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    theme:&Theme
)->TextElement {
    let mut buf = Vec::new();
    let mut text_element = TextElement::default();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"rPr" => text_element.set_font(get_font(reader, theme)),
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => text_element.set_text(e.unescape_and_decode(&reader).unwrap()),
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

fn get_rubi(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>) {
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"rPh" => return,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "rPh"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}