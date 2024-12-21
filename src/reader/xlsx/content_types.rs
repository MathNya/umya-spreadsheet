use super::driver::{get_attribute, xml_read_loop};
use super::XlsxError;
use crate::helper::const_str::CONTENT_TYPES;
use crate::structs::Spreadsheet;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::io;

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> Result<(), XlsxError> {
    let r = io::BufReader::new(arv.by_name(CONTENT_TYPES)?);
    let mut reader = Reader::from_reader(r);
    reader.config_mut().trim_text(true);
    let mut list: Vec<(String, String)> = Vec::new();

    xml_read_loop!(
        reader,
        Event::Empty(ref e) => {
            if e.name().into_inner() == b"Override" {
                let part_name = get_attribute(e, b"PartName").unwrap();
                let content_type = get_attribute(e, b"ContentType").unwrap();
                list.push((part_name, content_type));
            }
        },
        Event::Eof => break,
    );

    spreadsheet.set_backup_context_types(list);
    Ok(())
}
