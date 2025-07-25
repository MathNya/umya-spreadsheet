use std::io;

use super::XlsxError;
use crate::helper::const_str::*;
use crate::structs::Spreadsheet;
use crate::structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    if !spreadsheet.get_has_jsa_macros() {
        return Ok(());
    }
    let writer = spreadsheet.get_jsa_macros_code().unwrap();
    writer_mng.add_bin(PKG_JSA_PROJECT, writer)
}
