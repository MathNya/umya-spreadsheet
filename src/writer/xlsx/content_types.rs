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
    let is_light = writer_mng.get_is_light().clone();
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        Some("yes"),
    )));
    write_new_line(&mut writer);

    // Types
    write_start_tag(
        &mut writer,
        "Types",
        vec![(
            "xmlns",
            "http://schemas.openxmlformats.org/package/2006/content-types",
        )],
        false,
    );

    // Default rels
    write_start_tag(
        &mut writer,
        "Default",
        vec![
            ("Extension", "rels"),
            (
                "ContentType",
                "application/vnd.openxmlformats-package.relationships+xml",
            ),
        ],
        true,
    );

    // Default xml
    write_start_tag(
        &mut writer,
        "Default",
        vec![("Extension", "xml"), ("ContentType", "application/xml")],
        true,
    );

    // Default bin
    if writer_mng.has_extension("bin") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![
                ("Extension", "bin"),
                (
                    "ContentType",
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.printerSettings",
                ),
            ],
            true,
        );
    }

    // Default vml
    if writer_mng.has_extension("vml") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![
                ("Extension", "vml"),
                (
                    "ContentType",
                    "application/vnd.openxmlformats-officedocument.vmlDrawing",
                ),
            ],
            true,
        );
    }

    // Default png
    if writer_mng.has_extension("png") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "png"), ("ContentType", "image/png")],
            true,
        );
    }

    // Default jpg
    if writer_mng.has_extension("jpg") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "jpg"), ("ContentType", "image/jpeg")],
            true,
        );
    }

    // Default jpeg
    if writer_mng.has_extension("jpeg") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "jpeg"), ("ContentType", "image/jpeg")],
            true,
        );
    }

    // Default tiff
    if writer_mng.has_extension("tiff") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "tiff"), ("ContentType", "image/tiff")],
            true,
        );
    }

    // Default emf
    if writer_mng.has_extension("emf") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![("Extension", "emf"), ("ContentType", "image/x-emf")],
            true,
        );
    }

    // Default xlsx
    if writer_mng.has_extension("xlsx") {
        write_start_tag(
            &mut writer,
            "Default",
            vec![
                ("Extension", "xlsx"),
                (
                    "ContentType",
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                ),
            ],
            true,
        );
    }

    // Override
    for (part_name, content_type) in writer_mng.make_context_type_override(spreadsheet) {
        write_start_tag(
            &mut writer,
            "Override",
            vec![("PartName", &part_name), ("ContentType", &content_type)],
            true,
        );
    }

    write_end_tag(&mut writer, "Types");
    make_file_from_writer(
        "[Content_Types].xml",
        writer_mng.get_arv_mut(),
        writer,
        None,
        &is_light,
    )?;
    Ok(())
}
