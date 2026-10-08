//! Text that Rust parses as a non-finite float must not be written as a
//! number, since xlsx has no representation for NaN or infinity.
use std::io::{
    Cursor,
    Read,
};

use umya_spreadsheet::{
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

    let mut bytes = Vec::new();
    writer::xlsx::write_writer(&book, &mut bytes).unwrap();

    let mut xml = String::new();
    zip::ZipArchive::new(Cursor::new(bytes.clone()))
        .unwrap()
        .by_name("xl/worksheets/sheet1.xml")
        .unwrap()
        .read_to_string(&mut xml)
        .unwrap();
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
