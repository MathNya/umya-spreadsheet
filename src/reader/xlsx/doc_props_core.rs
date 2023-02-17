use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};

use structs::Spreadsheet;

const FILE_PATH: &str = "docProps/core.xml";

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
                    b"dc:title" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_title(string_value.clone());
                    }
                    b"dc:subject" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_subject(string_value.clone());
                    }
                    b"dc:creator" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_creator(string_value.clone());
                    }
                    b"cp:keywords" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_keywords(string_value.clone());
                    }
                    b"dc:description" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_description(string_value.clone());
                    }
                    b"cp:lastModifiedBy" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_last_modified_by(string_value.clone());
                    }
                    b"cp:revision" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_revision(string_value.clone());
                    }
                    b"dcterms:created" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_created(string_value.clone());
                    }
                    b"dcterms:modified" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_modified(string_value.clone());
                    }
                    b"cp:category" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_category(string_value.clone());
                    }
                    b"cp:version" => {
                        spreadsheet
                            .get_properties_mut()
                            .set_version(string_value.clone());
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
