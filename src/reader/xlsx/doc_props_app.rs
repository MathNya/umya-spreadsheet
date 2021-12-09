use std::{io, result};
use quick_xml::Reader;
use quick_xml::events::{Event};
use super::XlsxError;

use ::structs::Spreadsheet;

const FILE_PATH: &'static str = "docProps/app.xml";

pub(crate) fn read<R: io::Read + io::Seek>(arv: &mut zip::ZipArchive<R>, spreadsheet:&mut Spreadsheet) -> result::Result<(), XlsxError> {
    let  r = io::BufReader::new(arv.by_name(FILE_PATH)?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut string_value: String = String::from("");
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Text(e)) => string_value = e.unescape_and_decode(&reader).unwrap(),
            Ok(Event::End(ref e)) => {
                match e.name() {
                    b"Manager" => {spreadsheet.get_properties_mut().set_manager(string_value.clone());},
                    b"Company" => {spreadsheet.get_properties_mut().set_company(string_value.clone());},
                _ => (),
                }
                string_value = String::from("");
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    Ok(())
}