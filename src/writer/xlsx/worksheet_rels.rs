use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io;

use ::structs::Worksheet;
use super::driver::*;
use super::XlsxError;

const SUB_DIR: &'static str = "xl/worksheets/_rels";

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    p_worksheet_id: &str,
    drawing_id: &usize,
    comment_id: &usize,
    arv: &mut zip::ZipWriter<W>
) -> Result<(), XlsxError> {
    let file_name = format!("sheet{}.xml.rels", p_worksheet_id);
    let mut is_write = false;

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    // relationships
    write_start_tag(&mut writer, "Relationships", vec![
        ("xmlns", "http://schemas.openxmlformats.org/package/2006/relationships"),
    ], false);

    // write drawing relationships
    let mut id = 1;
    if worksheet.has_drawing_object() {
        is_write = write_relationship(
            &mut writer,
            id.to_string().as_str(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
            format!("../drawings/drawing{}.xml", drawing_id.to_string().as_str()).as_str(),
            ""
        );
        id += 1;
    }

    // Write hyperlink relationships
    for (_, hyperlink) in worksheet.get_hyperlink_collection() {
        if hyperlink.get_location() == &false {
            is_write = write_relationship(
                &mut writer,
                id.to_string().as_str(),
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink",
                hyperlink.get_url(),
                "External"
            );
            id+=1;
        }
    }

    // Write comments relationship
    if worksheet.get_comments().len() > 0 {
        write_relationship(
            &mut writer,
            id.to_string().as_str(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
            format!("../drawings/vmlDrawing{}.vml", comment_id.to_string().as_str()).as_str(),
            ""
        );
        id+=1;
        is_write = write_relationship(
            &mut writer,
            id.to_string().as_str(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
            format!("../comments{}.xml", comment_id.to_string().as_str()).as_str(),
            ""
        );
    }

    // Write header/footer relationship
    //let i = 1;
    //if worksheet.get_header_footer().get_header_footer_images().len() > 0 {
    //    is_write = write_relationship(
    //        &mut writer,
    //        format!("_headerfooter_vml{}", i.to_string().as_str()).as_str(),
    //        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
    //        format!("../drawings/vmlDrawingHF{}.xml", p_worksheet_id.to_string().as_str()).as_str(),
    //        ""
    //    );
    //}

    write_end_tag(&mut writer, "Relationships");

    if is_write {
        let _ = make_file_from_writer(&file_name, arv, writer, Some(SUB_DIR)).unwrap();
    }
    Ok(())
}


fn write_relationship(writer: &mut Writer<io::Cursor<Vec<u8>>>, p_id: &str, p_type: &str, p_target: &str, p_target_mode: &str)-> bool
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