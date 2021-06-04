use std::result;
use std::fs::File;
use std::io::Read;
use tempdir::TempDir;
use super::XlsxError;

use ::structs::Spreadsheet;

const FILE_PATH: &'static str = "xl/vbaProject.bin";

pub(crate) fn read(dir: &TempDir, spreadsheet:&mut Spreadsheet) -> result::Result<(), XlsxError> {
    let path = dir.path().join(FILE_PATH);
    let mut file = match File::open(path) {
        Ok(v) => {v},
        Err(_) => {return Ok(());}
    };
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    spreadsheet.set_macros_code(buf);
    spreadsheet.set_has_macros(true);
    
    Ok(())
}