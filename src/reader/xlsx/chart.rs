use super::XlsxError;
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

    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name().into_inner() {
                b"c:chartSpace" => {
                    chart_space.set_attributes(&mut reader, e);
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
