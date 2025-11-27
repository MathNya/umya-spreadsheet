use quick_xml::{
    Reader,
    events::Event,
};

use crate::{
    structs::{
        Comment,
        Worksheet,
        raw::RawFile,
    },
    xml_read_loop,
};

pub(crate) fn read(worksheet: &mut Worksheet, drawing_file: &RawFile) {
    let data = std::io::Cursor::new(drawing_file.file_data());
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(false);

    let mut authors: Vec<String> = Vec::new();
    let mut value: String = String::new();
    xml_read_loop!(
        reader,
        Event::Empty(ref e) => {
            if e.name().into_inner() == b"author" {
                authors.push(String::new());
            }
        },
        Event::Start(ref e) => {
            if e.name().into_inner() ==  b"comment" {
                let mut obj = Comment::default();
                obj.set_attributes(&mut reader, e, &authors);
                worksheet.add_comments(obj);
            }
        },
        Event::Text(e) => {
            value = e.unescape().unwrap().to_string();
        },
        Event::End(ref e) => {
            if e.name().into_inner() == b"author" {
                authors.push(value.clone());
            }
        },
        Event::Eof => break,
    );
}
