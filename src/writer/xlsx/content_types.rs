use quick_xml::events::{BytesDecl, Event};
use quick_xml::Writer;
use std::io;

use super::driver::*;
use structs::Spreadsheet;
use structs::WriterManager;

pub(crate) fn write<W: io::Seek + io::Write>(
    spreadsheet: &Spreadsheet,
    writer_mng: &mut WriterManager<W>,
) {
    let mut writer = Writer::new(io::Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(
        b"1.0",
        Some(b"UTF-8"),
        Some(b"yes"),
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

    // Override workbook
    let content_type = match spreadsheet.get_has_macros() {
        true => "application/vnd.ms-excel.sheet.macroEnabled.main+xml",
        false => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
    };
    write_start_tag(
        &mut writer,
        "Override",
        vec![
            ("PartName", "/xl/workbook.xml"),
            ("ContentType", content_type),
        ],
        true,
    );

    // Override sheet
    let mut index = 1;
    for _ in spreadsheet.get_sheet_collection_no_check() {
        let path_str = format!("/xl/worksheets/sheet{}.xml", index);
        write_start_tag(
            &mut writer,
            "Override",
            vec![
                ("PartName", &path_str),
                (
                    "ContentType",
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml",
                ),
            ],
            true,
        );
        index += 1;
    }

    // Override comments
    for path_str in writer_mng.has_find("xl/comments") {
        write_start_tag(
            &mut writer,
            "Override",
            vec![
                ("PartName", &path_str),
                (
                    "ContentType",
                    "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml",
                ),
            ],
            true,
        );
    }

    // Override theme
    write_start_tag(
        &mut writer,
        "Override",
        vec![
            ("PartName", "/xl/theme/theme1.xml"),
            (
                "ContentType",
                "application/vnd.openxmlformats-officedocument.theme+xml",
            ),
        ],
        true,
    );

    // Override styles
    write_start_tag(
        &mut writer,
        "Override",
        vec![
            ("PartName", "/xl/styles.xml"),
            (
                "ContentType",
                "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml",
            ),
        ],
        true,
    );

    // Override sharedStrings
    write_start_tag(
        &mut writer,
        "Override",
        vec![
            ("PartName", "/xl/sharedStrings.xml"),
            (
                "ContentType",
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml",
            ),
        ],
        true,
    );

    // Override drawing
    for path_str in writer_mng.has_find("xl/drawings/drawing") {
        write_start_tag(
            &mut writer,
            "Override",
            vec![
                ("PartName", &path_str),
                (
                    "ContentType",
                    "application/vnd.openxmlformats-officedocument.drawing+xml",
                ),
            ],
            true,
        );
    }

    // Override chart
    for path_str in writer_mng.has_find("xl/charts/chart") {
        write_start_tag(
            &mut writer,
            "Override",
            vec![
                ("PartName", &path_str),
                (
                    "ContentType",
                    "application/vnd.openxmlformats-officedocument.drawingml.chart+xml",
                ),
            ],
            true,
        );
    }

    // Override embeddings
    for path_str in writer_mng.has_find("xl/embeddings/oleObject") {
        write_start_tag(
            &mut writer,
            "Override",
            vec![
                ("PartName", &path_str),
                (
                    "ContentType",
                    "application/vnd.openxmlformats-officedocument.oleObject",
                ),
            ],
            true,
        );
    }

    // Override xl/vbaProject.bin
    if spreadsheet.get_has_macros() {
        write_start_tag(
            &mut writer,
            "Override",
            vec![
                ("PartName", "/xl/vbaProject.bin"),
                ("ContentType", "application/vnd.ms-office.vbaProject"),
            ],
            true,
        );
    };

    // Override docProps/core
    write_start_tag(
        &mut writer,
        "Override",
        vec![
            ("PartName", "/docProps/core.xml"),
            (
                "ContentType",
                "application/vnd.openxmlformats-package.core-properties+xml",
            ),
        ],
        true,
    );

    // Override docProps/app
    write_start_tag(
        &mut writer,
        "Override",
        vec![
            ("PartName", "/docProps/app.xml"),
            (
                "ContentType",
                "application/vnd.openxmlformats-officedocument.extended-properties+xml",
            ),
        ],
        true,
    );

    write_end_tag(&mut writer, "Types");
    let _ = make_file_from_writer(
        "[Content_Types].xml",
        writer_mng.get_arv_mut(),
        writer,
        None,
    )
    .unwrap();
}
