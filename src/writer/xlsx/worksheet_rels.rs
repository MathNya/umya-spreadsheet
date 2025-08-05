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
        COMMENTS_NS,
        DRAWINGS_NS,
        HYPERLINK_NS,
        IMAGE_NS,
        OLE_OBJECT_NS,
        PACKAGE_NS,
        PKG_SHEET_RELS,
        PRINTER_SETTINGS_NS,
        REL_NS,
        TABLE_NS,
        THREADED_COMMENT_NS,
        VML_DRAWING_NS,
    },
    structs::{
        Worksheet,
        WriterManager,
    },
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn write<W: io::Seek + io::Write>(
    worksheet: &Worksheet,
    worksheet_no: &str,
    drawing_no: &str,
    vml_drawing_no: &str,
    comment_no: &str,
    threaded_comment_no: &str,
    ole_object_no_list: &[String],
    excel_no_list: &[String],
    printer_settings_no: &str,
    table_no_list: &[String],
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let mut is_write = false;

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

    let mut r_id: i32 = 1;

    // Write hyperlink relationships
    for (_, hyperlink) in worksheet.hyperlink_collection() {
        if !hyperlink.location() {
            is_write = write_relationship(
                &mut writer,
                &r_id.to_string(),
                HYPERLINK_NS,
                hyperlink.url(),
                "External",
            );
            r_id += 1;
        }
    }

    // write pageSetup
    if worksheet.page_setup().object_data().is_some() {
        let object_name = format!("printerSettings{printer_settings_no}.bin");
        is_write = write_relationship(
            &mut writer,
            &r_id.to_string(),
            PRINTER_SETTINGS_NS,
            format!("../printerSettings/{object_name}").as_str(),
            "",
        );
        r_id += 1;
    }

    // write drawing relationships
    if worksheet.has_drawing_object() {
        is_write = write_relationship(
            &mut writer,
            &r_id.to_string(),
            DRAWINGS_NS,
            format!("../drawings/drawing{drawing_no}.xml").as_str(),
            "",
        );
        r_id += 1;
    }

    // Write vmlDrawing relationship
    if worksheet.has_legacy_drawing() {
        is_write = write_relationship(
            &mut writer,
            &r_id.to_string(),
            VML_DRAWING_NS,
            format!("../drawings/vmlDrawing{vml_drawing_no}.vml").as_str(),
            "",
        );
        r_id += 1;
    }

    // write table relationships
    for table_no in table_no_list {
        is_write = write_relationship(
            &mut writer,
            &r_id.to_string(),
            TABLE_NS,
            format!("../tables/table{table_no}.xml").as_str(),
            "",
        );
        r_id += 1;
    }

    // Write ole_objects
    let mut excel_no_list = excel_no_list.iter();
    let mut ole_object_no_list = ole_object_no_list.iter();
    for ole_object in worksheet.ole_objects().ole_object() {
        if ole_object.is_xlsx() {
            let excel_no = excel_no_list.next().unwrap();
            let object_name = format!("Microsoft_Excel_Worksheet{excel_no}.xlsx");
            write_relationship(
                &mut writer,
                &r_id.to_string(),
                PACKAGE_NS,
                format!("../embeddings/{object_name}").as_str(),
                "",
            );
            r_id += 1;
        }
        if ole_object.is_bin() {
            let ole_object_no = ole_object_no_list.next().unwrap();
            let object_name = format!("oleObject{ole_object_no}.bin");
            write_relationship(
                &mut writer,
                &r_id.to_string(),
                OLE_OBJECT_NS,
                format!("../embeddings/{object_name}").as_str(),
                "",
            );
            r_id += 1;
        }

        let image_name = ole_object.embedded_object_properties().image().image_name();
        is_write = write_relationship(
            &mut writer,
            &r_id.to_string(),
            IMAGE_NS,
            format!("../media/{image_name}").as_str(),
            "",
        );
        r_id += 1;
    }

    // Write comments relationship
    if worksheet.has_comments() {
        is_write = write_relationship(
            &mut writer,
            &r_id.to_string(),
            COMMENTS_NS,
            format!("../comments{comment_no}.xml").as_str(),
            "",
        );
        r_id += 1;
    }

    // Write threadedComment relationship
    if worksheet.has_threaded_comments() {
        is_write = write_relationship(
            &mut writer,
            &r_id.to_string(),
            THREADED_COMMENT_NS,
            format!("../threadedComments/threadedComment{threaded_comment_no}.xml").as_str(),
            "",
        );
    }

    write_end_tag(&mut writer, "Relationships");

    if is_write {
        let file_path = format!("{PKG_SHEET_RELS}{worksheet_no}.xml.rels");
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
    let mut attributes: crate::structs::AttrCollection = Vec::new();
    let r_id = format!("rId{p_id}");
    attributes.push(("Id", &r_id).into());
    attributes.push(("Type", p_type).into());
    attributes.push(("Target", p_target).into());
    if !p_target_mode.is_empty() {
        attributes.push(("TargetMode", p_target_mode).into());
    }
    write_start_tag(writer, tag_name, attributes, true);
    true
}
