use std::io;

use quick_xml::{events::Event, Reader};

use super::XlsxError;
use crate::{
    helper::const_str::PKG_SHARED_STRINGS,
    structs::{SharedStringTable, Workbook},
    xml_read_loop,
};

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    wb: &mut Workbook,
) -> Result<(), XlsxError> {
    let r = io::BufReader::new(match arv.by_name(PKG_SHARED_STRINGS) {
        Ok(v) => v,
        Err(zip::result::ZipError::FileNotFound) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e.into());
        }
    });
    let mut reader = Reader::from_reader(r);
    reader.config_mut().trim_text(false);

    xml_read_loop!(
        reader,
        Event::Start(ref e) => {
            if e.name().into_inner() == b"sst" {
                let mut obj = SharedStringTable::default();
                obj.set_attributes(&mut reader, e);
                wb.set_shared_string_table(obj);
            }
        },
        Event::Eof => break,
    );

    Ok(())
}
