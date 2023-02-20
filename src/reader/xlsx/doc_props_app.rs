use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};

use structs::Spreadsheet;

const FILE_PATH: &str = "docProps/app.xml";

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> result::Result<(), XlsxError> {
    let r = io::BufReader::new(match arv.by_name(FILE_PATH) {
        Ok(v) => v,
        Err(zip::result::ZipError::FileNotFound) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e.into());
        }
    });
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut string_value: String = String::from("");
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => string_value = e.unescape().unwrap().to_string(),
            Ok(Event::End(ref e)) => {
                match e.name().into_inner() {
                    b"Manager" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_manager(string_value.clone());
                    }
                    b"Company" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_company(string_value.clone());
                    }
                    _ => (),
                }
                string_value = String::from("");
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    Ok(())
}
