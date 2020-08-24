extern crate umya_spreadsheet;

#[test]
fn read_and_wite() {
    // reader
    let path = std::path::Path::new("C:/spread_test_data/aaa.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    let _ = book.get_sheet_mut(0).get_cell_mut("A1").set_value("TEST1");
    let a1_value = book.get_sheet(0).get_cell("A1").unwrap().get_value();
    assert_eq!("TEST1", a1_value);

    // writer
    let path = std::path::Path::new("C:/spread_test_data/bbb.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn new_and_wite() {
    // new file
    let mut book = umya_spreadsheet::new_file();

    let _ = book.get_sheet_mut(0).get_cell_mut("A1").set_value("TEST1");
    let a1_value = book.get_sheet(0).get_cell("A1").unwrap().get_value();
    assert_eq!("TEST1", a1_value);

    // writer
    let path = std::path::Path::new("C:/spread_test_data/eee.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}