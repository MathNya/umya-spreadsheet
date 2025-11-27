use std::io;

use super::XlsxError;
use crate::{
    helper::const_str::PKG_JSA_PROJECT,
    structs::{
        Workbook,
        WriterManager,
    },
};

pub(crate) fn write<W: io::Seek + io::Write>(
    wb: &Workbook,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    if !wb.has_jsa_macros() {
        return Ok(());
    }
    let writer = wb.jsa_macros_code().unwrap();
    writer_mng.add_bin(PKG_JSA_PROJECT, writer)
}
