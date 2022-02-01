use super::driver::normalize_path_to_str;
use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io::Read;
use std::{io, result};
use structs::drawing::charts::ChartSpace;

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
    target: &String,
    chart_space: &mut ChartSpace,
) -> result::Result<(), XlsxError> {
    let data = {
        let path_str = normalize_path_to_str(&format!("xl/drawings/{}", target));
        let mut r = io::BufReader::new(arv.by_name(path_str.as_str())?);
        let mut buf = Vec::new();
        r.read_to_end(&mut buf)?;
        std::io::Cursor::new(buf)
    };
    let mut reader = Reader::from_reader(data);

    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
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
