//! Whole-column and whole-row references: removing a row or column used to panic on
//! them, and neither inserts nor removals moved them.

use umya_spreadsheet::new_file;

fn book_with(formula: &str) -> umya_spreadsheet::Workbook {
    let mut book = new_file();
    let sheet = book.sheet_mut(0).unwrap();
    for row in 1..=4 {
        sheet.cell_mut((1, row)).set_value_number(row);
    }
    sheet.cell_mut("F1").set_formula(formula);
    book
}

fn formula(book: &umya_spreadsheet::Workbook, cell: &str) -> String {
    book.sheet(0).unwrap().cell(cell).unwrap().formula().to_string()
}

#[test]
fn remove_row_keeps_whole_column_reference() {
    let mut book = book_with("SUM(A:A)");
    book.remove_row("Sheet1", 2, 1).unwrap();
    assert_eq!(formula(&book, "F1"), "SUM(A:A)");
}

#[test]
fn column_edits_move_whole_column_references() {
    let mut book = book_with("SUM(C:C)");
    book.remove_column("Sheet1", "B", 1).unwrap();
    assert_eq!(formula(&book, "E1"), "SUM(B:B)");

    let mut book = book_with("SUM(C:C)");
    book.insert_new_column("Sheet1", "B", 1).unwrap();
    assert_eq!(formula(&book, "G1"), "SUM(D:D)");
}

#[test]
fn row_edits_move_whole_row_references() {
    let mut book = book_with("SUM(3:3)");
    book.remove_row("Sheet1", 2, 1).unwrap();
    assert_eq!(formula(&book, "F1"), "SUM(2:2)");
}
