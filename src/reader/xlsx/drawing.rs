use quick_xml::Reader;
use quick_xml::events::Event;

use crate::reader::driver::xml_read_loop;
use crate::structs::Worksheet;
use crate::structs::drawing::spreadsheet::WorksheetDrawing;
use crate::structs::raw::RawFile;
use crate::structs::raw::RawRelationships;

pub(crate) fn read(
    worksheet: &mut Worksheet,
    drawing_file: &RawFile,
    drawing_relationships: Option<&RawRelationships>,
) {
    let data = std::io::Cursor::new(drawing_file.get_file_data());
    let mut reader = Reader::from_reader(data);
    reader.config_mut().trim_text(true);

    xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() == b"xdr:wsDr" {
                let mut obj = WorksheetDrawing::default();
                obj.set_attributes(
                    &mut reader,
                    e,
                    drawing_relationships,
                    worksheet.get_ole_objects_mut(),
                );
                worksheet.set_worksheet_drawing(obj);
            }
        },
        Event::Eof => break
    );
}
