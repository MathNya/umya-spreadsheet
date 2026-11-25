//! The attributes of the print options (`printOptions`) must survive reading
//! and saving a workbook.
use std::io::{
    Cursor,
    Read,
    Write,
};

use umya_spreadsheet::{
    PrintOptions,
    Workbook,
    reader,
    writer,
};

const SHEET_XML: &str = "xl/worksheets/sheet1.xml";

fn write(book: &Workbook) -> Vec<u8> {
    let mut out = Vec::new();
    writer::xlsx::write_writer(book, &mut out).unwrap();
    out
}

fn sheet_xml(bytes: &[u8]) -> String {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name(SHEET_XML)
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    xml
}

fn replace_sheet_xml(bytes: Vec<u8>, edit: impl Fn(&mut String)) -> Vec<u8> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut output = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for index in 0..archive.len() {
        let mut part = archive.by_index(index).unwrap();
        let mut bytes = Vec::new();
        part.read_to_end(&mut bytes).unwrap();
        if part.name().unwrap() == SHEET_XML {
            let mut xml = String::from_utf8(bytes).unwrap();
            edit(&mut xml);
            bytes = xml.into_bytes();
        }
        output
            .start_file(
                part.name().unwrap(),
                zip::write::SimpleFileOptions::default(),
            )
            .unwrap();
        output.write_all(&bytes).unwrap();
    }
    output.finish().unwrap().into_inner()
}

fn print_options_xml(xml: &str) -> Option<String> {
    let start = xml.find("<printOptions")?;
    let end = start + xml[start..].find("/>").unwrap() + 2;
    Some(xml[start..end].to_string())
}

/// Saves a new workbook and puts `print_options` in its first sheet, right
/// before `pageMargins`.
fn book_with_print_options(print_options: &str) -> Vec<u8> {
    let book = umya_spreadsheet::new_file();
    replace_sheet_xml(write(&book), |xml| {
        assert!(print_options_xml(xml).is_none(), "{xml}");
        let start = xml.find("<pageMargins ").unwrap();
        xml.insert_str(start, print_options);
    })
}

fn first_print_options(book: &Workbook) -> &PrintOptions {
    book.sheet(0).unwrap().print_options()
}

const PRINT_OPTIONS: &str = r#"<printOptions horizontalCentered="1" verticalCentered="1" headings="1" gridLines="1" gridLinesSet="0"/>"#;

#[test]
fn print_options_attributes_are_read() {
    let bytes = book_with_print_options(PRINT_OPTIONS);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let options = first_print_options(&book);
    assert!(options.horizontal_centered());
    assert!(options.vertical_centered());
    assert!(options.headings());
    assert!(options.grid_lines());
    assert!(!options.grid_lines_set());
}

#[test]
fn missing_attributes_use_defaults() {
    let element = r#"<printOptions horizontalCentered="1"/>"#;
    let bytes = book_with_print_options(element);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let options = first_print_options(&book);
    assert!(!options.headings());
    assert!(!options.grid_lines());
    assert!(options.grid_lines_set());

    let options = print_options_xml(&sheet_xml(&write(&book)));
    assert_eq!(options.as_deref(), Some(element));
}

#[test]
fn print_options_attributes_are_written_back() {
    let bytes = book_with_print_options(PRINT_OPTIONS);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let options = print_options_xml(&sheet_xml(&write(&book)));
    assert_eq!(options.as_deref(), Some(PRINT_OPTIONS));

    let book = reader::xlsx::read_reader(Cursor::new(write(&book)), true).unwrap();
    let options = first_print_options(&book);
    assert!(options.headings());
    assert!(options.grid_lines());
    assert!(!options.grid_lines_set());
}

#[test]
fn grid_lines_only_are_written() {
    let element = r#"<printOptions gridLines="1"/>"#;
    let bytes = book_with_print_options(element);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let options = print_options_xml(&sheet_xml(&write(&book)));
    assert_eq!(options.as_deref(), Some(element));
}

#[test]
fn print_options_attributes_can_be_set() {
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .print_options_mut()
        .set_grid_lines(true)
        .set_headings(true);
    let options = print_options_xml(&sheet_xml(&write(&book)));
    assert_eq!(
        options.as_deref(),
        Some(r#"<printOptions headings="1" gridLines="1"/>"#)
    );
}

#[test]
fn new_workbook_has_no_print_options() {
    let options = print_options_xml(&sheet_xml(&write(&umya_spreadsheet::new_file())));
    assert_eq!(options, None);
}
