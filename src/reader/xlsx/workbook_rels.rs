use super::driver::*;
use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};
use structs::Spreadsheet;

const FILE_PATH: &str = "xl/_rels/workbook.xml.rels";

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> result::Result<Vec<(String, String, String)>, XlsxError> {
    let r = io::BufReader::new(arv.by_name(FILE_PATH)?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut result: Vec<(String, String, String)> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                b"Relationship" => {
                    let id_value = get_attribute(e, b"Id").unwrap();
                    let type_value = get_attribute(e, b"Type").unwrap();
                    let target_value = get_attribute(e, b"Target").unwrap();
                    let target_value = target_value
                        .strip_prefix("/xl/")
                        .map(|t| t.to_owned())
                        .unwrap_or(target_value);
                    if type_value == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition" {
                        spreadsheet.update_pivot_caches(id_value, target_value);
                    } else {
                        result.push((id_value, type_value, target_value));
                    }
                }
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    Ok(result)
}
