use super::driver::*;
use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    target: &str,
) -> result::Result<Vec<(String, String, String)>, XlsxError> {
    let path_str = normalize_path_to_str(&format!(
        "xl/drawings/_rels/{}.rels",
        target.replace("../drawings/", "")
    ));
    let r = io::BufReader::new(arv.by_name(path_str.as_str())?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut result: Vec<(String, String, String)> = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Empty(ref e)) => match e.name() {
                b"Relationship" => {
                    let id_value = get_attribute(e, b"Id").unwrap();
                    let type_value = get_attribute(e, b"Type").unwrap();
                    let target_value = get_attribute(e, b"Target").unwrap();
                    result.push((id_value, type_value, target_value));
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
