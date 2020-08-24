use std::result;
use std::collections::HashMap; 
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use super::driver::*;

use super::super::structs::drawing::Drawing;
use super::super::structs::worksheet::Worksheet;

pub(crate) fn read(
    dir: &TempDir,
    target: &str,
    worksheet: &mut Worksheet
) -> result::Result<Vec<(HashMap<String, usize>, HashMap<String, usize>, String, Option<String>)>, XlsxError>
{
    let path = dir.path().join(format!("xl/drawings/{}", target));
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut result: Vec<(HashMap<String, usize>, HashMap<String, usize>, String, Option<String>)> = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"xdr:twoCellAnchor" => {
                        result.push(two_cell_anchor(&mut reader, worksheet));
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

    Ok(result)
}

fn two_cell_anchor(
    reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>,
    worksheet: &mut Worksheet
)->(HashMap<String, usize>, HashMap<String, usize>, String, Option<String>)
{
    let mut buf = Vec::new();

    let mut from: HashMap<String, usize> = HashMap::new();
    let mut to: HashMap<String, usize> = HashMap::new();
    let mut name:String = String::from("");
    let mut chart_id: Option<String> = None;
    let mut drawing: Option<&Drawing> = None;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"xdr:sp" => {
                        drawing = Some(worksheet.new_drawing());
                    },
                    b"xdr:from" => {
                        from = from_and_to(reader);
                    },
                    b"xdr:to" => {
                        to = from_and_to(reader);
                    },
                    _ => (),
                }
            },
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"xdr:cNvPr" => {
                        name = get_attribute(e, b"name").unwrap();
                    },
                    b"c:chart" => {
                        chart_id = Some(get_attribute(e, b"r:id").unwrap());
                    },
                    _ => (),
                }
            },
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"xdr:twoCellAnchor" => return (from, to, name, chart_id),
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:twoCellAnchor"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}

fn from_and_to(reader:&mut quick_xml::Reader<std::io::BufReader<std::fs::File>>)->HashMap<String, usize>
{
    let mut buf = Vec::new();
    let mut result: HashMap<String, usize> = HashMap::new();

    let mut string_value:String = String::from("");

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Text(e)) => string_value = e.unescape_and_decode(&reader).unwrap(),
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"xdr:col" => {
                        result.insert("col".to_string(), string_value.parse::<usize>().unwrap());
                    },
                    b"xdr:colOff" => {
                        result.insert("colOff".to_string(), string_value.parse::<usize>().unwrap());
                    },
                    b"xdr:row" => {
                        result.insert("row".to_string(), string_value.parse::<usize>().unwrap());
                    },
                    b"xdr:rowOff" => {
                        result.insert("rowOff".to_string(), string_value.parse::<usize>().unwrap());
                    },
                    b"xdr:from" => return result,
                    b"xdr:to" => return result,
                    _ => (),
                }
            },
            Ok(Event::Eof) => panic!("Error not find {} end element", "c:from or c:to"),
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
}
