use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::result;
use structs::drawing::spreadsheet::WorksheetDrawing;
use structs::raw::RawFile;
use structs::raw::RawRelationships;
use structs::Worksheet;

pub(crate) fn read(
    worksheet: &mut Worksheet,
    drawing_file: &RawFile,
    drawing_relationships: Option<&RawRelationships>,
) -> result::Result<(), XlsxError> {
    let data = std::io::Cursor::new(drawing_file.get_file_data());
    let mut reader = Reader::from_reader(data);
    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name().into_inner() {
                b"xdr:wsDr" => {
                    let mut obj = WorksheetDrawing::default();
                    obj.set_attributes(
                        &mut reader,
                        e,
                        drawing_relationships,
                        worksheet.get_ole_objects_mut(),
                    );
                    worksheet.set_worksheet_drawing(obj);
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
