use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use super::XlsxError;
use structs::Spreadsheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    // relationships
    write_start_tag(
        &mut writer,
        "Relationships",
        vec![(
            "xmlns",
            "http://schemas.openxmlformats.org/package/2006/relationships",
        )],
        false,
    );

    // relationship docProps/custom.xml
    //if spreadsheet.get_properties().get_custom_properties().len() > 0 {
    //    write_relationship(
    //        &mut writer,
    //        "4",
    //        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/custom-properties",
    //        "docProps/custom.xml",
    //        ""
    //    );
    //}

    // relationship docProps/app.xml
    write_relationship(
        &mut writer,
        "3",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties",
        "docProps/app.xml",
        "",
    );

    // relationship docProps/core.xml
    write_relationship(
        &mut writer,
        "2",
        "http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties",
        "docProps/core.xml",
        "",
    );

    // relationship docProps/core.xml
    write_relationship(
        &mut writer,
        "1",
        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument",
        "xl/workbook.xml",
        "",
    );

    // a custom UI in workbook ?
    if spreadsheet.has_ribbon() {
        write_relationship(
            &mut writer,
            "5",
            "http://schemas.microsoft.com/office/2006/relationships/ui/extensibility",
            "xl/todo.xml", //TODO
            "",
        );
    }

    write_end_tag(&mut writer, "Relationships");

    let target = "_rels/.rels";
    writer_mng.add_writer(target, writer)
}

fn write_relationship(
    writer: &mut Writer<io::Cursor<Vec<u8>>>,
    p_id: &str,
    p_type: &str,
    p_target: &str,
    p_target_mode: &str,
) {
    let tag_name = "Relationship";
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    let r_id = format!("rId{}", p_id);
    attributes.push(("Id", r_id.as_str()));
    attributes.push(("Type", p_type));
    attributes.push(("Target", p_target));
    if !p_target_mode.is_empty() {
        attributes.push(("TargetMode", p_target_mode));
    }
    write_start_tag(writer, tag_name, attributes, true);
}
