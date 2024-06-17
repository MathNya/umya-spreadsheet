use super::XlsxError;
use crate::xml_read_loop;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::result;
use structs::drawing::charts::ChartSpace;
use structs::raw::RawFile;

pub(crate) fn read(
    raw_file: &RawFile,
    chart_space: &mut ChartSpace,
) -> result::Result<(), XlsxError> {
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

    Ok(())
}
