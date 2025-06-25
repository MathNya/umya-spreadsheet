use quick_xml::{
    Reader,
    events::Event,
};

use crate::{
    reader::driver::xml_read_loop,
    structs::{
        Worksheet,
        drawing::spreadsheet::WorksheetDrawing,
        raw::{
            RawFile,
            RawRelationships,
        },
    },
};

pub(crate) fn read(
    worksheet: &mut Worksheet,
    drawing_file: &RawFile,
    drawing_relationships: Option<&RawRelationships>,
) {
    let data = std::io::Cursor::new(drawing_file.file_data());
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
                    worksheet.ole_objects_mut(),
                );
                worksheet.set_worksheet_drawing(obj);
            }
        },
        Event::Eof => break
    );
}
