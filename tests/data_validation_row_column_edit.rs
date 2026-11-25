//! Data validations move with their cells when rows or columns are inserted or
//! removed.

use umya_spreadsheet::{
    DataValidation,
    DataValidations,
    new_file,
};

fn book_with_validation(sqref: &str) -> umya_spreadsheet::Workbook {
    let mut book = new_file();
    let mut validation = DataValidation::default();
    validation.sequence_of_references_mut().set_sqref(sqref);
    let mut validations = DataValidations::default();
    validations.add_data_validation_list(validation);
    book.sheet_mut(0).unwrap().set_data_validations(validations);
    book
}

fn sqref(book: &umya_spreadsheet::Workbook) -> String {
    book.sheet(0)
        .unwrap()
        .data_validations()
        .unwrap()
        .data_validation_list()[0]
        .sequence_of_references()
        .get_sqref()
}

#[test]
fn row_edits_move_data_validations() {
    let mut book = book_with_validation("C4:C8");
    book.insert_new_row("Sheet1", 2, 1).unwrap();
    assert_eq!(sqref(&book), "C5:C9");

    let mut book = book_with_validation("C4:C8");
    book.remove_row("Sheet1", 2, 1).unwrap();
    assert_eq!(sqref(&book), "C3:C7");
}

#[test]
fn column_edits_move_data_validations() {
    let mut book = book_with_validation("C4:C8");
    book.insert_new_column("Sheet1", "A", 1).unwrap();
    assert_eq!(sqref(&book), "D4:D8");
}
