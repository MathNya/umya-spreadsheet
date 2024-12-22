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
    driver::{
        write_end_tag,
        write_new_line,
        write_start_tag,
    },
};
use crate::{
    helper::const_str::{
        IMAGE_NS,
        PKG_VML_DRAWING_RELS,
        REL_NS,
    },
    structs::{
        Worksheet,
        WriterManager,
    },
};

pub(crate) fn write<W: io::Seek + io::Write>(
    _worksheet: &Worksheet,
    vml_drawing_no: &str,
    rel_list: &[(String, String)],
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut is_write = false;

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), Some("yes")))).unwrap();
    write_new_line(&mut writer);

    // relationships
    write_start_tag(&mut writer, "Relationships", vec![("xmlns", REL_NS)], false);

    let mut r_id = 1;
    for (key, value) in rel_list {
        if key == "IMAGE" {
            is_write = write_relationship(
                &mut writer,
                r_id,
                IMAGE_NS,
                format!("../media/{value}").as_str(),
                "",
            );
        }
        r_id += 1;
    }

    write_end_tag(&mut writer, "Relationships");

    if is_write {
        let file_path = format!("{PKG_VML_DRAWING_RELS}{vml_drawing_no}.vml.rels");
        return writer_mng.add_writer(&file_path, writer);
    }
    Ok(())
}

fn write_relationship(
    writer: &mut Writer<io::Cursor<Vec<u8>>>,
    r_id: i32,
    p_type: &str,
    p_target: &str,
    p_target_mode: &str,
) -> bool {
    let tag_name = "Relationship";
    let r_id_str = format!("rId{r_id}");
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
