use std::result;
use quick_xml::Reader;
use quick_xml::events::{Event};
use tempdir::TempDir;
use super::XlsxError;
use super::super::structs::worksheet::Worksheet;

pub(crate) fn read(
    dir: &TempDir,
    target: &str,
    worksheet: &mut Worksheet
)-> result::Result<(), XlsxError>{
    let path = dir.path().join(format!("xl/drawings/{}", target));
    let mut reader = Reader::from_file(path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"xdr:wsDr" => {
                        let worksheet_drawing = worksheet.get_worksheet_drawing_mut();
                        worksheet_drawing.set_attributes(&mut reader, e, dir, target);
                    },
                    _ => (),
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok(())
}
