//! A hyperlink's `ref` can be a range, as Excel writes for a hyperlink on a
//! merged cell. Reading such a file used to panic.

use std::io::{
    Cursor,
    Read,
    Write,
};

use umya_spreadsheet::{
    Hyperlink,
    Workbook,
    reader,
    writer,
};
use zip::{
    ZipArchive,
    ZipWriter,
    write::SimpleFileOptions,
};

fn write(book: &Workbook) -> Vec<u8> {
    let mut out = Vec::new();
    writer::xlsx::write_writer(book, &mut out).unwrap();
    out
}

/// Rewrite the first worksheet with `edit`.
fn patch_sheet(bytes: &[u8], edit: impl Fn(String) -> String) -> Vec<u8> {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut out = ZipWriter::new(Cursor::new(Vec::new()));
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).unwrap();
        let name = entry.name().unwrap().to_string();
        let mut data = Vec::new();
        entry.read_to_end(&mut data).unwrap();
        if name == "xl/worksheets/sheet1.xml" {
            let before = String::from_utf8(data).unwrap();
            let after = edit(before.clone());
            assert_ne!(before, after, "the patch changed nothing");
            data = after.into_bytes();
        }
        out.start_file(name, SimpleFileOptions::default()).unwrap();
        out.write_all(&data).unwrap();
    }
    out.finish().unwrap().into_inner()
}

fn read(bytes: Vec<u8>) -> Workbook {
    reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap()
}

fn book_with_link_on_merged_cell() -> Workbook {
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    sheet.add_merge_cells("A1:A2");
    let mut hyperlink = Hyperlink::default();
    hyperlink.set_url("Sheet1!C3").set_location(true);
    sheet
        .cell_mut("A1")
        .set_value("link")
        .set_hyperlink(hyperlink);
    book
}

#[test]
fn hyperlink_on_a_range_is_read_on_its_top_left_cell() {
    let bytes = patch_sheet(&write(&book_with_link_on_merged_cell()), |xml| {
        xml.replace(r#"<hyperlink ref="A1""#, r#"<hyperlink ref="A1:A2""#)
    });

    let book = read(bytes);
    let sheet = book.sheet(0).unwrap();
    let hyperlink = sheet.cell("A1").unwrap().hyperlink().unwrap();
    assert_eq!(hyperlink.url(), "Sheet1!C3");
    assert!(hyperlink.location());
    assert!(sheet.cell("A2").and_then(|cell| cell.hyperlink()).is_none());
}

#[test]
fn hyperlink_without_a_cell_reference_is_dropped() {
    let bytes = patch_sheet(&write(&book_with_link_on_merged_cell()), |xml| {
        xml.replace(r#"<hyperlink ref="A1""#, "<hyperlink")
    });

    let book = read(bytes);
    let sheet = book.sheet(0).unwrap();
    let cell = sheet.cell("A1").unwrap();
    assert!(cell.hyperlink().is_none());
    assert_eq!(cell.value(), "link");
}
