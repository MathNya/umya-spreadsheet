use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use super::XlsxError;
use structs::Worksheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    worksheet_no: &str,
    drawing_no: &str,
    vml_drawing_no: &str,
    comment_no: &str,
    ole_object_no_list: &Vec<String>,
    excel_no_list: &Vec<String>,
    printer_settings_no: &str,
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

    let mut r_id: i32 = 1;

    // Write hyperlink relationships
    for (_, hyperlink) in worksheet.get_hyperlink_collection_to_hashmap() {
        if hyperlink.get_location() == &false {
            is_write = write_relationship(
                &mut writer,
                r_id.to_string().as_str(),
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink",
                hyperlink.get_url(),
                "External",
            );
            r_id += 1;
        }
    }

    // write pageSetup
    if worksheet.get_page_setup().get_object_data().is_some() {
        let object_name = format!("printerSettings{}.bin", printer_settings_no);
        is_write = write_relationship(
            &mut writer,
            r_id.to_string().as_str(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/printerSettings",
            format!("../printerSettings/{}", object_name).as_str(),
            "",
        );
        r_id += 1;
    }

    // write drawing relationships
    if worksheet.has_drawing_object() {
        is_write = write_relationship(
            &mut writer,
            r_id.to_string().as_str(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/drawing",
            format!("../drawings/drawing{}.xml", drawing_no.to_string().as_str()).as_str(),
            "",
        );
        r_id += 1;
    }

    // Write vmlDrawing relationship
    if worksheet.has_legacy_drawing() {
        is_write = write_relationship(
            &mut writer,
            r_id.to_string().as_str(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/vmlDrawing",
            format!(
                "../drawings/vmlDrawing{}.vml",
                vml_drawing_no.to_string().as_str()
            )
            .as_str(),
            "",
        );
        r_id += 1;
    }

    // Write ole_objects
    let mut excel_no_list = excel_no_list.iter();
    let mut ole_object_no_list = ole_object_no_list.iter();
    for ole_object in worksheet.get_ole_objects().get_ole_object() {
        if ole_object.is_xlsx() {
            let excel_no = excel_no_list.next().unwrap();
            let object_name = format!("Microsoft_Excel_Worksheet{}.xlsx", excel_no);
            write_relationship(
                &mut writer,
                r_id.to_string().as_str(),
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/package",
                format!("../embeddings/{}", object_name).as_str(),
                "",
            );
            r_id += 1;
        }
        if ole_object.is_bin() {
            let ole_object_no = ole_object_no_list.next().unwrap();
            let object_name = format!("oleObject{}.bin", ole_object_no);
            write_relationship(
                &mut writer,
                r_id.to_string().as_str(),
                "http://schemas.openxmlformats.org/officeDocument/2006/relationships/oleObject",
                format!("../embeddings/{}", object_name).as_str(),
                "",
            );
            r_id += 1;
        }

        let image_name = ole_object
            .get_embedded_object_properties()
            .get_image()
            .get_image_name();
        is_write = write_relationship(
            &mut writer,
            r_id.to_string().as_str(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/image",
            format!("../media/{}", image_name).as_str(),
            "",
        );
        r_id += 1;
    }

    // Write comments relationship
    if !worksheet.get_comments().is_empty() {
        is_write = write_relationship(
            &mut writer,
            r_id.to_string().as_str(),
            "http://schemas.openxmlformats.org/officeDocument/2006/relationships/comments",
            format!("../comments{}.xml", comment_no.to_string().as_str()).as_str(),
            "",
        );
    }

    write_end_tag(&mut writer, "Relationships");

    if is_write {
        let file_path = format!("xl/worksheets/_rels/sheet{}.xml.rels", worksheet_no);
        writer_mng.add_writer(&file_path, writer)?;
    }
    Ok(())
}

fn write_relationship(
    writer: &mut Writer<io::Cursor<Vec<u8>>>,
    p_id: &str,
    p_type: &str,
    p_target: &str,
    p_target_mode: &str,
) -> bool {
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
    true
}
