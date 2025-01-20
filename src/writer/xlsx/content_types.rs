use std::io;

use quick_xml::{
    Writer,
    events::{BytesDecl, Event},
};

use super::{
    XlsxError,
    driver::{make_file_from_writer, write_end_tag, write_new_line, write_start_tag},
};
use crate::{
    helper::const_str::{
        CONTENT_TYPES, CONTYPES_NS, PRNTR_SETTINGS_TYPE, REL_TYPE, VML_DRAWING_TYPE, WORKBOOK,
    },
    structs::{Workbook, WriterManager},
};

pub(crate) fn write<W: io::Seek + io::Write>(
    wb: &Workbook,
    writer_mng: &mut WriterManager<W>,
) -> Result<(), XlsxError> {
    let is_light = writer_mng.get_is_light();
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

    // Types
    write_start_tag(
        &mut writer,
        "Types",
        vec![("xmlns", CONTYPES_NS).into()],
        false,
    );

    // Write default content types
    let default_content_types = [("rels", REL_TYPE), ("xml", "application/xml")];

    for (extension, content_type) in default_content_types {
        write_start_tag(
            &mut writer,
            "Default",
            vec![
                ("Extension", extension).into(),
                ("ContentType", content_type).into(),
            ],
            true,
        );
    }

    // Write additional content types based on extensions
    let optional_extensions = [
        ("bin", PRNTR_SETTINGS_TYPE),
        ("vml", VML_DRAWING_TYPE),
        ("png", "image/png"),
        ("jpg", "image/jpeg"),
        ("jpeg", "image/jpeg"),
        ("tiff", "image/tiff"),
        ("emf", "image/x-emf"),
        ("xlsx", WORKBOOK),
    ];

    for (extension, content_type) in optional_extensions {
        if writer_mng.has_extension(extension) {
            write_start_tag(
                &mut writer,
                "Default",
                vec![
                    ("Extension", extension).into(),
                    ("ContentType", content_type).into(),
                ],
                true,
            );
        }
    }

    // Override
    for (part_name, content_type) in writer_mng.make_context_type_override(wb) {
        write_start_tag(
            &mut writer,
            "Override",
            vec![
                ("PartName", &part_name).into(),
                ("ContentType", &content_type).into(),
            ],
            true,
        );
    }

    write_end_tag(&mut writer, "Types");
    make_file_from_writer(
        CONTENT_TYPES,
        writer_mng.get_arv_mut(),
        writer,
        None,
        is_light,
    )?;
    Ok(())
}
