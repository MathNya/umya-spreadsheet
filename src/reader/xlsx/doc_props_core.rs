use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};

use helper::const_str::*;
use structs::Spreadsheet;

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> result::Result<(), XlsxError> {
    let r = io::BufReader::new(match arv.by_name(ARC_CORE) {
        Ok(v) => v,
        Err(zip::result::ZipError::FileNotFound) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e.into());
        }
    });
    let mut reader = Reader::from_reader(r);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                if e.name().into_inner() == b"cp:coreProperties" {
                    spreadsheet
                        .get_properties_mut()
                        .set_attributes_core(&mut reader, e);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }
    Ok(())
}
