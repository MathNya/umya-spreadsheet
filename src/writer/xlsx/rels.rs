use std::io;

use quick_xml::{
    events::{BytesDecl, Event},
    Writer,
};

use super::{
    driver::{write_end_tag, write_new_line, write_start_tag},
    XlsxError,
};
use crate::{
    helper::const_str::{
        ARC_APP, ARC_CORE, ARC_CUSTOM, COREPROPS_REL, CUSTOMUI_NS, CUSTOM_PROPS_REL, OFCDOC_NS,
        PKG_WORKBOOK, REL_NS, XPROPS_REL,
    },
    structs::{Workbook, WriterManager},
};

pub(crate) fn write<W: io::Seek + io::Write>(
    wb: &Workbook,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
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

    // relationships
    write_start_tag(
        &mut writer,
        "Relationships",
        vec![("xmlns", REL_NS).into()],
        false,
    );

    // relationship docProps/app.xml
    write_relationship(&mut writer, "3", XPROPS_REL, ARC_APP, "");

    // relationship docProps/core.xml
    write_relationship(&mut writer, "2", COREPROPS_REL, ARC_CORE, "");

    // relationship docProps/core.xml
    write_relationship(&mut writer, "1", OFCDOC_NS, PKG_WORKBOOK, "");

    // relationship docProps/custom.xml
    if !wb
        .get_properties()
        .get_custom_properties()
        .get_custom_document_property_list()
        .is_empty()
    {
        write_relationship(&mut writer, "4", CUSTOM_PROPS_REL, ARC_CUSTOM, "");
    }

    // a custom UI in workbook ?
    if wb.has_ribbon() {
        write_relationship(
            &mut writer,
            "5",
            CUSTOMUI_NS,
            "xl/todo.xml", // TODO
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
    let mut attributes: crate::structs::AttrCollection = Vec::new();
    let r_id = format!("rId{p_id}");
    attributes.push(("Id", r_id.as_str()).into());
    attributes.push(("Type", p_type).into());
    attributes.push(("Target", p_target).into());
    if !p_target_mode.is_empty() {
        attributes.push(("TargetMode", p_target_mode).into());
    }
    write_start_tag(writer, tag_name, attributes, true);
}
