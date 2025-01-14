use quick_xml::{events::Event, Reader};

use crate::{
    structs::{drawing::charts::ChartSpace, raw::RawFile},
    xml_read_loop,
};

pub(crate) fn read(raw_file: &RawFile, chart_space: &mut ChartSpace) {
    let data = std::io::Cursor::new(raw_file.get_file_data());
    let mut reader = Reader::from_reader(data);

    reader.config_mut().trim_text(true);

    xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() == b"c:chartSpace" {
                chart_space.set_attributes(&mut reader, e);
            }
        },
        Event::Eof => break,
    );
}
