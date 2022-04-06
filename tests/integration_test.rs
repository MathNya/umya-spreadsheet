extern crate chrono;
extern crate umya_spreadsheet;

#[test]
fn read_and_wite() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
    read_and_wite_method(&mut book);

    book.get_sheet_by_name("Sheet1")
        .unwrap()
        .get_image_collection()
        .get(0)
        .unwrap()
        .download_image("./tests/result_files/bbb.png");

    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_image_collection_mut()
        .get_mut(0)
        .unwrap()
        .change_image("./images/sample1.png");

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn lazy_read_and_wite() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();
    read_and_wite_method(&mut book);

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_lazy.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn read_large_string() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa_large_string.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();
    let ns = book.get_sheet_by_name_mut("Sheet1").unwrap();
}

#[test]
fn lazy_read_and_wite_large_string() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa_large_string.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();
    let ns = book.new_sheet("new sheet").unwrap();

    for r in 1..5000 {
        for c in 1..30 {
            let cell = ns.get_cell_by_column_and_row_mut(c, r);
            let _ = cell.set_value_from_string(format!("r{}c{}", r, c));
        }
    }

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_large_string.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn lazy_read_and_wite_no_edit() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_lazy_no_edit.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

fn read_and_wite_method(book: &mut umya_spreadsheet::Spreadsheet) {
    let _ = book.get_sheet_mut(0).get_cell_mut("A1").set_value("TEST1");
    let a1_value = book.get_sheet(0).unwrap().get_value("A1");
    assert_eq!("TEST1", a1_value);
    let _ =  book.get_sheet_mut(0).remove_cell_by_column_and_row_mut(1, 1);
    let a1 = book.get_sheet(0).unwrap().get_cell("A1");
    assert_eq!(a1, None);
    let _ =  book.get_sheet_mut(0).remove_cell_by_column_and_row_mut(1, 2);
    let a2_value = book.get_sheet(0).unwrap().get_value("A2");
    assert_eq!(a2_value, "");
    let b5_value = book.get_sheet(0).unwrap().get_value("B5");
    assert_eq!(" ", b5_value);

    assert_eq!(
        "1.0000",
        book.get_sheet(0)
            .unwrap()
            .get_formatted_value_by_column_and_row(2, 20)
    );
    assert_eq!(
        "$3,333.0000",
        book.get_sheet(0).unwrap().get_formatted_value("B21")
    );
    assert_eq!(
        "$ 333.00",
        book.get_sheet(0).unwrap().get_formatted_value("B22")
    );
    assert_eq!(
        "2020年3月",
        book.get_sheet(0).unwrap().get_formatted_value("B23")
    );
    assert_eq!(
        "2:33 pm",
        book.get_sheet(0).unwrap().get_formatted_value("B24")
    );
    assert_eq!(
        "5.00%",
        book.get_sheet(0).unwrap().get_formatted_value("B25")
    );
    assert_eq!("1/2", book.get_sheet(0).unwrap().get_formatted_value("B26"));
    assert_eq!(
        "12/15/2020 14:01",
        book.get_sheet(0).unwrap().get_formatted_value("B27")
    );
    assert_eq!("444", book.get_sheet(0).unwrap().get_formatted_value("B28"));
    assert_eq!(
        "14-Dec-20",
        book.get_sheet(0).unwrap().get_formatted_value("B29")
    );
    assert_eq!(
        "2020年10月1日",
        book.get_sheet(0).unwrap().get_formatted_value("B30")
    );
    assert_eq!(
        "1.2345",
        book.get_sheet(0).unwrap().get_formatted_value("B31")
    );
    assert_eq!("1.2", book.get_sheet(0).unwrap().get_formatted_value("B32"));
    assert_eq!(
        "12,345,675,544.00",
        book.get_sheet(0).unwrap().get_formatted_value("B33")
    );
    assert_eq!(
        "1.235",
        book.get_sheet(0).unwrap().get_formatted_value("B34")
    );
    assert_eq!("1", book.get_sheet(0).unwrap().get_formatted_value("B35"));
    assert_eq!("", book.get_sheet(0).unwrap().get_formatted_value("B36"));
    assert_eq!(
        "123456789012345678",
        book.get_sheet(0).unwrap().get_formatted_value("B37")
    );

    let _ = book
        .get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("A1")
        .set_value("49046881.119999997");

    let _ = book
        .get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("A1")
        .get_number_format_mut()
        .set_format_code(umya_spreadsheet::NumberingFormat::FORMAT_NUMBER_COMMA_SEPARATED1);

    let value = book
        .get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_formatted_value("A1");
    assert_eq!("49,046,881.12", &value);

    let fg = umya_spreadsheet::Color::default()
        .set_argb(umya_spreadsheet::Color::COLOR_BLACK)
        .to_owned();
    let fill = umya_spreadsheet::PatternFill::default()
        .set_foreground_color(fg)
        .to_owned();
    book.get_sheet_by_name_mut("Sheet5")
        .unwrap()
        .get_row_dimension_mut(&5u32)
        .get_style_mut()
        .get_fill_mut()
        .set_pattern_fill(fill.clone());
    let font_color = umya_spreadsheet::Color::default()
        .set_argb(umya_spreadsheet::Color::COLOR_WHITE)
        .to_owned();
    book.get_sheet_by_name_mut("Sheet5")
        .unwrap()
        .get_row_dimension_mut(&5u32)
        .get_style_mut()
        .get_font_mut()
        .set_color(font_color.clone());
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
fn lazy_read_and_wite_by_empty() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa_empty.xlsx");
    let book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_lazy_empty.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn read_and_wite_xlsm() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsm");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
    read_and_wite_xlsm_method(&mut book);

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb.xlsm");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn lazy_read_and_wite_xlsm() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsm");
    let mut book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();
    read_and_wite_xlsm_method(&mut book);

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_lazy.xlsm");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn lazy_read_and_wite_xlsm_no_edit() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsm");
    let book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_lazy_no_edit.xlsm");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

fn read_and_wite_xlsm_method(book: &mut umya_spreadsheet::Spreadsheet) {
    let _ = book
        .get_sheet_mut(0)
        .get_cell_by_column_and_row_mut(1, 1)
        .set_value("TEST1");
    let a1_value = book
        .get_sheet(0)
        .unwrap()
        .get_cell_by_column_and_row(1, 1)
        .unwrap()
        .get_value();
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
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::LineChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

    // add chart (pie chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("B1");
    to_marker.set_coordinate("C2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::PieChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

    // add chart (doughnut chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("C1");
    to_marker.set_coordinate("D2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::DoughnutChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

    // add chart (area chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("D1");
    to_marker.set_coordinate("E2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::AreaChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

    // add chart (bar chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("E1");
    to_marker.set_coordinate("F2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::BarChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

    // add chart (bar 3d chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("A2");
    to_marker.set_coordinate("B3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::Bar3DChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

    // add chart (line 3d chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("B2");
    to_marker.set_coordinate("C3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::Line3DChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

    // add chart (pie 3d chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("C2");
    to_marker.set_coordinate("D3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::Pie3DChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

    // add chart (area 3d chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("D2");
    to_marker.set_coordinate("E3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::Area3DChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

    // add chart (of pie chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("E2");
    to_marker.set_coordinate("F3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::OfPieChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

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
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

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
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

    // add chart (scatter chart)
    let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("C3");
    to_marker.set_coordinate("D4");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let mut chart = umya_spreadsheet::structs::Chart::default();
    chart.new_chart(
        umya_spreadsheet::structs::ChartType::ScatterChart,
        from_marker,
        to_marker,
        area_chart_series_list,
    );
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_worksheet_drawing_mut()
        .add_chart_collection(chart);

    // Add Image
    let _ = book.new_sheet("Sheet Image");
    let mut marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
    marker.set_coordinate("B3");
    let mut image = umya_spreadsheet::structs::Image::default();
    image.new_image("./images/sample1.png", marker);
    book.get_sheet_by_name_mut("Sheet Image")
        .unwrap()
        .add_image(image);
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
fn new_file_and_edit() {
    // new file.
    let mut book = umya_spreadsheet::new_file();

    // new worksheet.
    let _ = book.new_sheet("Sheet2");
    let _ = book.new_sheet("Sheet3");

    // change value.
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_mut("A1")
        .set_value("TEST1");
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell("A1")
        .unwrap()
        .get_value();
    assert_eq!("TEST1", a1_value);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_by_column_and_row_mut(2, 2)
        .set_value_from_i32(1);
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell_by_column_and_row(2, 2)
        .unwrap()
        .get_value();
    assert_eq!("1", a1_value);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_by_column_and_row_mut(2, 2)
        .set_value_from_i32_ref(&1);
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell_by_column_and_row(2, 2)
        .unwrap()
        .get_value();
    assert_eq!("1", a1_value);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_by_column_and_row_mut(3, 3)
        .set_value_from_bool(true);
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell_by_column_and_row(3, 3)
        .unwrap()
        .get_value();
    assert_eq!("TRUE", a1_value);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_by_column_and_row_mut(3, 3)
        .set_value_from_bool_ref(&true);
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell_by_column_and_row(3, 3)
        .unwrap()
        .get_value();
    assert_eq!("TRUE", a1_value);

    // add bottom border.
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_mut("A1")
        .get_borders_mut()
        .get_bottom_mut()
        .set_border_style(umya_spreadsheet::Border::BORDER_MEDIUM);
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_by_column_and_row_mut(3, 2)
        .get_borders_mut()
        .get_left_mut()
        .set_border_style(umya_spreadsheet::Border::BORDER_THIN);

    // change font color.
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_mut("A1")
        .get_font_mut()
        .get_color_mut()
        .set_argb("00FF0000");

    let worksheet = book.get_sheet_by_name_mut("Sheet3").unwrap();
    worksheet.get_column_dimension_mut("A").set_auto_width(true);

    worksheet.get_cell_mut("E1").set_value("テスト");
    worksheet.get_cell_mut("E2").set_value("うみゃーねっと");
    worksheet.get_cell_mut("E3").set_value("案案案案");
    worksheet.get_column_dimension_mut("E").set_auto_width(true);

    worksheet.get_cell_mut("F1").set_value("AAAAAAAAAAAAAAAAAA");
    worksheet.get_cell_mut("F2").set_value("BBBBBBBBBBB");
    worksheet
        .get_cell_mut("F4")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.get_column_dimension_mut("F").set_auto_width(true);

    worksheet.get_cell_mut("G1").set_value("AAAAAAAAAAAAAAAAAA");
    worksheet.get_cell_mut("G2").set_value("BBBBBBBBBBB");
    worksheet
        .get_cell_mut("G3")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.get_column_dimension_mut("G").set_width(60f64);

    worksheet.get_cell_mut("D1").set_value("テスト");
    worksheet.get_cell_mut("D2").set_value("うみゃーねっと");
    worksheet.get_cell_mut("D3").set_value("案案案案");
    worksheet.get_column_dimension_mut("D").set_auto_width(true);

    worksheet.get_cell_mut("H1").set_value("テスト");
    worksheet
        .get_cell_mut("H2")
        .set_value("うみゃーねっと\nうみゃーねっと")
        .get_style_mut()
        .get_alignment_mut()
        .set_wrap_text(true);
    worksheet.get_cell_mut("H3").set_value("案案案案");
    worksheet.get_column_dimension_mut("H").set_auto_width(true);

    worksheet.get_cell_mut("I1").set_value("テスト");
    worksheet
        .get_cell_mut("I2")
        .set_value("うみゃーねっと")
        .get_style_mut()
        .get_font_mut()
        .get_font_size_mut()
        .set_val(20f64);
    worksheet.get_cell_mut("I3").set_value("案案案案");
    worksheet.get_column_dimension_mut("I").set_auto_width(true);

    worksheet
        .get_cell_mut("J2")
        .set_value("うみゃーねっと")
        .get_style_mut()
        .get_font_mut()
        .get_font_size_mut()
        .set_val(5f64);
    worksheet.get_column_dimension_mut("J").set_auto_width(true);

    worksheet
        .get_cell_mut("K4")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.get_column_dimension_mut("K").set_auto_width(true);

    worksheet
        .get_cell_mut("L4")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.get_column_dimension_mut("L").set_auto_width(true);

    worksheet
        .get_cell_mut("M4")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.get_column_dimension_mut("M").set_auto_width(true);

    worksheet
        .get_cell_mut("N1")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.get_column_dimension_mut("N").set_auto_width(true);

    worksheet.add_merge_cells("K8:L8");
    worksheet.add_merge_cells("M8:M10");
    worksheet.add_merge_cells("N:N");

    // writer.
    let path = std::path::Path::new("./tests/result_files/eee.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path).unwrap();
}

#[test]
fn new_and_wite() {
    // new file.
    let book = umya_spreadsheet::new_file();

    // writer.
    let path = std::path::Path::new("./tests/result_files/fff.xlsx");
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
