use std::io;

use quick_xml::{
    Writer,
    events::{
        BytesDecl,
        Event,
    },
};

use super::{
    XlsxError,
    driver::write_new_line,
};
use crate::structs::{
    Spreadsheet,
    WriterManager,
    drawing::charts::ChartSpace,
};

pub(crate) fn write<W: io::Seek + io::Write>(
    chart_space: &ChartSpace,
    spreadsheet: &Spreadsheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<String, XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer
        .write_event(Event::Decl(BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            Some("yes"),
        )))
        .unwrap();
    write_new_line(&mut writer);

    // c:chartSpace
    chart_space.write_to(&mut writer, spreadsheet);

    let file_no = writer_mng.add_file_at_chart(writer)?;
    Ok(file_no.to_string())
}
