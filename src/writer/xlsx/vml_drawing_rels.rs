use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use super::XlsxError;
use structs::Worksheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    vml_drawing_no: &str,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut is_write = false;

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

    let mut r_id = 1;
    for ole_object in worksheet.get_ole_objects().get_ole_object() {
        match ole_object.get_shape().get_image_data() {
            Some(v) => {
                is_write = write_relationship(
                    &mut writer,
                    &r_id,
                    "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
                    format!("../media/{}", v.get_image_name()).as_str(),
                    "",
                );
                r_id += 1;
            }
            None => {}
        }
    }

    write_end_tag(&mut writer, "Relationships");

    if is_write {
        let file_path = format!("xl/drawings/_rels/vmlDrawing{}.vml.rels", vml_drawing_no);
        return writer_mng.add_writer(&file_path, writer);
    }
    Ok(())
}

fn write_relationship(
    writer: &mut Writer<io::Cursor<Vec<u8>>>,
    r_id: &i32,
    p_type: &str,
    p_target: &str,
    p_target_mode: &str,
) -> bool {
    let tag_name = "Relationship";
    let r_id_str = format!("rId{}", r_id);
    let mut attributes: Vec<(&str, &str)> = Vec::new();
    attributes.push(("Id", &r_id_str));
    attributes.push(("Type", p_type));
    attributes.push(("Target", p_target));
    if !p_target_mode.is_empty() {
        attributes.push(("TargetMode", p_target_mode));
    }
    write_start_tag(writer, tag_name, attributes, true);
    true
}
