use super::XlsxError;
use std::io::Read;
use std::{io, result};

use structs::Spreadsheet;

const FILE_PATH: &str = "xl/vbaProject.bin";

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> result::Result<(), XlsxError> {
    let mut r = io::BufReader::new(match arv.by_name(FILE_PATH) {
        Ok(v) => v,
        Err(zip::result::ZipError::FileNotFound) => {
            return Ok(());
        }
        Err(e) => {
            return Err(e.into());
        }
    });
    let mut buf = Vec::new();
    let _ = r.read_to_end(&mut buf)?;

    spreadsheet.set_macros_code(buf);

    Ok(())
}
