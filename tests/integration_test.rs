extern crate umya_spreadsheet;

#[test]
fn read_and_wite() {
    // reader
    let path = std::path::Path::new("C:/spread_test_data/aaa.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    let _ = book.get_sheet_mut(0).get_cell_mut("A1").set_value("TEST1");
    let a1_value = book.get_sheet(0).unwrap().get_cell("A1").unwrap().get_value();
    assert_eq!("TEST1", a1_value);

    // writer
    let path = std::path::Path::new("C:/spread_test_data/bbb.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn new_and_wite() {
    // new file
    let mut book = umya_spreadsheet::new_file();

    // new worksheet
    let _ = book.new_sheet("Sheet2");

    // change value
    let _ = book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_mut("A1").set_value("TEST1");
    let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_cell("A1").unwrap().get_value();
    assert_eq!("TEST1", a1_value);

    // add bottom border
    let _ = book.get_sheet_by_name_mut("Sheet2").unwrap()
    .get_style_mut("A1")
    .get_borders_mut()
    .get_bottom_mut()
    .set_border_style(umya_spreadsheet::structs::border::Border::BORDER_MEDIUM);

    // writer
    let path = std::path::Path::new("C:/spread_test_data/eee.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn duplicate_sheet() {
    let mut book = umya_spreadsheet::new_file();
    let _ = book.new_sheet("Sheet2");
    match book.new_sheet("Sheet2") {
        Ok(_) => panic!("getting new sheet.."),
        Err(_) => {}
    }
}