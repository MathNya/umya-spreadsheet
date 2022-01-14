use std::{io, result};
use quick_xml::Reader;
use quick_xml::events::{Event};
use super::XlsxError;
use super::driver::*;

const FILE_PATH: &'static str = "xl/_rels/workbook.xml.rels";

pub(crate) fn read<R: io::Read + io::Seek>(arv: &mut zip::read::ZipArchive<R>) -> result::Result<Vec<(String, String, String)>, XlsxError> 
{
    let r = io::BufReader::new(arv.by_name(FILE_PATH)?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut result:Vec<(String, String, String)> = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"Relationship" => {
                        let id_value =  get_attribute(e, b"Id").unwrap();
                        let type_value =  get_attribute(e, b"Type").unwrap();
                        let target_value =  get_attribute(e, b"Target").unwrap();
                        result.push((id_value, type_value, target_value));
                    }
                    _ => (),
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    Ok(result)
}

pub(crate) fn read_rid<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
    rid: &str,
) -> result::Result<(String, String), XlsxError> {
    let mut result:(String, String) = (String::from(""), String::from(""));
    let r = io::BufReader::new(match arv.by_name(FILE_PATH) {
        Ok(v) => v,
        Err(zip::result::ZipError::FileNotFound) => {return Ok(result);},
        Err(e) => {return Err(e.into());}
    });
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => {
                match e.name() {
                    b"Relationship" => {
                        let id_value =  get_attribute(e, b"Id").unwrap();
                        let type_value =  get_attribute(e, b"Type").unwrap();
                        let target_value =  get_attribute(e, b"Target").unwrap();
                        if &id_value == rid {
                            result = (type_value, target_value);
                            return Ok(result);
                        }
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

    panic!("Not found Relationship.");
}
