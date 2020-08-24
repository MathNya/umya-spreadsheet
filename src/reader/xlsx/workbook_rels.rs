use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use super::driver::*;

const FILE_PATH: &'static str = "xl/_rels/workbook.xml.rels";

pub(crate) fn read(dir: &TempDir) -> result::Result<Vec<(String, String, String)>, XlsxError> 
{
    let path = dir.path().join(FILE_PATH);
    let mut reader = Reader::from_file(path)?;
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