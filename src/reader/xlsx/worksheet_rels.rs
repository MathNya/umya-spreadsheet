use std::{io, result};
use quick_xml::Reader;
use quick_xml::events::{Event};
use super::XlsxError;
use super::driver::*;

use ::structs::Worksheet;

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
    target: &str,
    hyperlink_vec: &Vec<(String, String)>,
    worksheet: &mut Worksheet
) -> result::Result<Vec<(String, String, String)>, XlsxError> {
    let mut result:Vec<(String, String, String)> = Vec::new();
    let path_str = normalize_path_to_str(&format!("xl/worksheets/_rels/{}.rels", target.replace("worksheets/","")));
    let r = io::BufReader::new(match arv.by_name(path_str.as_str()) {
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
                        if &type_value == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink" {
                            for (coordinate, rid) in hyperlink_vec {
                                if &id_value == rid {
                                    worksheet.get_cell_mut(coordinate).get_hyperlink_mut().set_url(target_value);
                                    break;
                                }
                            }
                        } else {
                            result.push((id_value, type_value, target_value));
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

    Ok(result)
}

pub(crate) fn read_rid<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
    target: &str,
    rid: &str,
) -> result::Result<(String, String), XlsxError> {
    let mut result:(String, String) = (String::from(""), String::from(""));
    let path_str = normalize_path_to_str(&format!("xl/worksheets/_rels/{}.rels", target.replace("worksheets/","")));
    let r = io::BufReader::new(match arv.by_name(path_str.as_str()) {
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
