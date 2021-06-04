use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use super::driver::*;

use ::structs::Worksheet;

pub(crate) fn read(
    dir: &TempDir,
    target: &str,
    hyperlink_vec: &Vec<(String, String)>,
    worksheet: &mut Worksheet
) -> result::Result<Vec<(String, String, String)>, XlsxError> {
    let mut result:Vec<(String, String, String)> = Vec::new();

    let path = dir.path().join(format!("xl/worksheets/_rels/{}.rels", target.replace("worksheets/","")));
    let mut reader = match Reader::from_file(path){
        Ok(v) => {v},
        Err(_) => {return Ok(result);}
    };
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
