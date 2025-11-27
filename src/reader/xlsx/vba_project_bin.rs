use std::{
    io,
    io::Read,
};

use super::XlsxError;
use crate::{
    helper::const_str::PKG_VBA_PROJECT,
    structs::Workbook,
};

pub(crate) fn read<R: Read + io::Seek>(
    arv: &mut zip::ZipArchive<R>,
    wb: &mut Workbook,
) -> Result<(), XlsxError> {
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
    r.read_to_end(&mut buf)?;

    wb.set_macros_code(buf);

    Ok(())
}
