use super::driver::normalize_path_to_str;
use super::XlsxError;
use std::io::Read;
use std::{io, result};

pub fn read<R: io::Read + io::Seek>(
    arv: &mut zip::read::ZipArchive<R>,
    target: &str,
) -> result::Result<Vec<u8>, XlsxError> {
    let path_str = normalize_path_to_str(&format!("xl/printerSettings/{}", target));
    let mut r = io::BufReader::new(arv.by_name(path_str.as_str())?);
    let mut buf = Vec::new();
    let _ = r.read_to_end(&mut buf)?;

    return Ok(buf);
}
