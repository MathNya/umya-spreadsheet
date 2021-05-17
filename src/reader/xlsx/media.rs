use std::result;
use std::fs::File;
use std::io::Read;
use tempdir::TempDir;
use super::XlsxError;

pub fn read(dir: &TempDir, target: &String) -> result::Result<Vec<u8>, XlsxError> {
    let path = dir.path().join(format!("xl/drawings/{}", target));
    let mut file = match File::open(path) {
        Ok(v) => {v},
        Err(_) => {panic!("Error not find image.")}
    };
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf)?;

    return Ok(buf);
}

