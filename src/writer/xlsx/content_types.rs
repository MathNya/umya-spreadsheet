use quick_xml::events::{Event, BytesDecl};
use quick_xml::Writer;
use std::io::Cursor;
use tempdir::TempDir;

use ::structs::Spreadsheet;
use super::driver::*;
use super::XlsxError;

pub(crate) fn write(spreadsheet: &Spreadsheet, dir: &TempDir, file_name: &str) -> Result<(), XlsxError> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    // XML header
    let _ = writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"UTF-8"), Some(b"yes"))));
    write_new_line(&mut writer);

    // Types
    write_start_tag(&mut writer, "Types", vec![
        ("xmlns", "http://schemas.openxmlformats.org/package/2006/content-types"),
    ], false);

    // Default bin
    //write_start_tag(&mut writer, "Default", vec![
    //    ("Extension", "bin"),
    //    ("ContentType", "application/vnd.openxmlformats-officedocument.spreadsheetml.printerSettings"),
    //], true);

    // Default rels
    write_start_tag(&mut writer, "Default", vec![
        ("Extension", "rels"),
        ("ContentType", "application/vnd.openxmlformats-package.relationships+xml"),
    ], true);

    // Default xml
    write_start_tag(&mut writer, "Default", vec![
        ("Extension", "xml"),
        ("ContentType", "application/xml"),
    ], true);

    // Default vml
    if spreadsheet.has_comment() {
        write_start_tag(&mut writer, "Default", vec![
            ("Extension", "vml"),
            ("ContentType", "application/vnd.openxmlformats-officedocument.vmlDrawing"),
        ], true);
    }

    // Default png
    let mut writed = false;
    for work_sheet in spreadsheet.get_sheet_collection() {
        for picture in work_sheet.get_worksheet_drawing().get_picture_collection() {
            if picture.get_blip_fill().get_blip().is_png() {
                write_start_tag(&mut writer, "Default", vec![
                    ("Extension", "png"),
                    ("ContentType", "image/png"),
                ], true);
                writed = true;
                break;
            }
        }
        if writed {break};
    }

    // Default jpg
    let mut writed = false;
    for work_sheet in spreadsheet.get_sheet_collection() {
        for picture in work_sheet.get_worksheet_drawing().get_picture_collection() {
            if picture.get_blip_fill().get_blip().is_jpg() {
                write_start_tag(&mut writer, "Default", vec![
                    ("Extension", "jpg"),
                    ("ContentType", "image/jpeg"),
                ], true);
                writed = true;
                break;
            }
        }
        if writed {break};
    }

    // Default jpeg
    let mut writed = false;
    for work_sheet in spreadsheet.get_sheet_collection() {
        for picture in work_sheet.get_worksheet_drawing().get_picture_collection() {
            if picture.get_blip_fill().get_blip().is_jpeg() {
                write_start_tag(&mut writer, "Default", vec![
                    ("Extension", "jpeg"),
                    ("ContentType", "image/jpeg"),
                ], true);
                writed = true;
                break;
            }
        }
        if writed {break};
    }

    // Default tiff
    let mut writed = false;
    for work_sheet in spreadsheet.get_sheet_collection() {
        for picture in work_sheet.get_worksheet_drawing().get_picture_collection() {
            if picture.get_blip_fill().get_blip().is_tiff() {
                write_start_tag(&mut writer, "Default", vec![
                    ("Extension", "tiff"),
                    ("ContentType", "image/tiff"),
                ], true);
                writed = true;
                break;
            }
        }
        if writed {break};
    }

    // Override workbook
    let content_type = match spreadsheet.get_has_macros() {
        &true => "application/vnd.ms-excel.sheet.macroEnabled.main+xml",
        &false => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"
    };
    write_start_tag(&mut writer, "Override", vec![
        ("PartName", "/xl/workbook.xml"),
        ("ContentType", content_type),
    ], true);

    // Override sheet
    for i in 0..spreadsheet.get_sheet_count() {
        write_start_tag(&mut writer, "Override", vec![
            ("PartName", format!("/xl/worksheets/sheet{}.xml", (i+1).to_string().as_str()).as_str()),
            ("ContentType", "application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"),
        ], true);
    }

    // Override comments
    for i in 0..spreadsheet.get_sheet_count() {
        let worksheet = &spreadsheet.get_sheet_collection()[i];
        if worksheet.get_comments().len() > 0 {
            write_start_tag(&mut writer, "Override", vec![
                ("PartName", format!("/xl/comments{}.xml", (i+1).to_string().as_str()).as_str()),
                ("ContentType", "application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml"),
            ], true);
        }
    }

    // Override theme
    write_start_tag(&mut writer, "Override", vec![
        ("PartName", "/xl/theme/theme1.xml"),
        ("ContentType", "application/vnd.openxmlformats-officedocument.theme+xml"),
    ], true);

    // Override styles
    write_start_tag(&mut writer, "Override", vec![
        ("PartName", "/xl/styles.xml"),
        ("ContentType", "application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"),
    ], true);

    // Override sharedStrings
    write_start_tag(&mut writer, "Override", vec![
        ("PartName", "/xl/sharedStrings.xml"),
        ("ContentType", "application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml"),
    ], true);

    let mut drawing_id = 1;
    let mut chart_id = 1;
    for i in 0..spreadsheet.get_sheet_count() {
        let worksheet = &spreadsheet.get_sheet_collection()[i];
        if worksheet.get_worksheet_drawing().has_drawing_object() {
            // Override drawing
            write_start_tag(&mut writer, "Override", vec![
                ("PartName", format!("/xl/drawings/drawing{}.xml", drawing_id.to_string().as_str()).as_str()),
                ("ContentType", "application/vnd.openxmlformats-officedocument.drawing+xml"),
            ], true);
            drawing_id += 1;
        }
        for _ in worksheet.get_worksheet_drawing().get_graphic_frame_collection() {
            // Override chart
            write_start_tag(&mut writer, "Override", vec![
                ("PartName", format!("/xl/charts/chart{}.xml", chart_id.to_string().as_str()).as_str()),
                ("ContentType", "application/vnd.openxmlformats-officedocument.drawingml.chart+xml"),
            ], true);
            chart_id += 1;
        }
    }

    // Override xl/vbaProject.bin
    match spreadsheet.get_has_macros() {
        &true => {
            write_start_tag(&mut writer, "Override", vec![
                ("PartName", "/xl/vbaProject.bin"),
                ("ContentType", "application/vnd.ms-office.vbaProject"),
            ], true);
        },
        &false => {}
    };

    // Override docProps/core
    write_start_tag(&mut writer, "Override", vec![
        ("PartName", "/docProps/core.xml"),
        ("ContentType", "application/vnd.openxmlformats-package.core-properties+xml"),
    ], true);

    // Override docProps/app
    write_start_tag(&mut writer, "Override", vec![
        ("PartName", "/docProps/app.xml"),
        ("ContentType", "application/vnd.openxmlformats-officedocument.extended-properties+xml"),
    ], true);

    write_end_tag(&mut writer, "Types");
    let _ = make_file_from_writer(format!("{}",file_name).as_str(), dir, writer, None).unwrap();
    Ok(())
}