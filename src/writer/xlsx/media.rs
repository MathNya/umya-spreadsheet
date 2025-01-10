use std::io;

use super::XlsxError;
use crate::helper::const_str::*;
use crate::structs::Worksheet;
use crate::structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    for image in worksheet.get_media_object_collection() {
        let file_name = format!("{PKG_MEDIA}/{}", image.get_image_name());
        writer_mng.add_bin(&file_name, image.get_image_data())?;
    }
    Ok(())
}
