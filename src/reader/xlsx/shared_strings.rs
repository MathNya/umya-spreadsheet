use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;

const SHARED_STRINGS: &'static str = "xl/sharedStrings.xml";

pub(crate) fn read(dir: &TempDir) -> result::Result<Vec<String>, XlsxError> {
    let path = dir.path().join(SHARED_STRINGS);
    dbg!(path.clone());
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut res: Vec<String> = Vec::new();

    let mut is_si = false;
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"si" => {
                        is_si = true;
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                if is_si {
                    res.push(e.unescape_and_decode(&reader).unwrap());
                }
                is_si = false;
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    Ok(res)
}