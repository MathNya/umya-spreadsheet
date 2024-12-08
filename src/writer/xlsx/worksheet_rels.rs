use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use super::XlsxError;
use crate::helper::const_str::*;
use crate::structs::Worksheet;
use crate::structs::WriterManager;

#[allow(clippy::too_many_arguments)]
pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    worksheet_no: &str,
    drawing_no: &str,
    vml_drawing_no: &str,
    comment_no: &str,
    ole_object_no_list: &[String],
    excel_no_list: &[String],
    printer_settings_no: &str,
    table_no_list: &[String],
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut is_write = false;

    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    // relationships
    write_start_tag(&mut writer, "Relationships", vec![("xmlns", REL_NS)], false);

    let mut r_id: i32 = 1;

    // Write hyperlink relationships
    for (_, hyperlink) in worksheet.get_hyperlink_collection_to_hashmap() {
        if !*hyperlink.get_location() {
            is_write = write_relationship(
                &mut writer,
                r_id.to_string().as_str(),
                HYPERLINK_NS,
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
            PRINTER_SETTINGS_NS,
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
            DRAWINGS_NS,
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
            VML_DRAWING_NS,
            format!(
                "../drawings/vmlDrawing{}.vml",
                vml_drawing_no.to_string().as_str()
            )
            .as_str(),
            "",
        );
        r_id += 1;
    }

    // write table relationships
    for table_no in table_no_list.iter() {
        is_write = write_relationship(
            &mut writer,
            r_id.to_string().as_str(),
            TABLE_NS,
            format!("../tables/table{}.xml", table_no.to_string().as_str()).as_str(),
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
                PACKAGE_NS,
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
                OLE_OBJECT_NS,
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
            IMAGE_NS,
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
            COMMENTS_NS,
            format!("../comments{}.xml", comment_no.to_string().as_str()).as_str(),
            "",
        );
    }

    write_end_tag(&mut writer, "Relationships");

    if is_write {
        let file_path = format!("{PKG_SHEET_RELS}{}.xml.rels", worksheet_no);
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
