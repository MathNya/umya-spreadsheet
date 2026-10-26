//! What-if data table formulas (`t="dataTable"`) must keep their range and
//! input cells when a workbook is read and saved again.
use std::io::{
    Cursor,
    Read,
    Write,
};

use umya_spreadsheet::{
    CellFormulaValues,
    Workbook,
    reader,
    writer,
};

fn write(book: &Workbook) -> Vec<u8> {
    let mut out = Vec::new();
    writer::xlsx::write_writer(book, &mut out).unwrap();
    out
}

fn sheet_xml(bytes: &[u8]) -> String {
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
    let mut xml = String::new();
    archive
        .by_name("xl/worksheets/sheet1.xml")
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
        if part.name().unwrap() == "xl/worksheets/sheet1.xml" {
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

fn formula_xml(xml: &str, cell: &str) -> String {
    let start = xml
        .find(&format!(r#"<c r="{cell}""#))
        .unwrap_or_else(|| panic!("{cell} not written: {xml}"));
    let start = start + xml[start..].find("<f").unwrap();
    let end = start + xml[start..].find("</f>").unwrap() + 4;
    xml[start..end].to_string()
}

/// Saves a sheet with a number in `cell`, then swaps that cell for the given
/// `<f>` element, the way Excel writes the top-left cell of a data table.
fn book_with_formula(cell: &str, formula: &str) -> Vec<u8> {
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .cell_mut(cell)
        .set_value_number(2);
    let plain = format!(r#"<c r="{cell}"><v>2</v></c>"#);
    let with_formula = format!(r#"<c r="{cell}">{formula}<v>2</v></c>"#);
    replace_sheet_xml(write(&book), |xml| {
        assert!(xml.contains(&plain), "{xml}");
        *xml = xml.replace(&plain, &with_formula);
    })
}

#[test]
fn one_variable_data_table_keeps_its_range() {
    let bytes = book_with_formula(
        "B3",
        r#"<f t="dataTable" ref="B3:B5" dt2D="0" dtr="0" r1="A1"/>"#,
    );

    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let formula = book
        .sheet(0)
        .unwrap()
        .cell("B3")
        .unwrap()
        .formula_obj()
        .unwrap();
    assert_eq!(formula.formula_type(), &CellFormulaValues::DataTable);
    assert_eq!(formula.reference(), "B3:B5");
    assert_eq!(formula.r1(), "A1");

    let xml = sheet_xml(&write(&book));
    let f = formula_xml(&xml, "B3");
    assert!(f.contains(r#"t="dataTable""#), "{f}");
    assert!(f.contains(r#"ref="B3:B5""#), "{f}");
    assert!(f.contains(r#"dt2D="0""#), "{f}");
    assert!(f.contains(r#"dtr="0""#), "{f}");
    assert!(f.contains(r#"r1="A1""#), "{f}");

    let book = reader::xlsx::read_reader(Cursor::new(write(&book)), true).unwrap();
    let formula = book
        .sheet(0)
        .unwrap()
        .cell("B3")
        .unwrap()
        .formula_obj()
        .unwrap();
    assert_eq!(formula.reference(), "B3:B5");
}

#[test]
fn two_variable_data_table_keeps_its_range() {
    let bytes = book_with_formula(
        "C3",
        r#"<f t="dataTable" ref="C3:E5" dt2D="1" dtr="1" r1="A1" r2="A2"/>"#,
    );

    let book = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let xml = sheet_xml(&write(&book));
    let f = formula_xml(&xml, "C3");
    assert!(f.contains(r#"t="dataTable""#), "{f}");
    assert!(f.contains(r#"ref="C3:E5""#), "{f}");
    assert!(f.contains(r#"dt2D="1""#), "{f}");
    assert!(f.contains(r#"dtr="1""#), "{f}");
    assert!(f.contains(r#"r1="A1""#), "{f}");
    assert!(f.contains(r#"r2="A2""#), "{f}");
}
