//! The attributes of the page setup (`pageSetup`) must survive reading and
//! saving a workbook.
use std::io::{
    Cursor,
    Read,
    Write,
};

use umya_spreadsheet::{
    CellCommentsValues,
    PageOrderValues,
    PageSetup,
    PrintErrorValues,
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

fn page_setup_xml(xml: &str) -> Option<String> {
    let start = xml.find("<pageSetup ")?;
    let end = start + xml[start..].find("/>").unwrap() + 2;
    Some(xml[start..end].to_string())
}

/// Saves a new workbook and puts `page_setup` in its first sheet, right
/// after `pageMargins`.
fn book_with_page_setup(page_setup: &str) -> Vec<u8> {
    let book = umya_spreadsheet::new_file();
    replace_sheet_xml(write(&book), |xml| {
        assert!(page_setup_xml(xml).is_none(), "{xml}");
        let start = xml.find("<pageMargins ").unwrap();
        let end = start + xml[start..].find("/>").unwrap() + 2;
        xml.insert_str(end, page_setup);
    })
}

fn first_page_setup(book: &Workbook) -> &PageSetup {
    book.sheet(0).unwrap().page_setup()
}

const PAGE_SETUP: &str = r#"<pageSetup paperSize="9" firstPageNumber="3" orientation="landscape" pageOrder="overThenDown" blackAndWhite="1" draft="1" cellComments="atEnd" useFirstPageNumber="1" errors="dash" copies="2"/>"#;

fn assert_page_setup(page_setup: &PageSetup) {
    assert_eq!(page_setup.paper_size(), 9);
    assert_eq!(page_setup.first_page_number(), 3);
    assert!(page_setup.use_first_page_number());
    assert!(matches!(
        page_setup.page_order(),
        PageOrderValues::OverThenDown
    ));
    assert!(page_setup.black_and_white());
    assert!(page_setup.draft());
    assert!(matches!(
        page_setup.cell_comments(),
        CellCommentsValues::AtEnd
    ));
    assert!(matches!(page_setup.errors(), PrintErrorValues::Dash));
    assert_eq!(page_setup.copies(), 2);
}

#[test]
fn page_setup_attributes_are_read() {
    let bytes = book_with_page_setup(PAGE_SETUP);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    assert_page_setup(first_page_setup(&book));
}

#[test]
fn page_setup_attributes_are_written_back() {
    let bytes = book_with_page_setup(PAGE_SETUP);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let page_setup = page_setup_xml(&sheet_xml(&write(&book)));
    assert_eq!(page_setup.as_deref(), Some(PAGE_SETUP));

    let book = reader::xlsx::read_reader(Cursor::new(write(&book)), true).unwrap();
    assert_page_setup(first_page_setup(&book));
}

#[test]
fn missing_attributes_use_defaults() {
    let element = r#"<pageSetup paperSize="9"/>"#;
    let bytes = book_with_page_setup(element);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let page_setup = first_page_setup(&book);
    assert_eq!(page_setup.first_page_number(), 1);
    assert!(!page_setup.use_first_page_number());
    assert!(matches!(
        page_setup.page_order(),
        PageOrderValues::DownThenOver
    ));
    assert!(!page_setup.black_and_white());
    assert!(!page_setup.draft());
    assert!(matches!(
        page_setup.cell_comments(),
        CellCommentsValues::None
    ));
    assert!(matches!(page_setup.errors(), PrintErrorValues::Displayed));
    assert_eq!(page_setup.copies(), 1);

    let page_setup = page_setup_xml(&sheet_xml(&write(&book)));
    assert_eq!(page_setup.as_deref(), Some(element));
}

#[test]
fn black_and_white_only_is_written() {
    let element = r#"<pageSetup blackAndWhite="1"/>"#;
    let bytes = book_with_page_setup(element);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let page_setup = page_setup_xml(&sheet_xml(&write(&book)));
    assert_eq!(page_setup.as_deref(), Some(element));
}

#[test]
fn page_setup_attributes_can_be_set() {
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .page_setup_mut()
        .set_first_page_number(5)
        .set_use_first_page_number(true)
        .set_errors(PrintErrorValues::NA);
    let page_setup = page_setup_xml(&sheet_xml(&write(&book)));
    assert_eq!(
        page_setup.as_deref(),
        Some(r#"<pageSetup firstPageNumber="5" useFirstPageNumber="1" errors="NA"/>"#)
    );
}

#[test]
fn new_workbook_has_no_page_setup() {
    let page_setup = page_setup_xml(&sheet_xml(&write(&umya_spreadsheet::new_file())));
    assert_eq!(page_setup, None);
}
