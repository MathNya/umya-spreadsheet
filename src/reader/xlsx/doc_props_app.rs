use super::XlsxError;
use helper::const_str::*;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};

use structs::Spreadsheet;

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> result::Result<(), XlsxError> {
    let r = io::BufReader::new(match arv.by_name(ARC_APP) {
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
                        spreadsheet.get_properties_mut().set_manager(&string_value);
                    }
                    b"Company" => {
                        spreadsheet.get_properties_mut().set_company(&string_value);
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
