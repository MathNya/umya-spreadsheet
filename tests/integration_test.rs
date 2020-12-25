extern crate umya_spreadsheet;

#[test]
fn read_and_wite() {
    // reader
    let path = std::path::Path::new("C:/spread_test_data/aaa.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    let _ = book.get_sheet_mut(0).get_cell_mut("A1").set_value("TEST1");
    let a1_value = book.get_sheet(0).unwrap().get_value("A1");
    assert_eq!("TEST1", a1_value);

    dbg!(book.get_sheet(0).unwrap().get_formatted_value_by_column_and_row(2, 20));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B20"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B21"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B22"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B23"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B24"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B25"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B26"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B27"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B28"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B29"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B30"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B31"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B32"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B33"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B34"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B35"));
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("B36"));

    let _ = book.get_sheet_mut(1).get_cell_mut("C20")
    .set_value("49046881.119999997").unwrap();
    let _ = book.get_sheet_mut(1).get_style_mut("C20")
    .get_number_format_mut()
    .set_format_code("#,##0.00");
    dbg!(book.get_sheet(1).unwrap().get_formatted_value("C20"));

    let value = book.get_sheet(0).unwrap().get_value("B30");
    let _ = book.get_sheet_mut(0).get_cell_mut("C30").set_value(value);
    let _ = book.get_sheet_mut(0).get_style_mut("C30")
    .get_number_format_mut()
    .set_format_code(umya_spreadsheet::NumberFormat::FORMAT_DATE_XLSX17);
    dbg!(book.get_sheet(0).unwrap().get_formatted_value("C30"));

    // writer
    let path = std::path::Path::new("C:/spread_test_data/bbb.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn read_and_wite_by_empty() {
    // reader
    let path = std::path::Path::new("C:/spread_test_data/aaa_empty.xlsx");
    let book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    // writer
    let path = std::path::Path::new("C:/spread_test_data/bbb_empty.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn read_and_wite_xlsm() {
    // reader
    let path = std::path::Path::new("C:/spread_test_data/aaa.xlsm");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    //let _ = book.get_sheet_mut(0).get_cell_mut("A1").set_value("TEST1");
    let _ = book.get_sheet_mut(0).get_cell_by_column_and_row_mut(1, 1).set_value("TEST1");
    let a1_value = book.get_sheet(0).unwrap().get_cell_by_column_and_row(1, 1).unwrap().get_value();
    assert_eq!("TEST1", a1_value);

    // writer
    let path = std::path::Path::new("C:/spread_test_data/bbb.xlsm");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn new_and_wite() {
    // new file.
    let mut book = umya_spreadsheet::new_file();

    // new worksheet.
    let _ = book.new_sheet("Sheet2");

    // change value.
    let _ = book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_mut("A1").set_value("TEST1").unwrap();
    let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_cell("A1").unwrap().get_value();
    assert_eq!("TEST1", a1_value);

    let _ = book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_by_column_and_row_mut(2, 2).set_value_and_data_type("1", umya_spreadsheet::Cell::TYPE_NUMERIC).unwrap();
    let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_cell_by_column_and_row(2, 2).unwrap().get_value();
    assert_eq!("1", a1_value);

    let _ = book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_by_column_and_row_mut(3, 3).set_value_and_data_type("true", umya_spreadsheet::Cell::TYPE_BOOL).unwrap();
    let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_cell_by_column_and_row(3, 3).unwrap().get_value();
    assert_eq!("true", a1_value);

    let _ = book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_by_column_and_row_mut(3, 4).set_value("true").unwrap();
    let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_cell_by_column_and_row(3, 4).unwrap().get_value();
    assert_eq!("true", a1_value);

    let _ = book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_by_column_and_row_mut(3, 5).set_value("false").unwrap();
    let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_cell_by_column_and_row(3, 5).unwrap().get_value();
    assert_eq!("false", a1_value);

    let _ = book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_by_column_and_row_mut(3, 6).set_value("100").unwrap();
    let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_cell_by_column_and_row(3, 6).unwrap().get_value();
    assert_eq!("100", a1_value);

    // add bottom border.
    let _ = book.get_sheet_by_name_mut("Sheet2").unwrap().get_style_mut("A1")
    .get_borders_mut()
    .get_bottom_mut()
    .set_border_style(umya_spreadsheet::Border::BORDER_MEDIUM);
    let _ = book.get_sheet_by_name_mut("Sheet2").unwrap().get_style_by_column_and_row_mut(3, 2)
    .get_borders_mut()
    .get_left_mut()
    .set_border_style(umya_spreadsheet::Border::BORDER_THIN);
    
    // change font color.
    let _ = book.get_sheet_by_name_mut("Sheet2").unwrap().get_style_mut("A1")
    .get_font_mut()
    .get_color_mut()
    .set_argb("00FF0000");

    // writer.
    let path = std::path::Path::new("C:/spread_test_data/eee.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path).unwrap();
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