use std::{io, result};
use quick_xml::Reader;
use quick_xml::events::{Event};
use super::XlsxError;
use structs::drawing::charts::ChartSpace;
use super::driver::normalize_path;

pub(crate) fn read<R: io::Read + io::Seek>(arv: &mut zip::read::ZipArchive<R>, target: &String, chart_space: &mut ChartSpace) -> result::Result<(), XlsxError> {
    let r = io::BufReader::new(arv.by_name(normalize_path(&format!("xl/drawings/{}", target)).to_str().unwrap_or(""))?);
    let mut reader = Reader::from_reader(r);
    reader.trim_text(true);
    let mut buf = Vec::new();

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                match e.name() {
                    b"c:chartSpace" => {
                        chart_space.set_attributes(&mut reader, e);
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
