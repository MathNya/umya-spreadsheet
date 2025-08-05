use super::XlsxError;
use crate::office2019::threaded_comment::ThreadedComment;
use crate::structs::raw::RawFile;
use crate::structs::Comment;
use crate::structs::Worksheet;
use crate::xml_read_loop;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::result;

pub(crate) fn read(
    worksheet: &mut Worksheet,
    drawing_file: &RawFile,
) -> result::Result<(), XlsxError> {
    let data = std::io::Cursor::new(drawing_file.get_file_data());
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(false);

    let mut authors: Vec<String> = Vec::new();
    let mut value: String = String::new();
    xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() ==  b"threadedComment" {
                let mut obj = ThreadedComment::default();
                obj.set_attributes(&mut reader, e);
                worksheet.add_threaded_comments(obj);
            }
        },
        Event::Text(e) => {
            value = e.unescape().unwrap().to_string();
        },
        Event::Eof => break,
    );

    Ok(())
}
