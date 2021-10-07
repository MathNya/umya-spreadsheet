extern crate umya_spreadsheet;

#[test]
fn read_and_wite() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    let _ = book.get_sheet_mut(0).get_cell_mut("A1").set_value("TEST1");
    let a1_value = book.get_sheet(0).unwrap().get_value("A1");
    assert_eq!("TEST1", a1_value);
    let b5_value = book.get_sheet(0).unwrap().get_value("B5");
    assert_eq!(" ", b5_value);

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

    let _ = book.get_sheet_by_name_mut("Sheet1").unwrap().get_cell_mut("A1")
    .set_value("49046881.119999997");

    let _ = book.get_sheet_by_name_mut("Sheet1").unwrap().get_style_mut("A1")
    .get_number_format_mut().set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_NUMBER_COMMA_SEPARATED1);

    let value =  book.get_sheet_by_name_mut("Sheet1").unwrap().get_formatted_value("A1");
    dbg!(&value);

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn read_and_wite_by_empty() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa_empty.xlsx");
    let book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_empty.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn read_and_wite_xlsm() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsm");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    //let _ = book.get_sheet_mut(0).get_cell_mut("A1").set_value("TEST1");
    let _ = book.get_sheet_mut(0).get_cell_by_column_and_row_mut(1, 1).set_value("TEST1");
    let a1_value = book.get_sheet(0).unwrap().get_cell_by_column_and_row(1, 1).unwrap().get_value();
    assert_eq!("TEST1", a1_value);

    //dbg!(book.get_sheet_mut(0).get_style("D10"));

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb.xlsm");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn insert_and_remove_cells() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa_insertCell.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    book.insert_new_row("Sheet1", 2, 3);
    book.insert_new_colmun("Sheet1", "B", 3);
    book.insert_new_colmun_by_index("Sheet1", 2, 3);

    book.remove_row("Sheet1", 6, 2);
    book.remove_colmun_by_index("Sheet1", 6, 2);

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_insertCell.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn new_and_wite() {
    // new file.
    let mut book = umya_spreadsheet::new_file();

    // new worksheet.
    let _ = book.new_sheet("Sheet2");

    // change value.
    book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_mut("A1").set_value("TEST1");
    let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_cell("A1").unwrap().get_value();
    assert_eq!("TEST1", a1_value);

    book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_by_column_and_row_mut(2, 2).set_value_from_i32(1);
    let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_cell_by_column_and_row(2, 2).unwrap().get_value();
    assert_eq!("1", a1_value);

    book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_by_column_and_row_mut(3, 3).set_value_from_bool(true);
    let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_cell_by_column_and_row(3, 3).unwrap().get_value();
    assert_eq!("TRUE", a1_value);

    // add bottom border.
    book.get_sheet_by_name_mut("Sheet2").unwrap().get_style_mut("A1")
    .get_borders_mut()
    .get_bottom_mut()
    .set_border_style(umya_spreadsheet::Border::BORDER_MEDIUM);
    book.get_sheet_by_name_mut("Sheet2").unwrap().get_style_by_column_and_row_mut(3, 2)
    .get_borders_mut()
    .get_left_mut()
    .set_border_style(umya_spreadsheet::Border::BORDER_THIN);
    
    // change font color.
    book.get_sheet_by_name_mut("Sheet2").unwrap().get_style_mut("A1")
    .get_font_mut()
    .get_color_mut()
    .set_argb("00FF0000");

    // writer.
    let path = std::path::Path::new("./tests/result_files/eee.xlsx");
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
