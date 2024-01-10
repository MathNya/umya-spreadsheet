use super::XlsxError;
use std::io::Read;
use std::{io, result};

use helper::const_str::*;
use structs::Spreadsheet;

pub(crate) fn read<R: io::Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    spreadsheet: &mut Spreadsheet,
) -> result::Result<(), XlsxError> {
    let mut r = io::BufReader::new(match arv.by_name(PKG_VBA_PROJECT) {
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
