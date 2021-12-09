use std::{io, result};
use std::io::Read;
use super::XlsxError;
use super::driver::normalize_path;

pub fn read<R: io::Read + io::Seek>(arv: &mut zip::read::ZipArchive<R>, target: &String) -> result::Result<Vec<u8>, XlsxError> {
    let mut r = io::BufReader::new(arv.by_name(normalize_path(&format!("xl/drawings/{}", target)).to_str().unwrap_or(""))?);
    let mut buf = Vec::new();
    let _ = r.read_to_end(&mut buf)?;

    return Ok(buf);
}

