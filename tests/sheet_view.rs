//! The attributes of the sheet view (`sheetView`) must survive reading and
//! saving a workbook.
use std::io::{
    Cursor,
    Read,
    Write,
};

use umya_spreadsheet::{
    SheetView,
    SheetViewValues,
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

fn sheet_view_xml(xml: &str) -> String {
    let start = xml
        .find("<sheetView ")
        .unwrap_or_else(|| panic!("sheetView not written: {xml}"));
    let end = start + xml[start..].find("/>").unwrap() + 2;
    xml[start..end].to_string()
}

fn has_attribute(element: &str, name: &str, value: &str) -> bool {
    element.contains(&format!(r#" {name}="{value}""#))
}

/// Saves a new workbook and replaces the `sheetView` of its first sheet with
/// `sheet_view`.
fn book_with_sheet_view(sheet_view: &str) -> Vec<u8> {
    let book = umya_spreadsheet::new_file();
    replace_sheet_xml(write(&book), |xml| {
        let old = sheet_view_xml(xml);
        *xml = xml.replace(&old, sheet_view);
    })
}

fn first_sheet_view(book: &Workbook) -> &SheetView {
    &book.sheet(0).unwrap().sheets_views().sheet_view_list()[0]
}

const SHEET_VIEW: &str = r#"<sheetView windowProtection="1" showFormulas="1" showGridLines="0" showRowColHeaders="0" showZeros="0" rightToLeft="1" tabSelected="1" showRuler="0" showOutlineSymbols="0" defaultGridColor="0" showWhiteSpace="0" view="pageLayout" topLeftCell="B2" colorId="10" zoomScale="80" zoomScaleNormal="90" zoomScaleSheetLayoutView="70" zoomScalePageLayoutView="60" workbookViewId="0"/>"#;

#[test]
fn sheet_view_attributes_are_read() {
    let bytes = book_with_sheet_view(SHEET_VIEW);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let view = first_sheet_view(&book);
    assert!(view.window_protection());
    assert!(view.show_formulas());
    assert!(!view.show_grid_lines());
    assert!(!view.show_row_col_headers());
    assert!(!view.show_zeros());
    assert!(view.right_to_left());
    assert!(view.tab_selected());
    assert!(!view.show_ruler());
    assert!(!view.show_outline_symbols());
    assert!(!view.default_grid_color());
    assert!(!view.show_white_space());
    assert!(matches!(view.view(), SheetViewValues::PageLayout));
    assert_eq!(view.top_left_cell(), "B2");
    assert_eq!(view.color_id(), 10);
    assert_eq!(view.zoom_scale(), 80);
    assert_eq!(view.zoom_scale_normal(), 90);
    assert_eq!(view.zoom_scale_sheet_layout_view(), 70);
    assert_eq!(view.zoom_scale_page_layout_view(), 60);
}

#[test]
fn missing_attributes_use_defaults() {
    let element = r#"<sheetView workbookViewId="0"/>"#;
    let bytes = book_with_sheet_view(element);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let view = first_sheet_view(&book);
    assert!(!view.window_protection());
    assert!(!view.show_formulas());
    assert!(view.show_row_col_headers());
    assert!(view.show_zeros());
    assert!(!view.right_to_left());
    assert!(view.show_ruler());
    assert!(view.show_outline_symbols());
    assert!(view.default_grid_color());
    assert!(view.show_white_space());
    assert_eq!(view.color_id(), 64);

    let view = sheet_view_xml(&sheet_xml(&write(&book)));
    assert_eq!(view, element);
}

#[test]
fn sheet_view_attributes_are_written_back() {
    let bytes = book_with_sheet_view(SHEET_VIEW);
    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let view = sheet_view_xml(&sheet_xml(&write(&book)));
    for (name, value) in [
        ("windowProtection", "1"),
        ("showFormulas", "1"),
        ("showGridLines", "0"),
        ("showRowColHeaders", "0"),
        ("showZeros", "0"),
        ("rightToLeft", "1"),
        ("tabSelected", "1"),
        ("showRuler", "0"),
        ("showOutlineSymbols", "0"),
        ("defaultGridColor", "0"),
        ("showWhiteSpace", "0"),
        ("view", "pageLayout"),
        ("topLeftCell", "B2"),
        ("colorId", "10"),
        ("zoomScale", "80"),
        ("zoomScaleNormal", "90"),
        ("zoomScaleSheetLayoutView", "70"),
        ("zoomScalePageLayoutView", "60"),
        ("workbookViewId", "0"),
    ] {
        assert!(has_attribute(&view, name, value), "{name}: {view}");
    }

    let book = reader::xlsx::read_reader(Cursor::new(write(&book)), true).unwrap();
    let view = first_sheet_view(&book);
    assert!(view.right_to_left());
    assert!(!view.show_zeros());
    assert_eq!(view.color_id(), 10);
}

#[test]
fn sheet_view_attributes_can_be_set() {
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .sheet_views_mut()
        .sheet_view_list_mut()[0]
        .set_right_to_left(true)
        .set_show_zeros(false)
        .set_show_row_col_headers(false);
    let view = sheet_view_xml(&sheet_xml(&write(&book)));
    assert_eq!(
        view,
        r#"<sheetView showRowColHeaders="0" showZeros="0" rightToLeft="1" workbookViewId="0"/>"#
    );
}

#[test]
fn new_sheet_view_is_unchanged() {
    let view = sheet_view_xml(&sheet_xml(&write(&umya_spreadsheet::new_file())));
    assert_eq!(view, r#"<sheetView workbookViewId="0"/>"#);
}
