//! The calculate cell flag (`ca`) of formulas must survive reading and saving
//! a workbook.
use std::io::{
    Cursor,
    Read,
    Write,
};

use umya_spreadsheet::{
    CellFormula,
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

/// Saves `NOW()` in A1 and `1+1` in A2, then marks A1 with `ca="1"`, the way
/// Excel writes formulas that use volatile functions.
fn book_with_calculate_cell() -> Vec<u8> {
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    sheet.cell_mut("A1").set_formula("NOW()");
    sheet.cell_mut("A2").set_formula("1+1");
    replace_sheet_xml(write(&book), |xml| {
        assert!(xml.contains("<f>NOW()</f>"), "{xml}");
        *xml = xml.replace("<f>NOW()</f>", r#"<f ca="1">NOW()</f>"#);
    })
}

#[test]
fn calculate_cell_is_read() {
    let book = reader::xlsx::read_reader(Cursor::new(book_with_calculate_cell()), true).unwrap();
    let sheet = book.sheet(0).unwrap();
    let a1 = sheet.cell("A1").unwrap().formula_obj().unwrap();
    assert!(a1.calculate_cell());
    let a2 = sheet.cell("A2").unwrap().formula_obj().unwrap();
    assert!(!a2.calculate_cell());
}

#[test]
fn calculate_cell_is_written_back() {
    let book = reader::xlsx::read_reader(Cursor::new(book_with_calculate_cell()), true).unwrap();
    let xml = sheet_xml(&write(&book));
    let a1 = formula_xml(&xml, "A1");
    assert!(a1.contains(r#"ca="1""#), "{a1}");
    assert!(a1.contains("NOW()"), "{a1}");
    let a2 = formula_xml(&xml, "A2");
    assert!(!a2.contains("ca="), "{a2}");

    let book = reader::xlsx::read_reader(Cursor::new(write(&book)), true).unwrap();
    let a1 = book
        .sheet(0)
        .unwrap()
        .cell("A1")
        .unwrap()
        .formula_obj()
        .unwrap();
    assert!(a1.calculate_cell());
}

#[test]
fn calculate_cell_can_be_set() {
    let mut formula = CellFormula::default();
    formula
        .set_text("INDIRECT(\"B1\")")
        .set_calculate_cell(true);
    let mut book = umya_spreadsheet::new_file();
    book.sheet_mut(0)
        .unwrap()
        .cell_mut("A1")
        .cell_value_mut()
        .set_formula_obj(formula);

    let xml = sheet_xml(&write(&book));
    let a1 = formula_xml(&xml, "A1");
    assert!(a1.contains(r#"ca="1""#), "{a1}");
}
