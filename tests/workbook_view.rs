//! The attributes of the workbook view (`workbookView`) must survive reading
//! and saving a workbook.
use std::io::{
    Cursor,
    Read,
    Write,
};

use umya_spreadsheet::{
    VisibilityValues,
    Workbook,
    reader,
    writer,
};

fn write(book: &Workbook) -> Vec<u8> {
    let mut out = Vec::new();
    writer::xlsx::write_writer(book, &mut out).unwrap();
    out
}

fn workbook_xml(bytes: &[u8]) -> String {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("xl/workbook.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    xml
}

fn replace_workbook_xml(bytes: Vec<u8>, edit: impl Fn(&mut String)) -> Vec<u8> {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut output = zip::ZipWriter::new(Cursor::new(Vec::new()));
    for index in 0..archive.len() {
        let mut part = archive.by_index(index).unwrap();
        let mut bytes = Vec::new();
        part.read_to_end(&mut bytes).unwrap();
        if part.name().unwrap() == "xl/workbook.xml" {
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

fn workbook_view_xml(xml: &str) -> String {
    let start = xml
        .find("<workbookView")
        .unwrap_or_else(|| panic!("workbookView not written: {xml}"));
    let end = start + xml[start..].find("/>").unwrap() + 2;
    xml[start..end].to_string()
}

fn has_attribute(element: &str, name: &str, value: &str) -> bool {
    element.contains(&format!(r#" {name}="{value}""#))
}

/// Saves a workbook with three sheets and replaces its `workbookView` with
/// `workbook_view`.
fn book_with_workbook_view(workbook_view: &str) -> Vec<u8> {
    let mut book = umya_spreadsheet::new_file();
    book.new_sheet("Sheet2").unwrap();
    book.new_sheet("Sheet3").unwrap();
    replace_workbook_xml(write(&book), |xml| {
        let old = workbook_view_xml(xml);
        *xml = xml.replace(&old, workbook_view);
    })
}

const WORKBOOK_VIEW: &str = r#"<workbookView visibility="hidden" minimized="1" showHorizontalScroll="0" showVerticalScroll="0" showSheetTabs="0" xWindow="100" yWindow="200" windowWidth="20000" windowHeight="10000" tabRatio="750" firstSheet="1" activeTab="2" autoFilterDateGrouping="0"/>"#;

#[test]
fn workbook_view_attributes_are_read() {
    let bytes = book_with_workbook_view(WORKBOOK_VIEW);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let view = book.workbook_view();
    assert_eq!(view.visibility(), &VisibilityValues::Hidden);
    assert!(view.minimized());
    assert!(!view.show_horizontal_scroll());
    assert!(!view.show_vertical_scroll());
    assert!(!view.show_sheet_tabs());
    assert_eq!(view.x_window(), 100);
    assert_eq!(view.y_window(), 200);
    assert_eq!(view.window_width(), 20000);
    assert_eq!(view.window_height(), 10000);
    assert_eq!(view.tab_ratio(), 750);
    assert_eq!(view.first_sheet(), 1);
    assert_eq!(view.active_tab(), 2);
    assert!(!view.auto_filter_date_grouping());
}

#[test]
fn missing_attributes_use_defaults() {
    let element =
        r#"<workbookView xWindow="100" yWindow="200" windowWidth="20000" windowHeight="10000"/>"#;
    let bytes = book_with_workbook_view(element);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let view = book.workbook_view();
    assert_eq!(view.visibility(), &VisibilityValues::Visible);
    assert!(!view.minimized());
    assert!(view.show_horizontal_scroll());
    assert!(view.show_vertical_scroll());
    assert!(view.show_sheet_tabs());
    assert_eq!(view.first_sheet(), 0);
    assert!(view.auto_filter_date_grouping());

    let view = workbook_view_xml(&workbook_xml(&write(&book)));
    assert_eq!(view, element);
}

#[test]
fn workbook_view_attributes_are_written_back() {
    let bytes = book_with_workbook_view(WORKBOOK_VIEW);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let view = workbook_view_xml(&workbook_xml(&write(&book)));
    for (name, value) in [
        ("visibility", "hidden"),
        ("minimized", "1"),
        ("showHorizontalScroll", "0"),
        ("showVerticalScroll", "0"),
        ("showSheetTabs", "0"),
        ("xWindow", "100"),
        ("yWindow", "200"),
        ("windowWidth", "20000"),
        ("windowHeight", "10000"),
        ("tabRatio", "750"),
        ("firstSheet", "1"),
        ("activeTab", "2"),
        ("autoFilterDateGrouping", "0"),
    ] {
        assert!(has_attribute(&view, name, value), "{name}: {view}");
    }

    let book = reader::xlsx::read_reader(Cursor::new(write(&book)), true).unwrap();
    assert!(!book.workbook_view().show_sheet_tabs());
    assert_eq!(book.workbook_view().first_sheet(), 1);
}

#[test]
fn workbook_view_attributes_can_be_set() {
    let mut book = umya_spreadsheet::new_file();
    book.new_sheet("Sheet2").unwrap();
    book.workbook_view_mut()
        .set_show_sheet_tabs(false)
        .set_first_sheet(1);
    let view = workbook_view_xml(&workbook_xml(&write(&book)));
    assert_eq!(
        view,
        r#"<workbookView activeTab="0" xWindow="240" yWindow="105" windowWidth="14805" windowHeight="8010" showSheetTabs="0" firstSheet="1"/>"#
    );
}

#[test]
fn new_workbook_view_is_unchanged() {
    let view = workbook_view_xml(&workbook_xml(&write(&umya_spreadsheet::new_file())));
    assert_eq!(
        view,
        r#"<workbookView activeTab="0" xWindow="240" yWindow="105" windowWidth="14805" windowHeight="8010"/>"#
    );
}
