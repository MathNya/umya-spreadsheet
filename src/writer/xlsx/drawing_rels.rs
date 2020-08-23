use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;

use super::super::structs::worksheet::Worksheet;
use super::driver::*;
use super::XlsxError;

const SUB_DIR: &'static str = "xl/drawings/_rels";

pub(crate) fn write(
    worksheet: &Worksheet,
    p_worksheet_id: &str,
    chart_start_id: &usize,
    dir: &TempDir
) -> Result<(), XlsxError> 
{
    let file_name = format!("drawing{}.xml.rels", p_worksheet_id);
    let charts = worksheet.get_chart_collection();
    let mut is_write = false;

    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    // relationships
    write_start_tag(&mut writer, "Relationships", vec![
        ("xmlns", "http://schemas.openxmlformats.org/package/2006/relationships"),
    ], false);

    let mut id = 1;
    let mut chart_id = chart_start_id.clone();
    for _ in charts {
        is_write = write_relationship(
            &mut writer,
            id.to_string().as_str(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/chart",
            format!("../charts/chart{}.xml", chart_id).as_str(),
            ""
        );
        id += 1;
        chart_id += 1;
    }

    write_end_tag(&mut writer, "Relationships");

    if is_write {
        let _ = make_file_from_writer(format!("{}/{}", SUB_DIR, file_name).as_str(), dir, writer, Some(SUB_DIR)).unwrap();
    }
    Ok(())
}


fn write_relationship(writer: &mut Writer<Cursor<Vec<u8>>>, p_id: &str, p_type: &str, p_target: &str, p_target_mode: &str) -> bool
{
    let tag_name = "Relationship";
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    let r_id = format!("rId{}", p_id);
    attributes.push(("Id", r_id.as_str()));
    attributes.push(("Type", p_type));
    attributes.push(("Target", p_target));
    if p_target_mode != "" {
        attributes.push(("TargetMode", p_target_mode));
    }
    write_start_tag(writer, tag_name, attributes, true);
    true
}