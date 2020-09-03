use tempdir::TempDir;

use super::super::structs::spreadsheet::Spreadsheet;
use super::driver::*;
use super::XlsxError;

pub(crate) fn write(spreadsheet: &Spreadsheet, dir: &TempDir, sub_dir: &str, file_name: &str) -> Result<(), XlsxError> {
    match spreadsheet.get_has_macros() {
        &true => {},
        &false => return Ok(())
    }
    let writer = spreadsheet.get_macros_code().as_ref().unwrap();
    let _ = make_file_from_bin(format!("{}/{}",sub_dir,file_name).as_str(), dir, writer, Some(sub_dir)).unwrap();
    Ok(())
}