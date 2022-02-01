use super::driver::*;
use super::XlsxError;
use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;
use structs::drawing::charts::ChartSpace;
use structs::Spreadsheet;

const SUB_DIR: &'static str = "xl/charts";

pub(crate) fn write<W: io::Seek + io::Write>(
    chart_space: &ChartSpace,
    p_chart_id: &usize,
    arv: &mut zip::ZipWriter<W>,
    spreadsheet: &Spreadsheet,
) -> Result<(), XlsxError> {
    let file_name = format!("chart{}.xml", p_chart_id);

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(
        b"1.0",
        Some(b"UTF-8"),
        Some(b"yes"),
    )));
    write_new_line(&mut writer);

    // c:chartSpace
    chart_space.write_to(&mut writer, spreadsheet);

    let _ = make_file_from_writer(&file_name, arv, writer, Some(SUB_DIR)).unwrap();
    Ok(())
}
