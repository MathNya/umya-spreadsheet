use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use super::XlsxError;
use crate::helper::const_str::*;
use crate::structs::Spreadsheet;
use crate::structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    ))).unwrap();
    write_new_line(&mut writer);

    // relationships
    write_start_tag(&mut writer, "Relationships", vec![("xmlns", REL_NS)], false);

    // relationship docProps/app.xml
    write_relationship(&mut writer, "3", XPROPS_REL, ARC_APP, "");

    // relationship docProps/core.xml
    write_relationship(&mut writer, "2", COREPROPS_REL, ARC_CORE, "");

    // relationship docProps/core.xml
    write_relationship(&mut writer, "1", OFCDOC_NS, PKG_WORKBOOK, "");

    // relationship docProps/custom.xml
    if !spreadsheet
        .get_properties()
        .get_custom_properties()
        .get_custom_document_property_list().is_empty()
    {
        write_relationship(&mut writer, "4", CUSTOM_PROPS_REL, ARC_CUSTOM, "");
    }

    // a custom UI in workbook ?
    if spreadsheet.has_ribbon() {
        write_relationship(
            &mut writer,
            "5",
            CUSTOMUI_NS,
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
