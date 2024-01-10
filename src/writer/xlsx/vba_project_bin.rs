use std::io;

use super::XlsxError;
use helper::const_str::*;
use structs::Spreadsheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    match spreadsheet.get_has_macros() {
        true => {}
        false => return Ok(()),
    }
    let writer = spreadsheet.get_macros_code().as_ref().unwrap();
    writer_mng.add_bin(PKG_VBA_PROJECT, writer)
}
