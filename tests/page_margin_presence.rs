//! Explicit zero print margins must survive an XLSX round trip.
use std::io::{
    Cursor,
    Read,
    Write,
};

use umya_spreadsheet::{
    reader,
    writer,
};

#[test]
fn explicit_zero_margins_survive_round_trip() {
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    sheet.cell_mut("A1").set_value("Borderless report");
    sheet
        .page_margins_mut()
        .set_left(0.0)
        .set_right(0.0)
        .set_top(0.0)
        .set_bottom(0.0)
        .set_header(0.0)
        .set_footer(0.0);
    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&book, &mut bytes).unwrap();
    let reopened = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let sheet = reopened.sheet(0).unwrap();
    assert_eq!(sheet.value("A1"), "Borderless report");
    let margins = sheet.page_margins();
    assert_eq!(
        [
            margins.left(),
            margins.right(),
            margins.top(),
            margins.bottom(),
            margins.header(),
            margins.footer()
        ],
        [0.0; 6]
    );
}

#[test]
fn unset_margins_keep_excel_defaults_on_write() {
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .cell_mut("A1")
        .set_value("Default report");
    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&book, &mut bytes).unwrap();
    let reopened = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let margins = reopened.sheet(0).unwrap().page_margins();
    assert_eq!(
        [
            margins.left(),
            margins.right(),
            margins.top(),
            margins.bottom(),
            margins.header(),
            margins.footer()
        ],
        [0.7, 0.7, 0.75, 0.75, 0.3, 0.3]
    );
}

#[test]
fn individually_set_margins_preserve_zero_and_default_only_unset_fields() {
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    sheet.cell_mut("A1").set_value("Mixed margins");
    let margins = sheet.page_margins_mut();
    assert!(!margins.has_left());
    assert!(!margins.has_right());
    assert!(!margins.has_top());
    assert!(!margins.has_bottom());
    assert!(!margins.has_header());
    assert!(!margins.has_footer());
    margins.set_left(0.0).set_top(1.25).set_footer(0.0);
    assert!(margins.has_left());
    assert!(margins.has_top());
    assert!(margins.has_footer());
    assert!(!margins.has_right());
    assert!(!margins.has_bottom());
    assert!(!margins.has_header());
    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&book, &mut bytes).unwrap();
    let reopened = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let margins = reopened.sheet(0).unwrap().page_margins();
    assert_eq!(
        [
            margins.left(),
            margins.right(),
            margins.top(),
            margins.bottom(),
            margins.header(),
            margins.footer()
        ],
        [0.0, 0.7, 1.25, 0.75, 0.3, 0.0]
    );
    assert!(margins.has_left());
    assert!(margins.has_right());
    assert!(margins.has_top());
    assert!(margins.has_bottom());
    assert!(margins.has_header());
    assert!(margins.has_footer());
}

#[test]
fn absent_page_margins_remain_unset_after_reading() {
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .cell_mut("A1")
        .set_value("No print metadata");
    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&book, &mut bytes).unwrap();
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut output = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for index in 0..archive.len() {
        let mut part = archive.by_index(index).unwrap();
        let mut bytes = Vec::new();
        part.read_to_end(&mut bytes).unwrap();
        if part.name() == "xl/worksheets/sheet1.xml" {
            let mut xml = String::from_utf8(bytes).unwrap();
            let start = xml.find("<pageMargins ").unwrap();
            let end = start + xml[start..].find("/>").unwrap() + 2;
            xml.replace_range(start..end, "");
            bytes = xml.into_bytes();
        }
        output
            .start_file(part.name(), zip::write::SimpleFileOptions::default())
            .unwrap();
        output.write_all(&bytes).unwrap();
    }
    let bytes = output.finish().unwrap().into_inner();
    let reopened = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let sheet = reopened.sheet(0).unwrap();
    assert_eq!(sheet.value("A1"), "No print metadata");
    let margins = sheet.page_margins();
    assert!(!margins.has_left());
    assert!(!margins.has_right());
    assert!(!margins.has_top());
    assert!(!margins.has_bottom());
    assert!(!margins.has_header());
    assert!(!margins.has_footer());
}
