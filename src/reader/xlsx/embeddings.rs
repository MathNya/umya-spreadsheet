use std::{io, result};
use std::io::Read;
use super::XlsxError;
use super::driver::normalize_path_to_str;

pub fn read<R: io::Read + io::Seek>(arv: &mut zip::read::ZipArchive<R>, target: &str) -> result::Result<Vec<u8>, XlsxError> {
    let path_str = normalize_path_to_str(&format!("xl/embeddings/{}", target));
    let mut r = io::BufReader::new(arv.by_name(path_str.as_str())?);
    let mut buf = Vec::new();
    let _ = r.read_to_end(&mut buf)?;

    return Ok(buf);
}

