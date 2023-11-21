use crate::xml_read_loop;

use super::XlsxError;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};
use structs::SharedStringTable;
use structs::Spreadsheet;

const FILE_PATH: &str = "xl/sharedStrings.xml";

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> result::Result<(), XlsxError> {
    let r = io::BufReader::new(match arv.by_name(FILE_PATH) {
        Ok(v) => v,
        Err(zip::result::ZipError::FileNotFound) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e.into());
        }
    });
    let mut reader = Reader::from_reader(r);
    reader.trim_text(false);

    let theme = spreadsheet.get_theme().clone();

    xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() == b"sst" {
                let mut obj = SharedStringTable::default();
                obj.set_attributes(&mut reader, e, &theme);
                spreadsheet.set_shared_string_table(obj);
            }
        },
        Event::Eof => break,
    );

    Ok(())
}
