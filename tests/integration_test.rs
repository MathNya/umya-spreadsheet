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

    assert_eq!("1.0000",            book.get_sheet(0).unwrap().get_formatted_value_by_column_and_row(2, 20));
    assert_eq!("$3,333.0000",       book.get_sheet(0).unwrap().get_formatted_value("B21"));
    assert_eq!("$ 333.00",          book.get_sheet(0).unwrap().get_formatted_value("B22"));
    assert_eq!("2020年3月",         book.get_sheet(0).unwrap().get_formatted_value("B23"));
    assert_eq!("2:33 pm",           book.get_sheet(0).unwrap().get_formatted_value("B24"));
    assert_eq!("5.00%",             book.get_sheet(0).unwrap().get_formatted_value("B25"));
    assert_eq!("1/2",               book.get_sheet(0).unwrap().get_formatted_value("B26"));
    assert_eq!("12/15/2020 14:01",  book.get_sheet(0).unwrap().get_formatted_value("B27"));
    assert_eq!("444",               book.get_sheet(0).unwrap().get_formatted_value("B28"));
    assert_eq!("14-Dec-20",         book.get_sheet(0).unwrap().get_formatted_value("B29"));
    assert_eq!("2020年10月1日",     book.get_sheet(0).unwrap().get_formatted_value("B30"));
    assert_eq!("1.2345",            book.get_sheet(0).unwrap().get_formatted_value("B31"));
    assert_eq!("1.2",               book.get_sheet(0).unwrap().get_formatted_value("B32"));
    assert_eq!("12,345,675,544.00", book.get_sheet(0).unwrap().get_formatted_value("B33"));
    assert_eq!("1.235",             book.get_sheet(0).unwrap().get_formatted_value("B34"));
    assert_eq!("1",                 book.get_sheet(0).unwrap().get_formatted_value("B35"));
    assert_eq!("",                  book.get_sheet(0).unwrap().get_formatted_value("B36"));
    assert_eq!("123456789012345678", book.get_sheet(0).unwrap().get_formatted_value("B37"));

    let _ = book.get_sheet_by_name_mut("Sheet1").unwrap().get_cell_mut("A1")
    .set_value("49046881.119999997");

    let _ = book.get_sheet_by_name_mut("Sheet1").unwrap().get_style_mut("A1")
    .get_number_format_mut().set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_NUMBER_COMMA_SEPARATED1);

    let value =  book.get_sheet_by_name_mut("Sheet1").unwrap().get_formatted_value("A1");
    assert_eq!("49,046,881.12", &value);

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

    // copy sheet
    let mut clone_sheet = book.get_sheet(0).unwrap().clone();
    clone_sheet.set_title("New Sheet");
    let _ = book.add_sheet(clone_sheet);

    // add chart (line chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("A1");
    to_marker.set_coordinate("B2");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::LineChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);

    // add chart (pie chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("B1");
    to_marker.set_coordinate("C2");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::PieChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);

    // add chart (doughnut chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("C1");
    to_marker.set_coordinate("D2");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::DoughnutChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);
    
    // add chart (area chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("D1");
    to_marker.set_coordinate("E2");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::AreaChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);

    // add chart (bar chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("E1");
    to_marker.set_coordinate("F2");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::BarChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);

    // add chart (bar 3d chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("A2");
    to_marker.set_coordinate("B3");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::Bar3DChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);

    // add chart (line 3d chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("B2");
    to_marker.set_coordinate("C3");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::Line3DChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);

    // add chart (pie 3d chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("C2");
    to_marker.set_coordinate("D3");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::Pie3DChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);

    // add chart (area 3d chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("D2");
    to_marker.set_coordinate("E3");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::Area3DChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);

    // add chart (of pie chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("E2");
    to_marker.set_coordinate("F3");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::OfPieChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);

    // add chart (bubble chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("A3");
    to_marker.set_coordinate("B4");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
        "New Sheet!$I$7:$I$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::BubbleChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);

    // add chart (radar chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("B3");
    to_marker.set_coordinate("C4");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
        "New Sheet!$I$7:$I$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::RadarChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);

    // add chart (scatter chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("C3");
    to_marker.set_coordinate("D4");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
    ];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::ScatterChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);

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

    book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_by_column_and_row_mut(2, 2).set_value_from_i32_ref(&1);
    let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_cell_by_column_and_row(2, 2).unwrap().get_value();
    assert_eq!("1", a1_value);

    book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_by_column_and_row_mut(3, 3).set_value_from_bool(true);
    let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_cell_by_column_and_row(3, 3).unwrap().get_value();
    assert_eq!("TRUE", a1_value);

    book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_by_column_and_row_mut(3, 3).set_value_from_bool_ref(&true);
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
