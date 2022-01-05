use std::{io, result};
use quick_xml::Reader;
use quick_xml::events::{Event};
use super::XlsxError;
use super::driver::*;
use structs::Theme;
use structs::Worksheet;
use structs::Comment;

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
    target: &str,
    worksheet: &mut Worksheet,
    _theme: &Theme
) -> result::Result<(), XlsxError> {
    let path_str = normalize_path_to_str(&format!("xl/worksheets/{}", target));
    let r = io::BufReader::new(arv.by_name(path_str.as_str())?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(false);

    let mut authors: Vec<String> = Vec::new();
    let mut value: String = String::from("");
    let mut buf = Vec::new();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"comment" => {
                        let mut obj = Comment::default();
                        obj.set_attributes(&mut reader, e, &authors);
                        worksheet.add_comments(obj);
                    },
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                value = e.unescape_and_decode(&reader).unwrap();
            },
            Ok(Event::End(ref e)) => match e.name() {
                b"author" => {
                    authors.push(value.clone());
                },
                _ => (),
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    Ok(())
}
