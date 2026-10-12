//! xlsx has no representation for NaN or infinity, so they must never be
//! written as a number: text that Rust parses as a non-finite float stays
//! text, and non-finite numbers become the `#NUM!` error.
use std::io::{
    Cursor,
    Read,
};

use umya_spreadsheet::{
    CellErrorType,
    CellRawValue,
    reader,
    writer,
};

const NON_FINITE: [&str; 8] = [
    "NaN",
    "nan",
    "-NaN",
    "inf",
    "-inf",
    "Infinity",
    "+infinity",
    "1e400",
];

#[test]
fn non_finite_strings_are_stored_as_text() {
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    for (row, text) in (1..).zip(NON_FINITE) {
        let cell = sheet.cell_mut((1, row));
        cell.set_value(text);
        assert_eq!(cell.data_type(), "s", "{text}");
        assert_eq!(cell.value_number(), None, "{text}");
    }
    sheet.cell_mut("B1").set_value("1.5");
    assert_eq!(sheet.cell("B1").unwrap().data_type(), "n");

    let (bytes, xml) = write_with_sheet_xml(&book);
    for text in NON_FINITE {
        assert!(!xml.contains(&format!("<v>{text}</v>")), "{text}");
    }

    let reopened = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let sheet = reopened.sheet(0).unwrap();
    for (row, text) in (1..).zip(NON_FINITE) {
        assert_eq!(sheet.value((1, row)), text);
    }
    assert_eq!(sheet.cell("B1").unwrap().value_number(), Some(1.5));
}

#[test]
fn non_finite_numbers_are_stored_as_num_error() {
    let values = [f64::NAN, f64::INFINITY, f64::NEG_INFINITY];

    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    for (row, value) in (1..).zip(values) {
        let cell = sheet.cell_mut((1, row));
        cell.set_value_number(value);
        assert_eq!(cell.raw_value(), &CellRawValue::Error(CellErrorType::Num));
        assert_eq!(cell.value(), "#NUM!");

        let cell = sheet.cell_mut((2, row));
        cell.set_formula("1/0").set_formula_result_number(value);
        assert_eq!(cell.formula(), "1/0");
        assert_eq!(cell.raw_value(), &CellRawValue::Error(CellErrorType::Num));
    }
    sheet.cell_mut("C1").set_value_number(1.5);
    assert_eq!(sheet.cell("C1").unwrap().value_number(), Some(1.5));

    let (bytes, xml) = write_with_sheet_xml(&book);
    assert!(xml.contains(r#"<c r="A1" t="e"><v>#NUM!</v></c>"#));
    assert!(!xml.contains("<v>NaN</v>"));
    assert!(!xml.contains("<v>inf</v>"));
    assert!(!xml.contains("<v>-inf</v>"));

    let reopened = reader::xlsx::read_reader(Cursor::new(bytes), true).unwrap();
    let sheet = reopened.sheet(0).unwrap();
    for row in 1..=3 {
        for col in 1..=2 {
            assert_eq!(
                sheet.cell((col, row)).unwrap().raw_value(),
                &CellRawValue::Error(CellErrorType::Num)
            );
        }
    }
    assert_eq!(sheet.cell("C1").unwrap().value_number(), Some(1.5));
}

fn write_with_sheet_xml(book: &umya_spreadsheet::Workbook) -> (Vec<u8>, String) {
    let mut bytes = Vec::new();
    writer::xlsx::write_writer(book, &mut bytes).unwrap();

    let mut xml = String::new();
    zip::ZipArchive::new(Cursor::new(bytes.clone()))
        .unwrap()
        .by_name("xl/worksheets/sheet1.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
    (bytes, xml)
}
