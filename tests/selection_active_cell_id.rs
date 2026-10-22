use std::io::{
    Cursor,
    Read,
};

use umya_spreadsheet::{
    Coordinate,
    Selection,
    Workbook,
    reader,
    writer,
};
use zip::ZipArchive;

fn write(book: &Workbook) -> Vec<u8> {
    let mut out = Vec::new();
    writer::xlsx::write_writer(book, &mut out).unwrap();
    out
}

fn selection_xml(bytes: &[u8]) -> String {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("xl/worksheets/sheet1.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    let start = xml.find("<selection").expect("no selection written");
    let end = start + xml[start..].find("/>").unwrap() + 2;
    xml[start..end].to_string()
}

fn set_selection(book: &mut Workbook, active_cell: &str, sqref: &str) {
    let mut coordinate = Coordinate::default();
    coordinate.set_coordinate(active_cell);
    let mut selection = Selection::default();
    selection.set_active_cell(coordinate);
    selection.sequence_of_references_mut().set_sqref(sqref);
    book.sheet_mut(0)
        .unwrap()
        .sheet_views_mut()
        .sheet_view_list_mut()[0]
        .set_selection(selection);
}

fn roundtrip_selection(active_cell: &str, sqref: &str) -> String {
    let mut book = umya_spreadsheet::new_file();
    set_selection(&mut book, active_cell, sqref);
    let book = reader::xlsx::read_reader(Cursor::new(write(&book)), true).unwrap();
    selection_xml(&write(&book))
}

#[test]
fn active_cell_inside_single_range() {
    // e.g. Ctrl+A with the cursor on C4, or Tab inside a selected block
    let xml = roundtrip_selection("C4", "A1:XFD1048576");
    assert!(!xml.contains("activeCellId"), "{xml}");
    assert!(xml.contains(r#"activeCell="C4""#), "{xml}");

    let xml = roundtrip_selection("B2", "A1:C3");
    assert!(!xml.contains("activeCellId"), "{xml}");
}

#[test]
fn active_cell_inside_second_range() {
    let xml = roundtrip_selection("C3", "A1:A3 C1:C5");
    assert!(xml.contains(r#"activeCellId="1""#), "{xml}");

    // A10 is not A1, even though "A10:A12" contains the text "A1"
    let xml = roundtrip_selection("A1", "A10:A12 A1:B2");
    assert!(xml.contains(r#"activeCellId="1""#), "{xml}");
}

#[test]
fn active_cell_at_range_corner() {
    let xml = roundtrip_selection("A1", "A1:B2");
    assert!(!xml.contains("activeCellId"), "{xml}");

    let xml = roundtrip_selection("D4", "A1:B2 D4");
    assert!(xml.contains(r#"activeCellId="1""#), "{xml}");
}
