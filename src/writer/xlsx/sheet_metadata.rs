use std::io;

use super::XlsxError;
use crate::{
    helper::const_str::PKG_SHEET_METADATA,
    structs::{
        Workbook,
        WriterManager,
    },
};

pub(crate) fn write<W: io::Seek + io::Write>(
    wb: &Workbook,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let Some(data) = wb.sheet_metadata() else {
        return Ok(());
    };
    writer_mng.add_bin(PKG_SHEET_METADATA, data)
}
