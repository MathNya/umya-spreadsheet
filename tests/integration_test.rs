#![allow(warnings)]
#![allow(clippy::all)]

extern crate chrono;
extern crate umya_spreadsheet;
use std::time::Instant;

use umya_spreadsheet::*;

use crate::helper::color;

#[test]
fn read_and_wite() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    read_and_wite_method(&mut book);

    book.get_sheet_by_name("Sheet1")
        .unwrap()
        .get_image("M17")
        .unwrap()
        .download_image("./tests/result_files/bbb.png");

    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_image_mut("M17")
        .unwrap()
        .change_image("./images/sample1.png");

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn read_and_wite_with_password() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    read_and_wite_method(&mut book);

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_password.xlsx");
    let _unused = writer::xlsx::write_with_password(&book, path, "password");
}

#[test]
fn wite_with_password() {
    // writer
    let from_path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let to_path = std::path::Path::new("./tests/result_files/bbb_password2.xlsx");
    let _unused = writer::xlsx::set_password(&from_path, &to_path, "password");
}

#[test]
fn lazy_read_and_wite() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = reader::xlsx::lazy_read(path).unwrap();
    read_and_wite_method(&mut book);

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_lazy.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn read_and_wite_libre2() {
    // reader
    let path = std::path::Path::new("./tests/test_files/libre2.xlsx");
    let book = reader::xlsx::read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/libre2.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn read_large_string() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa_large_string.xlsx");
    let mut book = reader::xlsx::lazy_read(path).unwrap();
    // let _ns = book.get_sheet_by_name_mut("Sheet1").unwrap();
}

#[test]
fn lazy_read_and_wite_large_string() {
    // reader
    let start = Instant::now();
    let path = std::path::Path::new("./tests/test_files/aaa_large_string.xlsx");
    let mut book = reader::xlsx::lazy_read(path).unwrap();
    let ns = book.new_sheet("new sheet").unwrap();
    let end = start.elapsed();
    println!("read:{}.{:03}sec.", end.as_secs(), end.subsec_millis());

    let start = Instant::now();
    for r in 1..5000 {
        for c in 1..30 {
            let cell = ns.get_cell_mut((c, r));
            let _unused = cell.set_value_string(format!("r{}c{}", r, c));
        }
    }
    let end = start.elapsed();
    println!("edit:{}.{:03}sec.", end.as_secs(), end.subsec_millis());

    // writer
    let start = Instant::now();
    let path = std::path::Path::new("./tests/result_files/bbb_large_string.xlsx");
    let _unused = writer::xlsx::write(&book, path);
    let end = start.elapsed();
    println!("write:{}.{:03}sec.", end.as_secs(), end.subsec_millis());
}

#[test]
fn lazy_read_and_wite_no_edit() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let book = reader::xlsx::lazy_read(path).unwrap();

    let cells = book.get_lazy_read_sheet_cells(0).unwrap();
    assert_eq!("英語", cells.get_cell_value((5, 12)).get_value());
    assert_eq!("英語", cells.get_cell_value("E12").get_value());

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_lazy_no_edit.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

fn read_and_wite_method(book: &mut Workbook) {
    let _unused = book
        .get_sheet_mut(0)
        .unwrap()
        .get_cell_mut("A1")
        .set_value("TEST1");
    let a1_value = book.get_sheet(0).unwrap().get_value("A1");
    assert_eq!("TEST1", a1_value);
    let _unused = book.get_sheet_mut(0).unwrap().remove_cell((1, 1));
    let a1 = book.get_sheet(0).unwrap().get_cell("A1");
    assert_eq!(a1, None);
    let _unused = book.get_sheet_mut(0).unwrap().remove_cell((1, 2));
    let a2_value = book.get_sheet(0).unwrap().get_value("A2");
    assert_eq!(a2_value, "");
    let b5_value = book.get_sheet(0).unwrap().get_value("B5");
    assert_eq!(" ", b5_value);

    assert_eq!(
        "1.0000",
        book.get_sheet(0).unwrap().get_formatted_value((2, 20))
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

    let _unused = book
        .get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_cell_mut("A1")
        .set_value("49046881.119999997");

    let _unused = book
        .get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_style_mut("A1")
        .get_number_format_mut()
        .set_format_code(NumberingFormat::FORMAT_NUMBER_COMMA_SEPARATED1);

    let value = book
        .get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_formatted_value("A1");
    assert_eq!("49,046,881.12", &value);

    let fg = Color::default().set_argb(Color::COLOR_BLACK).to_owned();
    let fill = PatternFill::default().set_foreground_color(fg).to_owned();
    book.get_sheet_by_name_mut("Sheet5")
        .unwrap()
        .get_row_dimension_mut(5u32)
        .get_style_mut()
        .get_fill_mut()
        .set_pattern_fill(fill);
    let font_color = Color::default().set_argb(Color::COLOR_WHITE).to_owned();
    book.get_sheet_by_name_mut("Sheet5")
        .unwrap()
        .get_row_dimension_mut(5u32)
        .get_style_mut()
        .get_font_mut()
        .set_color(font_color);

    let _unused = book
        .get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .get_cell_mut("A1")
        .get_style_mut()
        .get_font_mut()
        .set_name("Arial");

    book.get_sheet_by_name_mut("Sheet1")
        .unwrap()
        .get_row_dimension_mut(3)
        .set_height(46.0);
}

#[test]
fn read_and_wite_by_empty() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa_empty.xlsx");
    let book = reader::xlsx::read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_empty.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn lazy_read_and_wite_by_empty() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa_empty.xlsx");
    let book = reader::xlsx::lazy_read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_lazy_empty.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn read_and_wite_xlsm() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsm");
    let mut book = reader::xlsx::read(path).unwrap();
    read_and_wite_xlsm_method(&mut book);

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb.xlsm");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn lazy_read_and_wite_xlsm() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsm");
    let mut book = reader::xlsx::lazy_read(path).unwrap();
    read_and_wite_xlsm_method(&mut book);

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_lazy.xlsm");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn lazy_read_and_wite_xlsm2() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsm");
    let mut book = reader::xlsx::lazy_read(path).unwrap();

    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
    let cell = sheet.get_cell_mut((1, 2));
    cell.set_value("test");

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_lazy2.xlsm");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn lazy_read_and_wite_xlsm_no_edit() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsm");
    let book = reader::xlsx::lazy_read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_lazy_no_edit.xlsm");
    let _unused = writer::xlsx::write(&book, path);
}

fn read_and_wite_xlsm_method(book: &mut Workbook) {
    let _unused = book
        .get_sheet_mut(0)
        .unwrap()
        .get_cell_mut((1, 1))
        .set_value("TEST1");
    let a1_value = book
        .get_sheet(0)
        .unwrap()
        .get_cell((1, 1))
        .unwrap()
        .get_value();
    assert_eq!("TEST1", a1_value);

    // copy sheet
    let mut clone_sheet = book.get_sheet(0).unwrap().clone();
    clone_sheet.set_name("New Sheet");
    let _unused = book.add_sheet(clone_sheet);

    // remove sheet
    let mut clone_sheet = book.get_sheet(0).unwrap().clone();
    clone_sheet.set_name("DeletedSheet");
    let _unused = book.add_sheet(clone_sheet);
    book.get_sheet_by_name("DeletedSheet").unwrap();
    book.remove_sheet_by_name("DeletedSheet").unwrap();

    // add chart (line chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("A1");
    to_marker.set_coordinate("B2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::LineChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title")
        .set_grouping(drawing::charts::GroupingValues::Standard);
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (pie chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("B1");
    to_marker.set_coordinate("C2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::PieChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (doughnut chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("C1");
    to_marker.set_coordinate("D2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::DoughnutChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (area chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("D1");
    to_marker.set_coordinate("E2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::AreaChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (bar chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("E1");
    to_marker.set_coordinate("F2");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::BarChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (bar 3d chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("A2");
    to_marker.set_coordinate("B3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::Bar3DChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (line 3d chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("B2");
    to_marker.set_coordinate("C3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::Line3DChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (pie 3d chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("C2");
    to_marker.set_coordinate("D3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::Pie3DChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (area 3d chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("D2");
    to_marker.set_coordinate("E3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::Area3DChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (of pie chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("E2");
    to_marker.set_coordinate("F3");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::OfPieChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (bubble chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("A3");
    to_marker.set_coordinate("B4");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
        "New Sheet!$I$7:$I$10",
    ];
    let series_title_list = vec!["Line1", "Line2", "Line3"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::BubbleChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (radar chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("B3");
    to_marker.set_coordinate("C4");
    let area_chart_series_list = vec![
        "New Sheet!$G$7:$G$10",
        "New Sheet!$H$7:$H$10",
        "New Sheet!$I$7:$I$10",
    ];
    let series_title_list = vec!["Line1", "Line2", "Line3"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::RadarChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // add chart (scatter chart)
    let mut from_marker = drawing::spreadsheet::MarkerType::default();
    let mut to_marker = drawing::spreadsheet::MarkerType::default();
    from_marker.set_coordinate("C3");
    to_marker.set_coordinate("D4");
    let area_chart_series_list = vec!["New Sheet!$G$7:$G$10", "New Sheet!$H$7:$H$10"];
    let series_title_list = vec!["Line1", "Line2"];
    let series_point_title_list = vec!["Point1", "Point2", "Point3", "Point4"];
    let mut chart = Chart::default();
    chart
        .new_chart(
            &ChartType::ScatterChart,
            from_marker,
            to_marker,
            area_chart_series_list,
        )
        .set_series_title(series_title_list)
        .set_series_point_title(series_point_title_list)
        .set_default_language("ja-JP")
        .set_title("Chart Title")
        .set_horizontal_title("Horizontal Title")
        .set_vertical_title("Vertical Title");
    book.get_sheet_by_name_mut("Sheet7")
        .unwrap()
        .add_chart(chart);

    // Add Image
    let _unused = book.new_sheet("Sheet Image");
    let mut marker = drawing::spreadsheet::MarkerType::default();
    marker.set_coordinate("B3");
    let mut image = Image::default();
    image.new_image("./images/sample1.png", marker);
    book.get_sheet_by_name_mut("Sheet Image")
        .unwrap()
        .add_image(image);
}

#[test]
fn insert_and_remove_cells() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa_insertCell.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    book.insert_new_row("Sheet1", 2, 3);
    book.insert_new_column("Sheet1", "B", 3);
    book.insert_new_column_by_index("Sheet1", 2, 3);

    book.remove_row("Sheet1", 6, 2);
    book.remove_column_by_index("Sheet1", 6, 2);

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_insertCell.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn new_sheet_and_edit() {
    const BG_COLOR: &str = "#333";
    const TEST_SHEET: &str = "Sheet2233";

    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = reader::xlsx::lazy_read(path).unwrap();

    // set cell value
    let sheet = book.new_sheet(TEST_SHEET).unwrap();
    let cell = sheet.get_cell_mut("A2");
    let _unused = cell.set_value("test");

    // set style by range
    let mut style = Style::default();
    style.set_background_color(BG_COLOR);
    sheet.set_style_by_range("A3:A4", &style);

    let path = std::path::Path::new("./tests/result_files/bbb_new_sheet_value.xlsx");
    let _unused = writer::xlsx::write(&book, path);

    let mut book = reader::xlsx::lazy_read(path).unwrap();
    let a2_value = book
        .get_sheet_by_name_mut(TEST_SHEET)
        .unwrap()
        .get_cell("A2")
        .unwrap()
        .get_value();
    assert_eq!("test", a2_value);

    {
        let a3_bg = book
            .get_sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .get_style_mut("A3")
            .get_fill_mut()
            .get_pattern_fill_mut()
            .get_foreground_color_mut()
            .get_argb_str();

        assert_eq!(a3_bg, "FF333333");
    }

    {
        let a4_bg = book
            .get_sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .get_style_mut("A4")
            .get_fill_mut()
            .get_pattern_fill_mut()
            .get_foreground_color_mut()
            .get_argb_str();

        assert_eq!(a4_bg, "FF333333");
    }
}

#[test]
fn three_digit_hex_color_with_hash() {
    const BG_COLOR: &str = "#F0D";
    const TEST_SHEET: &str = "Sheet2233";

    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = reader::xlsx::lazy_read(path).unwrap();

    // set cell value
    let sheet = book.new_sheet(TEST_SHEET).unwrap();
    let cell = sheet.get_cell_mut("A2");
    let _unused = cell.set_value("test");

    // set style by range
    let mut style = Style::default();
    style.set_background_color(BG_COLOR);
    sheet.set_style_by_range("A3:A4", &style);

    let path = std::path::Path::new("./tests/result_files/three_digit_hex_color_with_hash.xlsx");
    let _unused = writer::xlsx::write(&book, path);

    let mut book = reader::xlsx::lazy_read(path).unwrap();
    let a2_value = book
        .get_sheet_by_name_mut(TEST_SHEET)
        .unwrap()
        .get_cell("A2")
        .unwrap()
        .get_value();
    assert_eq!("test", a2_value);

    {
        let a3_bg = book
            .get_sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .get_style_mut("A3")
            .get_fill_mut()
            .get_pattern_fill_mut()
            .get_foreground_color_mut()
            .get_argb_str();

        assert_eq!(a3_bg, "FFFF00DD");
    }

    {
        let a4_bg = book
            .get_sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .get_style_mut("A4")
            .get_fill_mut()
            .get_pattern_fill_mut()
            .get_foreground_color_mut()
            .get_argb_str();

        assert_eq!(a4_bg, "FFFF00DD");
    }
}

#[test]
fn three_digit_hex_color_without_hash() {
    const BG_COLOR: &str = "ACE";
    const TEST_SHEET: &str = "Sheet2233";

    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = reader::xlsx::lazy_read(path).unwrap();

    // set cell value
    let sheet = book.new_sheet(TEST_SHEET).unwrap();
    let cell = sheet.get_cell_mut("A2");
    let _unused = cell.set_value("test");

    // set style by range
    let mut style = Style::default();
    style.set_background_color(BG_COLOR);
    sheet.set_style_by_range("A3:A4", &style);

    let path = std::path::Path::new("./tests/result_files/three_digit_hex_color_without_hash.xlsx");
    let _unused = writer::xlsx::write(&book, path);

    let mut book = reader::xlsx::lazy_read(path).unwrap();
    let a2_value = book
        .get_sheet_by_name_mut(TEST_SHEET)
        .unwrap()
        .get_cell("A2")
        .unwrap()
        .get_value();
    assert_eq!("test", a2_value);

    {
        let a3_bg = book
            .get_sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .get_style_mut("A3")
            .get_fill_mut()
            .get_pattern_fill_mut()
            .get_foreground_color_mut()
            .get_argb_str();

        assert_eq!(a3_bg, "FFAACCEE");
    }

    {
        let a4_bg = book
            .get_sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .get_style_mut("A4")
            .get_fill_mut()
            .get_pattern_fill_mut()
            .get_foreground_color_mut()
            .get_argb_str();

        assert_eq!(a4_bg, "FFAACCEE");
    }
}

#[test]
fn new_file_and_edit() {
    // new file.
    let mut book = new_file();

    // new worksheet.
    let _unused = book.new_sheet("Sheet2");
    let _unused = book.new_sheet("Sheet3");

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
        .get_cell_mut((2, 2))
        .set_value_number(1);
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell((2, 2))
        .unwrap()
        .get_value();
    assert_eq!("1", a1_value);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_mut((2, 2))
        .set_value_number(1);
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell((2, 2))
        .unwrap()
        .get_value();
    assert_eq!("1", a1_value);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_mut((3, 3))
        .set_value_bool(true);
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell((3, 3))
        .unwrap()
        .get_value();
    assert_eq!("TRUE", a1_value);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_cell_mut((3, 3))
        .set_value_bool(true);
    let a1_value = book
        .get_sheet_by_name("Sheet2")
        .unwrap()
        .get_cell((3, 3))
        .unwrap()
        .get_value();
    assert_eq!("TRUE", a1_value);

    // add bottom border.
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_mut("A1")
        .get_borders_mut()
        .get_bottom_mut()
        .set_border_style(Border::BORDER_MEDIUM);
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_mut((3, 2))
        .get_borders_mut()
        .get_left_mut()
        .set_border_style(Border::BORDER_THIN);

    // change font color.
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_mut("A1")
        .get_font_mut()
        .get_color_mut()
        .set_argb_str("00FF0000");

    // change background color.
    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_mut("A1")
        .set_background_color(Color::COLOR_BLUE_STR);

    book.get_sheet_by_name_mut("Sheet2")
        .unwrap()
        .get_style_mut("A2")
        .set_background_color_with_pattern(
            Color::COLOR_BLUE_STR,
            Color::COLOR_RED_STR,
            PatternValues::DarkGrid,
        );

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
    writer::xlsx::write(&book, path).unwrap();
}

#[test]
fn new_and_wite() {
    // new file.
    let book = new_file();

    // writer.
    let path = std::path::Path::new("./tests/result_files/fff.xlsx");
    writer::xlsx::write(&book, path).unwrap();
}

#[test]
fn duplicate_sheet() {
    let mut book = new_file();
    let _unused = book.new_sheet("Sheet2");
    if book.new_sheet("Sheet2").is_ok() {
        panic!("getting new sheet..")
    }
}

#[test]
fn witer_csv() {
    let mut book = new_file();
    book.set_active_sheet(1);
    let sheet = book.new_sheet("Sheet2").unwrap();
    // ---
    sheet.get_cell_mut("A1").set_value(" TEST");
    sheet.get_cell_mut("B1").set_value("あいうえお");
    sheet.get_cell_mut("C1").set_value("漢字");
    sheet.get_cell_mut("E1").set_value("1");
    // ---
    sheet.get_cell_mut("A2").set_value("TEST ");
    sheet.get_cell_mut("B2").set_value("あいうえお");
    sheet.get_cell_mut("C2").set_value("漢字");
    // ---
    sheet.get_cell_mut("A3").set_value(" TEST ");
    // ---

    // writer
    let mut option = CsvWriterOption::default();
    option.set_csv_encode_value(CsvEncodeValues::ShiftJis);
    option.set_do_trim(true);
    option.set_wrap_with_char("\"");
    let path = std::path::Path::new("./tests/result_files/bbb.csv");
    let _unused = writer::csv::write(&book, path, Some(&option));
}

#[test]
fn new_file_empty_worksheet() {
    let book = umya_spreadsheet::new_file_empty_worksheet();
    let path = std::path::Path::new("./tests/result_files/empty_worksheet.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn google() {
    // reader
    let path = std::path::Path::new("./tests/test_files/google.xlsx");
    let book = reader::xlsx::read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/google.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn libre() {
    // reader
    let path = std::path::Path::new("./tests/test_files/libre.xlsm");
    let book = reader::xlsx::read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/libre.xlsm");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn read_and_wite_theme() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa_theme.xlsx");
    let book = reader::xlsx::read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_theme.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn openpyxl() {
    let path = std::path::Path::new("./tests/test_files/openpyxl.xlsx");
    let book = reader::xlsx::read(path).unwrap();
    let sheet = book.get_active_sheet();

    assert_eq!(sheet.get_cell("A1").unwrap().get_value(), "TEST");
    assert_eq!(sheet.get_cell("A2").unwrap().get_value(), " TEST ");

    let path = std::path::Path::new("./tests/result_files/openpyxl.xlsx");
    writer::xlsx::write(&book, path).unwrap();
}

#[test]
fn read_and_wite_2() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa_2.xlsx");
    let book = reader::xlsx::read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_2.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_110() {
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
    let cell = sheet.get_cell_mut("A1");
    // work on 0.87
    // cell.set_value("test".to_string());
    // work on 0.9
    cell.set_value("test");
    let path = std::path::Path::new("./tests/result_files/aaa_issue_110.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn compression_test() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let book = reader::xlsx::read(path).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_comp_standard.xlsx");
    let _unused = writer::xlsx::write(&book, path);

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_comp_light.xlsx");
    let _unused = writer::xlsx::write_light(&book, path);
}

#[test]
fn move_range_test() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa_move_range.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let sheet_name = "Sheet1";
    let range = "C5:F9";
    let row = 12;
    let column = 4;
    book.get_sheet_by_name_mut(sheet_name)
        .unwrap()
        .move_range(range, row, column);

    // Checking to ensure cells that move into another cell overwrites it
    let range_2 = "A14:A14";
    book.get_sheet_by_name_mut(sheet_name)
        .unwrap()
        .move_range(range, row, column);
    book.get_sheet_by_name_mut(sheet_name)
        .unwrap()
        .move_range(range_2, 0, 1);

    let path = std::path::Path::new("./tests/result_files/bbb_move_range.xlsx");
    let _unused = writer::xlsx::write_light(&book, path);
}

#[test]
fn issue_72() {
    let xlsx_path = std::path::Path::new("./tests/test_files/wps_comment.xlsx");

    let wb = reader::xlsx::read(xlsx_path).unwrap();

    // save to new file
    let path = std::path::Path::new("./tests/result_files/wps_comment_corrupted.xlsx");
    writer::xlsx::write(&wb, path).unwrap();
}

#[test]
fn issue_129() {
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let book = reader::xlsx::read(path).unwrap();
    let img = book
        .get_sheet_by_name("Sheet1")
        .unwrap()
        .get_image("M17")
        .unwrap();
    // dbg!(img.get_one_cell_anchor().is_some());
    // dbg!(img.get_two_cell_anchor().is_some());
    assert_eq!(img.get_image_name(), "image1.png");
    assert_eq!(img.get_coordinate(), "M17");
    assert_eq!(img.get_col(), 12);
    assert_eq!(img.get_row(), 16);

    let path = std::path::Path::new("./tests/result_files/issue_129.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn wb_with_shared_strings() {
    let path = std::path::Path::new("./tests/test_files/wb_with_shared_strings.xlsx");
    let book = reader::xlsx::read(path).unwrap();
    let sheet = book.get_sheet_by_name("Sheet To Read From").unwrap();

    assert_eq!(sheet.get_cell("A2").unwrap().get_value(), "11");
    assert_eq!(sheet.get_cell("A3").unwrap().get_value(), "22");
    assert_eq!(sheet.get_cell("A4").unwrap().get_value(), "ABCdef");
    assert_eq!(sheet.get_cell("A5").unwrap().get_value(), "ABCdef");
    assert_eq!(sheet.get_cell("A6").unwrap().get_value(), "ABCdef");

    let path = std::path::Path::new("./tests/result_files/wb_with_shared_strings.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_test() {
    let path = std::path::Path::new("./tests/test_files/table.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/table.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn sheetlock_test() {
    let path = std::path::Path::new("./tests/test_files/sheet_lock.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let mut sheet = book.get_sheet_mut(2).unwrap();
    sheet
        .get_sheet_protection_mut()
        .set_password("password")
        .set_sheet(true);

    let path = std::path::Path::new("./tests/result_files/sheet_lock.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn workbooklock_test() {
    let path = std::path::Path::new("./tests/test_files/book_lock.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    book.get_workbook_protection_mut()
        .set_workbook_password("password");

    let path = std::path::Path::new("./tests/result_files/book_lock.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_147() {
    let path = std::path::Path::new("./tests/test_files/issue_147.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let from_sheet = book.get_sheet(0).unwrap();
    let source_cell = from_sheet.get_cell((2, 3)).unwrap();
    let mut target_cell = source_cell.clone();

    let mut to_sheet = book.get_sheet_mut(1).unwrap();
    to_sheet.set_cell(target_cell);

    let path = std::path::Path::new("./tests/result_files/issue_147.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn html_to_richtext_test() {
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let mut sheet = book.get_sheet_by_name_mut("Sheet1").unwrap();

    let html = r##"<font color="red">test</font><br><font class="test" color="#48D1CC">TE<b>S</b>T<br/>TEST</font>"##;
    let richtext = helper::html::html_to_richtext(html).unwrap();

    sheet.get_cell_mut("G16").set_rich_text(richtext);
    sheet
        .get_cell_mut("G16")
        .get_style_mut()
        .get_alignment_mut()
        .set_wrap_text(true);

    let path = std::path::Path::new("./tests/result_files/bbb_html_to_richtext.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_162() {
    let path = std::path::Path::new("./tests/test_files/issue_162.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_162.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_172() {
    let value = helper::date::convert_date(2024, 2, 3, 10, 59, 00);
    let mut numbering_format = NumberingFormat::default();
    numbering_format.set_format_code("dd-mmm-yy");

    let mut book = new_file();
    let mut sheet = book.get_sheet_mut(0).unwrap();
    sheet.get_cell_mut("A1").set_value_number(value);
    sheet
        .get_style_mut("A1")
        .set_numbering_format(numbering_format);

    let result = sheet.get_formatted_value("A1");
    assert_eq!("03-Feb-24", result);
}

#[test]
fn issue_177() {
    let test = helper::coordinate::CellCoordinates::from("A1");
    let test2 = test.clone();
    assert_eq!(test.row, test2.row);
    assert_eq!(test.col, test2.col);
}

#[test]
fn issue_178() {
    let path = std::path::Path::new("./tests/test_files/issue_178.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_178.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_181() {
    let path = std::path::Path::new("./tests/test_files/issue_181.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let mut sheet = book.get_sheet_by_name_mut("LOV").unwrap();
    sheet.remove_row(2, 1);
    for (key, row) in sheet.get_row_dimensions_to_hashmap() {
        assert_eq!(key, &1);
        assert_eq!(row.get_row_num(), 1);
        assert_eq!(row.get_height(), 35.0);
    }
    let path = std::path::Path::new("./tests/result_files/issue_181.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_181_2() {
    let path = std::path::Path::new("./tests/test_files/issue_181_2.xlsx");

    let mut book = reader::xlsx::read(path).unwrap();
    let shee1: &mut Worksheet = book.get_sheet_mut(0).unwrap();
    let new_row_index = 4;
    shee1.insert_new_row(new_row_index, 5);
    shee1.get_cell_mut((1, new_row_index)).set_value("123");

    let template_row_index = 3;
    shee1.remove_row(template_row_index, 1);

    shee1.remove_column("B", 1);

    let path = std::path::Path::new("./tests/result_files/issue_181_2.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_178_2() {
    let path = std::path::Path::new("./tests/test_files/issue_178_2.xlsx");

    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_178_2.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_185() {
    let path = std::path::Path::new("./tests/test_files/issue_185.xlsx");
    let book = reader::xlsx::read(path).unwrap();
    assert_eq!(
        book.get_sheet(0)
            .unwrap()
            .get_cell("A1")
            .unwrap()
            .is_formula(),
        true
    );
}

#[test]
fn issue_187() {
    let path = std::path::Path::new("./tests/test_files/issue_187.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let mut sheet = book.get_sheet_mut(0).unwrap();

    let mut cell = sheet.get_cell("C4").unwrap().clone();
    cell.set_coordinate("B5");
    sheet.set_cell(cell);

    let mut cell = sheet.get_cell("C4").unwrap().clone();
    cell.set_coordinate("D6");
    sheet.set_cell(cell);

    let mut cell = sheet.get_cell("C4").unwrap().clone();
    cell.set_coordinate("C7");
    sheet.set_cell(cell);

    let path = std::path::Path::new("./tests/result_files/issue_187.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_188() {
    let path = std::path::Path::new("./tests/test_files/issue_188.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_188.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_188_2() {
    let path = std::path::Path::new("./tests/test_files/issue_188_2.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_188_2.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_189() {
    let path = std::path::Path::new("./tests/test_files/issue_189.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_189.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn expect_red_indexed_color() {
    let path = std::path::Path::new("./tests/test_files/red_indexed_color.xlsx");
    let book = reader::xlsx::read(path).unwrap();

    let cell = book.get_sheet(0).unwrap().get_cell("A1").unwrap();

    let color = cell
        .get_style()
        .get_font()
        .unwrap()
        .get_color()
        .get_argb_str();

    assert_eq!("FFFF0000", color);
}

#[test]
fn issue_190() {
    let path = std::path::Path::new("./tests/test_files/issue_190.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    // remove
    book.get_sheet_mut(0).unwrap().remove_column("E", 1);
    book.get_sheet_mut(0).unwrap().remove_row(4, 1);

    // insert
    book.get_sheet_mut(1).unwrap().insert_new_column("E", 1);
    book.get_sheet_mut(1).unwrap().insert_new_row(4, 1);

    let path = std::path::Path::new("./tests/result_files/issue_190.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_194() {
    let path = std::path::Path::new("./tests/test_files/issue_194.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    book.get_sheet_mut(0).unwrap().insert_new_column("D", 1);

    assert_eq!(
        book.get_sheet(0)
            .unwrap()
            .get_cell("B2")
            .unwrap()
            .get_formula(),
        "SUM(B1)"
    );
    assert_eq!(
        book.get_sheet(0)
            .unwrap()
            .get_cell("C2")
            .unwrap()
            .get_formula(),
        "SUM(C1)"
    );
    assert_eq!(
        book.get_sheet(0)
            .unwrap()
            .get_cell("E2")
            .unwrap()
            .get_formula(),
        "SUM(E1)"
    );

    let path = std::path::Path::new("./tests/result_files/issue_194.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_200() {
    let input_buffer = std::fs::read("./tests/test_files/issue_200.xlsx").unwrap();
    let cursor = std::io::Cursor::new(input_buffer);
    let workbook = reader::xlsx::read_reader(cursor, true).unwrap();
    let output_file = std::fs::File::create("./tests/result_files/issue_200.xlsx").unwrap();
    writer::xlsx::write_writer(&workbook, output_file).unwrap();
}

#[test]
fn issue_201() {
    let path = std::path::Path::new("./tests/test_files/issue_201.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let mut cell = book.get_sheet_mut(0).unwrap().get_cell_mut("B1");
    cell.set_formula_result_default("");

    let path = std::path::Path::new("./tests/result_files/issue_201.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_206() {
    let path = std::path::Path::new("./tests/test_files/issue_206.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_206.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_194_2() {
    let path = std::path::Path::new("./tests/test_files/issue_194_2.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_194_2.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_188_3() {
    let path = std::path::Path::new("./tests/test_files/issue_188_3.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_188_3.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_184() {
    let path = std::path::Path::new("./tests/test_files/issue_184.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let theme = book.get_theme();
    let color = book
        .get_sheet(0)
        .unwrap()
        .get_cell("A1")
        .unwrap()
        .get_style()
        .get_font()
        .unwrap()
        .get_color()
        .get_argb_with_theme(theme);
    assert_eq!(color, "A88570");
}

#[test]
fn issue_188_4() {
    let path = std::path::Path::new("./tests/test_files/issue_188_4.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    assert_eq!(
        book.get_sheet(0)
            .unwrap()
            .get_cell("H4")
            .unwrap()
            .get_formula(),
        "SUM(B4:G4)"
    );
    assert_eq!(
        book.get_sheet(0)
            .unwrap()
            .get_cell("H5")
            .unwrap()
            .get_formula(),
        "SUM(B5:G5)"
    );

    // remove
    book.get_sheet_mut(0).unwrap().remove_column("E", 1);

    assert_eq!(
        book.get_sheet(0)
            .unwrap()
            .get_cell("G4")
            .unwrap()
            .get_formula(),
        "SUM(B4:F4)"
    );
    assert_eq!(
        book.get_sheet(0)
            .unwrap()
            .get_cell("G5")
            .unwrap()
            .get_formula(),
        "SUM(B5:F5)"
    );

    let path = std::path::Path::new("./tests/result_files/issue_188_4.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_210() {
    let path = std::path::Path::new("./tests/test_files/issue_210.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let sheet = book.get_sheet(0).unwrap();
    for cell in sheet.get_cells() {
        if let Some(varA) = cell.get_style().get_alignment() {
            let horizontal = varA.get_horizontal().get_value_string();
            let vertical = varA.get_vertical().get_value_string();
            let rot = varA.get_text_rotation();
            let wrap = varA.get_wrap_text();
            // dbg!(vec![
            //    cell.get_coordinate().to_string(),
            //    horizontal.to_string(),
            //    vertical.to_string(),
            //    rot.to_string(),
            //    wrap.to_string()
            //]);
        }
    }
}

#[test]
fn issue_208() {
    let path = std::path::Path::new("./tests/test_files/issue_208.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let path = std::path::Path::new("./tests/result_files/issue_208.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_184_2() {
    let path = std::path::Path::new("./tests/test_files/issue_184_2.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let data = vec![
        ("A1", "FFFFFF"),
        ("A2", "F2F2F2"),
        ("A3", "D9D9D9"),
        ("A4", "BFBFBF"),
        ("A5", "A6A6A6"),
        ("A6", "808080"),
        ("B1", "000000"),
        ("B2", "7F7F7F"),
        ("B3", "595959"),
        ("B4", "404040"),
        ("B5", "262626"),
        ("B6", "0D0D0D"),
        ("C1", "E8E8E8"),
        ("C2", "D1D1D1"),
        ("C3", "AEAEAE"),
        ("C4", "747474"),
        ("C5", "3A3A3A"),
        ("C6", "171717"),
        ("D1", "0E2841"),
        ("D2", "DBEAF7"),
        ("D3", "A6CAEC"),
        ("D4", "4D95D9"),
        ("D5", "21609B"),
        ("D6", "163E64"),
        ("A8", "FFC00000"),
        ("B8", "FFFF0000"),
        ("C8", "FFFFC000"),
        ("D8", "FFFFFF00"),
    ];

    for (coordinate, result) in data {
        let color = book
            .get_sheet(0)
            .unwrap()
            .get_cell(coordinate)
            .unwrap()
            .get_style()
            .get_background_color()
            .unwrap()
            .get_argb_with_theme(book.get_theme());
        assert_eq!(color, result);
    }
}

#[test]
fn issue_215() {
    let path = std::path::Path::new("./tests/test_files/issue_215.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let path = std::path::Path::new("./tests/result_files/issue_215.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_216() {
    let path = std::path::Path::new("./tests/test_files/issue_216.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
}

#[test]
fn issue_217() {
    let path = std::path::Path::new("./tests/test_files/issue_217.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    book.get_sheet_mut(2)
        .unwrap()
        .set_state(SheetStateValues::Hidden);
    let path = std::path::Path::new("./tests/result_files/issue_217.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_218() {
    let mut out_book = new_file();
    let out_sheet = out_book.new_sheet("listDataTable").unwrap();

    let mut color = Color::default();
    color.set_argb_str("FF0000");
    let mut pattern_fill = PatternFill::default();
    pattern_fill.set_background_color(color);
    let mut fill = Fill::default();
    fill.set_pattern_fill(pattern_fill);
    let mut style = Style::default();
    style.set_fill(fill);

    let mut form = Formula::default();
    form.set_string_value("20");

    let mut cond = ConditionalFormattingRule::default();
    cond.set_type(ConditionalFormatValues::CellIs)
        .set_operator(ConditionalFormattingOperatorValues::GreaterThan)
        .set_priority(1)
        .set_style(style)
        .set_formula(form);

    let mut seq = SequenceOfReferences::default();
    seq.set_sqref("B2:B4");

    let mut new_cond = ConditionalFormatting::default();
    new_cond.set_sequence_of_references(seq);

    new_cond.set_conditional_collection(vec![cond]);
    out_sheet.set_conditional_formatting_collection(vec![new_cond]);

    let path = std::path::Path::new("./tests/result_files/issue_218.xlsx");
    let _unused = writer::xlsx::write(&out_book, path);
}

#[test]
fn issue_219() {
    let path = std::path::Path::new("./tests/test_files/issue_219.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_219.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_220() {
    let path = std::path::Path::new("./tests/test_files/issue_220.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    book.get_sheet_mut(0)
        .unwrap()
        .get_cell_mut("A1")
        .set_value("TEST1");

    book.get_sheet_mut(0)
        .unwrap()
        .get_cell_mut("B1")
        .set_value("TEST1");

    book.get_sheet_mut(0)
        .unwrap()
        .get_cell_mut("B2")
        .set_value("TEST1");

    book.get_sheet_mut(0)
        .unwrap()
        .get_cell_mut("A2")
        .set_value("TEST1");

    let path = std::path::Path::new("./tests/result_files/issue_220.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_222() {
    let path = std::path::Path::new("./tests/test_files/issue_222.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_222.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_224() {
    let mut book = new_file();
    let mut sheet = book.get_sheet_mut(0).unwrap();
    let mut num = NumberingFormat::default();
    num.set_format_code("[<1]0;0");
    sheet
        .get_cell_mut("A1")
        .get_style_mut()
        .set_numbering_format(num);
    sheet.get_cell_mut("A1").set_value_number(1.3);
    assert_eq!("1", sheet.get_formatted_value("A1"));
}

#[test]
fn issue_225() {
    let path = std::path::Path::new("./tests/test_files/issue_225.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_225.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_227() {
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = reader::xlsx::lazy_read(path).unwrap();
    book.read_sheet_by_name("Sheet2");
    let sheet = book.get_sheet_by_name("Sheet2").unwrap();
    assert_eq!("3", sheet.get_cell("B6").unwrap().get_value());
}

#[test]
fn issue_230() {
    let mut wb = new_file();
    let sheet = wb.get_sheet_mut(0).unwrap();
    sheet.get_cell_mut("A1").set_value("12");
    sheet
        .get_style_mut("A1")
        .get_number_format_mut()
        .set_format_code("#\\ #");
    // assert_eq!("1 2", sheet.get_formatted_value("A1"));
}

#[test]
fn issue_232() {
    let path = std::path::Path::new("./tests/test_files/issue_232.xlsx");
    let mut book = reader::xlsx::lazy_read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/issue_232.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_233() {
    let path = std::path::Path::new("./tests/test_files/issue_233.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    // book.get_sheet_mut(0).unwrap().cleanup();

    let path = std::path::Path::new("./tests/result_files/issue_233.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_244() {
    let path = std::path::Path::new("./tests/test_files/issue_244.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let sheet = book.get_sheet_mut(0).unwrap();

    let mut comment = Comment::default();
    comment.new_comment("B2");
    comment.set_text_string("TEST");

    let media_object = helper::binary::make_media_object("./images/sample1.png");
    comment
        .get_shape_mut()
        .get_fill_mut()
        .unwrap()
        .set_image(media_object);

    sheet.add_comments(comment);

    let path = std::path::Path::new("./tests/result_files/issue_244.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_246() {
    let path = std::path::Path::new("./tests/test_files/issue_246.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let sheet = book.get_sheet_mut(0).unwrap();

    sheet.copy_row_styling(3, 5, None, None);
    sheet.copy_row_styling(3, 6, Some(4), Some(6));

    sheet.copy_col_styling(10, 12, None, None);
    sheet.copy_col_styling(10, 13, Some(11), Some(15));

    let path = std::path::Path::new("./tests/result_files/issue_246.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_248() {
    let mut book = new_file();
    let mut sheet = book.get_sheet_mut(0).unwrap();
    sheet
        .get_sheet_views_mut()
        .get_sheet_view_list_mut()
        .get_mut(0)
        .unwrap()
        .set_show_grid_lines(false);

    let path = std::path::Path::new("./tests/result_files/issue_248.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}
