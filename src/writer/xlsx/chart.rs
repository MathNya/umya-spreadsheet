use super::driver::*;
use super::XlsxError;
use crate::structs::drawing::charts::ChartSpace;
use crate::structs::Spreadsheet;
use crate::structs::WriterManager;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

pub(crate) fn write<W: io::Seek + io::Write>(
    chart_space: &ChartSpace,
    spreadsheet: &Spreadsheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<String, XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    // c:chartSpace
    chart_space.write_to(&mut writer, spreadsheet);

    let file_no = writer_mng.add_file_at_chart(writer)?;
    Ok(file_no.to_string())
}
