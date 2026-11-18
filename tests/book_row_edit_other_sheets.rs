//! Inserting or removing rows through the workbook used to move the cells of every sheet.

use umya_spreadsheet::new_file;

fn book() -> umya_spreadsheet::Workbook {
    let mut book = new_file();
    book.sheet_mut(0).unwrap().set_name("Budget");
    book.new_sheet("Data").unwrap();
    for (sheet, prefix) in [("Budget", "b"), ("Data", "d")] {
        let sheet = book.sheet_by_name_mut(sheet).unwrap();
        for row in 1..=4 {
            sheet.cell_mut((1, row)).set_value(format!("{prefix}{row}"));
        }
    }
    book.sheet_by_name_mut("Data").unwrap().cell_mut("B1").set_formula("Budget!A4");
    book
}

#[test]
fn remove_row_leaves_other_sheets_in_place() {
    let mut book = book();
    book.remove_row("Budget", 2, 1).unwrap();

    let data = book.sheet_by_name("Data").unwrap();
    assert_eq!(data.value("A2"), "d2");
    assert_eq!(data.cell("B1").unwrap().formula(), "Budget!A3");
    assert_eq!(book.sheet_by_name("Budget").unwrap().value("A2"), "b3");
}

#[test]
fn insert_new_row_leaves_other_sheets_in_place() {
    let mut book = book();
    book.insert_new_row("Budget", 2, 1).unwrap();

    let data = book.sheet_by_name("Data").unwrap();
    assert_eq!(data.value("A2"), "d2");
    assert_eq!(data.cell("B1").unwrap().formula(), "Budget!A5");
    assert_eq!(book.sheet_by_name("Budget").unwrap().value("A3"), "b2");
}

#[test]
fn sheet_from_other_sheet_only_updates_references() {
    let mut book = book();
    let data = book.sheet_by_name_mut("Data").unwrap();
    data.insert_new_row_from_other_sheet("Budget", 2, 1);
    assert_eq!(data.value("A2"), "d2");
    assert_eq!(data.cell("B1").unwrap().formula(), "Budget!A5");

    data.remove_column_from_other_sheet("Budget", "B", 1);
    assert_eq!(data.cell("B1").unwrap().formula(), "Budget!A5");
}
