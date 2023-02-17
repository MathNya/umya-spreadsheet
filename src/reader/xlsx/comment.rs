use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::result;
use structs::raw::RawFile;
use structs::Comment;
use structs::Worksheet;

pub(crate) fn read(
    worksheet: &mut Worksheet,
    drawing_file: &RawFile,
) -> result::Result<(), XlsxError> {
    let data = std::io::Cursor::new(drawing_file.get_file_data());
    let mut reader = Reader::from_reader(data);
    reader.trim_text(false);
    let mut buf = Vec::new();

    let mut authors: Vec<String> = Vec::new();
    let mut value: String = String::from("");
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                b"author" => {
                    authors.push(String::from(""));
                }
                _ => (),
            },
            Ok(Event::Start(ref e)) => match e.name().into_inner() {
                b"comment" => {
                    let mut obj = Comment::default();
                    obj.set_attributes(&mut reader, e, &authors);
                    worksheet.add_comments(obj);
                }
                _ => (),
            },
            Ok(Event::Text(e)) => {
                value = e.unescape().unwrap().to_string();
            }
            Ok(Event::End(ref e)) => match e.name().into_inner() {
                b"author" => {
                    authors.push(value.clone());
                }
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
