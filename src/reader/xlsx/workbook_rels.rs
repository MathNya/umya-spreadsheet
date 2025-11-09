use super::driver::*;
use super::XlsxError;
use crate::helper::const_str::*;
use crate::structs::Spreadsheet;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::{io, result};

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> result::Result<Vec<(String, String, String)>, XlsxError> {
    let r = io::BufReader::new(arv.by_name(PKG_WORKBOOK_RELS)?);
    let mut reader = Reader::from_reader(r);
    reader.config_mut().trim_text(true);

    let mut result: Vec<(String, String, String)> = Vec::new();

    xml_read_loop!(
        reader,
        Event::Empty(ref e) => {
            if e.name().into_inner() == b"Relationship" {
                let id_value = get_attribute(e, b"Id").unwrap();
                let type_value = get_attribute(e, b"Type").unwrap();
                let target_value = get_attribute(e, b"Target").unwrap();
                let target_value = target_value
                    .strip_prefix("/xl/")
                    .map(|t| t.to_owned())
                    .unwrap_or(target_value);
                if type_value == PIVOT_CACHE_DEF_NS {
                    spreadsheet.update_pivot_caches(id_value.clone(), target_value.clone());
                }
                result.push((id_value, type_value, target_value));
            }
        },
        Event::Eof => break,
    );

    Ok(result)
}
