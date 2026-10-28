use quick_xml::{
    Reader,
    events::Event,
};

use crate::{
    XlsxError,
    office2019::threaded_comment::ThreadedComment,
    structs::{
        Worksheet,
        raw::RawFile,
    },
    try_xml_read_loop,
};

pub(crate) fn read(worksheet: &mut Worksheet, drawing_file: &RawFile) -> Result<(), XlsxError> {
    let data = std::io::Cursor::new(drawing_file.file_data());
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(false);

    try_xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() ==  b"threadedComment" {
                let mut obj = ThreadedComment::default();
                obj.set_attributes(&mut reader, e);
                worksheet.add_threaded_comments(obj);
            }
        },
        Event::Eof => break,
    );

    Ok(())
}
