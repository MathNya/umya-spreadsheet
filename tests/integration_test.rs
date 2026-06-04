#![allow(warnings)]
#![allow(clippy::all)]

extern crate chrono;
extern crate umya_spreadsheet;
use std::{
    env,
    ffi::c_void,
    fs::File,
    io::{
        BufWriter,
        Read,
    },
    path::PathBuf,
    time::Instant,
};

use umya_spreadsheet::*;

use crate::helper::color;

#[cfg(target_os = "linux")]
fn current_process_memory_mb() -> Option<f64> {
    process_status_memory_mb("VmRSS:")
}

#[cfg(target_os = "linux")]
fn peak_process_memory_mb() -> Option<f64> {
    process_status_memory_mb("VmHWM:")
}

#[cfg(target_os = "linux")]
fn process_status_memory_mb(prefix: &str) -> Option<f64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    status.lines().find_map(|line| {
        let value = line.strip_prefix(prefix)?.trim();
        let kb = value.split_whitespace().next()?.parse::<f64>().ok()?;
        Some(kb / 1024.0)
    })
}

#[cfg(windows)]
fn current_process_memory_mb() -> Option<f64> {
    process_memory_mb().map(|(_, working_set)| working_set)
}

#[cfg(windows)]
fn peak_process_memory_mb() -> Option<f64> {
    process_memory_mb().map(|(peak_working_set, _)| peak_working_set)
}

#[cfg(windows)]
#[repr(C)]
struct ProcessMemoryCounters {
    cb:                              u32,
    page_fault_count:                u32,
    peak_working_set_size:           usize,
    working_set_size:                usize,
    quota_peak_paged_pool_usage:     usize,
    quota_paged_pool_usage:          usize,
    quota_peak_non_paged_pool_usage: usize,
    quota_non_paged_pool_usage:      usize,
    pagefile_usage:                  usize,
    peak_pagefile_usage:             usize,
}

#[cfg(windows)]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetCurrentProcess() -> *mut c_void;
}

#[cfg(windows)]
#[link(name = "psapi")]
unsafe extern "system" {
    fn GetProcessMemoryInfo(
        process: *mut c_void,
        counters: *mut ProcessMemoryCounters,
        size: u32,
    ) -> i32;
}

#[cfg(windows)]
fn process_memory_mb() -> Option<(f64, f64)> {
    let mut counters = ProcessMemoryCounters {
        cb:                              std::mem::size_of::<ProcessMemoryCounters>() as u32,
        page_fault_count:                0,
        peak_working_set_size:           0,
        working_set_size:                0,
        quota_peak_paged_pool_usage:     0,
        quota_paged_pool_usage:          0,
        quota_peak_non_paged_pool_usage: 0,
        quota_non_paged_pool_usage:      0,
        pagefile_usage:                  0,
        peak_pagefile_usage:             0,
    };
    let ok = unsafe {
        GetProcessMemoryInfo(
            GetCurrentProcess(),
            &mut counters,
            std::mem::size_of::<ProcessMemoryCounters>() as u32,
        )
    };
    if ok == 0 {
        return None;
    }
    Some((
        counters.peak_working_set_size as f64 / 1024.0 / 1024.0,
        counters.working_set_size as f64 / 1024.0 / 1024.0,
    ))
}

#[cfg(all(unix, not(target_os = "linux")))]
fn current_process_memory_mb() -> Option<f64> {
    let output = std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
        .ok()?;
    let kb = String::from_utf8(output.stdout)
        .ok()?
        .trim()
        .parse::<f64>()
        .ok()?;
    Some(kb / 1024.0)
}

#[cfg(all(unix, not(target_os = "linux")))]
fn peak_process_memory_mb() -> Option<f64> {
    None
}

#[cfg(not(any(unix, windows)))]
fn current_process_memory_mb() -> Option<f64> {
    None
}

#[cfg(not(any(unix, windows)))]
fn peak_process_memory_mb() -> Option<f64> {
    None
}

fn print_large_file_checkpoint(label: &str, start: Instant) {
    let elapsed = start.elapsed();
    let memory = current_process_memory_mb()
        .map(|value| format!("{value:.1}MB"))
        .unwrap_or_else(|| "unknown".to_string());
    let peak_memory = peak_process_memory_mb()
        .map(|value| format!("{value:.1}MB"))
        .unwrap_or_else(|| "unknown".to_string());
    println!(
        "{label}:{}.{:03}sec memory:{memory} peak:{peak_memory}",
        elapsed.as_secs(),
        elapsed.subsec_millis()
    );
}

fn profile_input_path() -> PathBuf {
    env::var_os("UMYA_PROFILE_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("./tests/test_files/aaa_large_string.xlsx"))
}

#[test]
fn read_and_wite() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    read_and_wite_method(&mut book);

    book.sheet_by_name("Sheet1")
        .unwrap()
        .image("M17")
        .unwrap()
        .download_image("./tests/result_files/bbb.png");

    book.sheet_by_name_mut("Sheet1")
        .unwrap()
        .image_mut("M17")
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
    // let _ns = book.sheet_by_name_mut("Sheet1").unwrap();
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
            let cell = ns.cell_mut((c, r));
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
#[ignore]
fn profile_large_string_memory() {
    let total = Instant::now();
    print_large_file_checkpoint("start", total);

    let path = profile_input_path();
    println!("profile_file:{}", path.display());
    let mut book = reader::xlsx::lazy_read(&path).unwrap();
    print_large_file_checkpoint("lazy_read", total);

    if env::var_os("UMYA_PROFILE_DESERIALIZE").is_some() {
        let sheet_name = env::var("UMYA_PROFILE_SHEET").unwrap_or_else(|_| "Sheet1".to_string());
        book.read_sheet_by_name(&sheet_name);
        print_large_file_checkpoint("deserialize_sheet", total);
    }

    if env::var_os("UMYA_PROFILE_EDIT").is_some() {
        let ns = book.new_sheet("profile sheet").unwrap();
        for r in 1..5000 {
            for c in 1..30 {
                let cell = ns.cell_mut((c, r));
                let _unused = cell.set_value_string(format!("r{}c{}", r, c));
            }
        }
        print_large_file_checkpoint("edit", total);
    }

    let output_path = env::var_os("UMYA_PROFILE_OUTPUT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("./tests/result_files/profile_large_string.xlsx"));
    match env::var("UMYA_PROFILE_WRITER").as_deref() {
        Ok("buffer") => {
            let file = File::create(&output_path).unwrap();
            writer::xlsx::write_writer(&book, BufWriter::new(file)).unwrap();
        }
        Ok("light") => writer::xlsx::write_light(&book, &output_path).unwrap(),
        _ => writer::xlsx::write(&book, &output_path).unwrap(),
    }
    print_large_file_checkpoint("write", total);
}

#[test]
#[ignore]
fn profile_large_string_stream_memory() {
    let total = Instant::now();
    print_large_file_checkpoint("start", total);

    let path = profile_input_path();
    let sheet_name = env::var("UMYA_PROFILE_SHEET").unwrap_or_else(|_| "Sheet1".to_string());
    println!("profile_file:{}", path.display());
    println!("profile_sheet:{sheet_name}");

    let mut cell_count = 0usize;
    reader::xlsx::read_sheet_by_name_stream(&path, &sheet_name, |_| {
        cell_count += 1;
        if cell_count % 500_000 == 0 {
            print_large_file_checkpoint(&format!("stream_cells_{cell_count}"), total);
        }
    })
    .unwrap();

    println!("stream_cell_count:{cell_count}");
    print_large_file_checkpoint("stream_done", total);
}

#[test]
fn lazy_read_and_wite_no_edit() {
    // reader
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let book = reader::xlsx::lazy_read(path).unwrap();

    let cells = book.lazy_read_sheet_cells(0).unwrap();
    assert_eq!("英語", cells.cell_value((5, 12)).value());
    assert_eq!("英語", cells.cell_value("E12").value());

    // writer
    let path = std::path::Path::new("./tests/result_files/bbb_lazy_no_edit.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

fn read_and_wite_method(book: &mut Workbook) {
    let _unused = book.sheet_mut(0).unwrap().cell_mut("A1").set_value("TEST1");
    let a1_value = book.sheet(0).unwrap().value("A1");
    assert_eq!("TEST1", a1_value);
    let _unused = book.sheet_mut(0).unwrap().remove_cell((1, 1));
    let a1 = book.sheet(0).unwrap().cell("A1");
    assert_eq!(a1, None);
    let _unused = book.sheet_mut(0).unwrap().remove_cell((1, 2));
    let a2_value = book.sheet(0).unwrap().value("A2");
    assert_eq!(a2_value, "");
    let b5_value = book.sheet(0).unwrap().value("B5");
    assert_eq!(" ", b5_value);

    assert_eq!("1.0000", book.sheet(0).unwrap().formatted_value((2, 20)));
    assert_eq!("$3,333.0000", book.sheet(0).unwrap().formatted_value("B21"));
    assert_eq!("$ 333.00", book.sheet(0).unwrap().formatted_value("B22"));
    assert_eq!("2020年3月", book.sheet(0).unwrap().formatted_value("B23"));
    assert_eq!("2:33 pm", book.sheet(0).unwrap().formatted_value("B24"));
    assert_eq!("5.00%", book.sheet(0).unwrap().formatted_value("B25"));
    assert_eq!("1/2", book.sheet(0).unwrap().formatted_value("B26"));

    assert_eq!(
        "12/15/2020 14:01",
        book.sheet(0).unwrap().formatted_value("B27")
    );

    assert_eq!("444", book.sheet(0).unwrap().formatted_value("B28"));
    assert_eq!("14-Dec-20", book.sheet(0).unwrap().formatted_value("B29"));
    assert_eq!(
        "2020年10月1日",
        book.sheet(0).unwrap().formatted_value("B30")
    );
    assert_eq!("1.2345", book.sheet(0).unwrap().formatted_value("B31"));
    assert_eq!("1.2", book.sheet(0).unwrap().formatted_value("B32"));
    assert_eq!(
        "12,345,675,544.00",
        book.sheet(0).unwrap().formatted_value("B33")
    );
    assert_eq!("1.235", book.sheet(0).unwrap().formatted_value("B34"));
    assert_eq!("1", book.sheet(0).unwrap().formatted_value("B35"));
    assert_eq!("", book.sheet(0).unwrap().formatted_value("B36"));
    assert_eq!(
        "123456789012345678",
        book.sheet(0).unwrap().formatted_value("B37")
    );

    let _unused = book
        .sheet_by_name_mut("Sheet1")
        .unwrap()
        .cell_mut("A1")
        .set_value("49046881.119999997");

    let _unused = book
        .sheet_by_name_mut("Sheet1")
        .unwrap()
        .style_mut("A1")
        .number_format_mut()
        .set_format_code(NumberingFormat::FORMAT_NUMBER_COMMA_SEPARATED1);

    let value = book
        .sheet_by_name_mut("Sheet1")
        .unwrap()
        .formatted_value("A1");
    assert_eq!("49,046,881.12", &value);

    let fg = Color::default().set_argb(Color::COLOR_BLACK).to_owned();
    let fill = PatternFill::default().set_foreground_color(fg).to_owned();
    book.sheet_by_name_mut("Sheet5")
        .unwrap()
        .row_dimension_mut(5u32)
        .style_mut()
        .fill_mut()
        .set_pattern_fill(fill);
    let font_color = Color::default().set_argb(Color::COLOR_WHITE).to_owned();
    book.sheet_by_name_mut("Sheet5")
        .unwrap()
        .row_dimension_mut(5u32)
        .style_mut()
        .font_mut()
        .set_color(font_color);

    let _unused = book
        .sheet_by_name_mut("Sheet7")
        .unwrap()
        .cell_mut("A1")
        .style_mut()
        .font_mut()
        .set_name("Arial");

    book.sheet_by_name_mut("Sheet1")
        .unwrap()
        .row_dimension_mut(3)
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

    let sheet = book.sheet_by_name_mut("Sheet1").unwrap();
    let cell = sheet.cell_mut((1, 2));
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
        .sheet_mut(0)
        .unwrap()
        .cell_mut((1, 1))
        .set_value("TEST1");
    let a1_value = book.sheet(0).unwrap().cell((1, 1)).unwrap().value();
    assert_eq!("TEST1", a1_value);

    // copy sheet
    let mut clone_sheet = book.sheet(0).unwrap().clone();
    clone_sheet.set_name("New Sheet");
    let _unused = book.add_sheet(clone_sheet);

    // remove sheet
    let mut clone_sheet = book.sheet(0).unwrap().clone();
    clone_sheet.set_name("DeletedSheet");
    let _unused = book.add_sheet(clone_sheet);
    book.sheet_by_name("DeletedSheet").unwrap();
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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

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
    book.sheet_by_name_mut("Sheet7").unwrap().add_chart(chart);

    // Add Image
    let _unused = book.new_sheet("Sheet Image");
    let mut marker = drawing::spreadsheet::MarkerType::default();
    marker.set_coordinate("B3");
    let mut image = Image::default();
    image.new_image("./images/sample1.png", marker);
    book.sheet_by_name_mut("Sheet Image")
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
    let cell = sheet.cell_mut("A2");
    let _unused = cell.set_value("test");

    // set style by range
    let mut style = Style::default();
    style.set_background_color(BG_COLOR);
    sheet.set_style_by_range("A3:A4", &style);

    let path = std::path::Path::new("./tests/result_files/bbb_new_sheet_value.xlsx");
    let _unused = writer::xlsx::write(&book, path);

    let mut book = reader::xlsx::lazy_read(path).unwrap();
    let a2_value = book
        .sheet_by_name_mut(TEST_SHEET)
        .unwrap()
        .cell("A2")
        .unwrap()
        .value();
    assert_eq!("test", a2_value);

    {
        let a3_bg = book
            .sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .style_mut("A3")
            .fill_mut()
            .pattern_fill_mut()
            .foreground_color_mut()
            .argb_str();

        assert_eq!(a3_bg, "FF333333");
    }

    {
        let a4_bg = book
            .sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .style_mut("A4")
            .fill_mut()
            .pattern_fill_mut()
            .foreground_color_mut()
            .argb_str();

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
    let cell = sheet.cell_mut("A2");
    let _unused = cell.set_value("test");

    // set style by range
    let mut style = Style::default();
    style.set_background_color(BG_COLOR);
    sheet.set_style_by_range("A3:A4", &style);

    let path = std::path::Path::new("./tests/result_files/three_digit_hex_color_with_hash.xlsx");
    let _unused = writer::xlsx::write(&book, path);

    let mut book = reader::xlsx::lazy_read(path).unwrap();
    let a2_value = book
        .sheet_by_name_mut(TEST_SHEET)
        .unwrap()
        .cell("A2")
        .unwrap()
        .value();
    assert_eq!("test", a2_value);

    {
        let a3_bg = book
            .sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .style_mut("A3")
            .fill_mut()
            .pattern_fill_mut()
            .foreground_color_mut()
            .argb_str();

        assert_eq!(a3_bg, "FFFF00DD");
    }

    {
        let a4_bg = book
            .sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .style_mut("A4")
            .fill_mut()
            .pattern_fill_mut()
            .foreground_color_mut()
            .argb_str();

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
    let cell = sheet.cell_mut("A2");
    let _unused = cell.set_value("test");

    // set style by range
    let mut style = Style::default();
    style.set_background_color(BG_COLOR);
    sheet.set_style_by_range("A3:A4", &style);

    let path = std::path::Path::new("./tests/result_files/three_digit_hex_color_without_hash.xlsx");
    let _unused = writer::xlsx::write(&book, path);

    let mut book = reader::xlsx::lazy_read(path).unwrap();
    let a2_value = book
        .sheet_by_name_mut(TEST_SHEET)
        .unwrap()
        .cell("A2")
        .unwrap()
        .value();
    assert_eq!("test", a2_value);

    {
        let a3_bg = book
            .sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .style_mut("A3")
            .fill_mut()
            .pattern_fill_mut()
            .foreground_color_mut()
            .argb_str();

        assert_eq!(a3_bg, "FFAACCEE");
    }

    {
        let a4_bg = book
            .sheet_by_name_mut(TEST_SHEET)
            .unwrap()
            .style_mut("A4")
            .fill_mut()
            .pattern_fill_mut()
            .foreground_color_mut()
            .argb_str();

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
    book.sheet_by_name_mut("Sheet2")
        .unwrap()
        .cell_mut("A1")
        .set_value("TEST1");
    let a1_value = book
        .sheet_by_name("Sheet2")
        .unwrap()
        .cell("A1")
        .unwrap()
        .value();
    assert_eq!("TEST1", a1_value);

    book.sheet_by_name_mut("Sheet2")
        .unwrap()
        .cell_mut((2, 2))
        .set_value_number(1);
    let a1_value = book
        .sheet_by_name("Sheet2")
        .unwrap()
        .cell((2, 2))
        .unwrap()
        .value();
    assert_eq!("1", a1_value);

    book.sheet_by_name_mut("Sheet2")
        .unwrap()
        .cell_mut((2, 2))
        .set_value_number(1);
    let a1_value = book
        .sheet_by_name("Sheet2")
        .unwrap()
        .cell((2, 2))
        .unwrap()
        .value();
    assert_eq!("1", a1_value);

    book.sheet_by_name_mut("Sheet2")
        .unwrap()
        .cell_mut((3, 3))
        .set_value_bool(true);
    let a1_value = book
        .sheet_by_name("Sheet2")
        .unwrap()
        .cell((3, 3))
        .unwrap()
        .value();
    assert_eq!("TRUE", a1_value);

    book.sheet_by_name_mut("Sheet2")
        .unwrap()
        .cell_mut((3, 3))
        .set_value_bool(true);
    let a1_value = book
        .sheet_by_name("Sheet2")
        .unwrap()
        .cell((3, 3))
        .unwrap()
        .value();
    assert_eq!("TRUE", a1_value);

    // add bottom border.
    book.sheet_by_name_mut("Sheet2")
        .unwrap()
        .style_mut("A1")
        .borders_mut()
        .bottom_mut()
        .set_border_style(Border::BORDER_MEDIUM);
    book.sheet_by_name_mut("Sheet2")
        .unwrap()
        .style_mut((3, 2))
        .borders_mut()
        .left_mut()
        .set_border_style(Border::BORDER_THIN);

    // change font color.
    book.sheet_by_name_mut("Sheet2")
        .unwrap()
        .style_mut("A1")
        .font_mut()
        .color_mut()
        .set_argb_str("00FF0000");

    // change background color.
    book.sheet_by_name_mut("Sheet2")
        .unwrap()
        .style_mut("A1")
        .set_background_color(Color::COLOR_BLUE_STR);

    book.sheet_by_name_mut("Sheet2")
        .unwrap()
        .style_mut("A2")
        .set_background_color_with_pattern(
            Color::COLOR_BLUE_STR,
            Color::COLOR_RED_STR,
            PatternValues::DarkGrid,
        );

    let worksheet = book.sheet_by_name_mut("Sheet3").unwrap();
    worksheet.column_dimension_mut("A").set_auto_width(true);

    worksheet.cell_mut("E1").set_value("テスト");
    worksheet.cell_mut("E2").set_value("うみゃーねっと");
    worksheet.cell_mut("E3").set_value("案案案案");
    worksheet.column_dimension_mut("E").set_auto_width(true);

    worksheet.cell_mut("F1").set_value("AAAAAAAAAAAAAAAAAA");
    worksheet.cell_mut("F2").set_value("BBBBBBBBBBB");
    worksheet
        .cell_mut("F4")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.column_dimension_mut("F").set_auto_width(true);

    worksheet.cell_mut("G1").set_value("AAAAAAAAAAAAAAAAAA");
    worksheet.cell_mut("G2").set_value("BBBBBBBBBBB");
    worksheet
        .cell_mut("G3")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.column_dimension_mut("G").set_width(60f64);

    worksheet.cell_mut("D1").set_value("テスト");
    worksheet.cell_mut("D2").set_value("うみゃーねっと");
    worksheet.cell_mut("D3").set_value("案案案案");
    worksheet.column_dimension_mut("D").set_auto_width(true);

    worksheet.cell_mut("H1").set_value("テスト");
    worksheet
        .cell_mut("H2")
        .set_value("うみゃーねっと\nうみゃーねっと")
        .style_mut()
        .alignment_mut()
        .set_wrap_text(true);
    worksheet.cell_mut("H3").set_value("案案案案");
    worksheet.column_dimension_mut("H").set_auto_width(true);

    worksheet.cell_mut("I1").set_value("テスト");
    worksheet
        .cell_mut("I2")
        .set_value("うみゃーねっと")
        .style_mut()
        .font_mut()
        .font_size_mut()
        .set_val(20f64);
    worksheet.cell_mut("I3").set_value("案案案案");
    worksheet.column_dimension_mut("I").set_auto_width(true);

    worksheet
        .cell_mut("J2")
        .set_value("うみゃーねっと")
        .style_mut()
        .font_mut()
        .font_size_mut()
        .set_val(5f64);
    worksheet.column_dimension_mut("J").set_auto_width(true);

    worksheet
        .cell_mut("K4")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.column_dimension_mut("K").set_auto_width(true);

    worksheet
        .cell_mut("L4")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.column_dimension_mut("L").set_auto_width(true);

    worksheet
        .cell_mut("M4")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.column_dimension_mut("M").set_auto_width(true);

    worksheet
        .cell_mut("N1")
        .set_value("CCCCCCCCCCCCCCCCCCCCCCCCCC");
    worksheet.column_dimension_mut("N").set_auto_width(true);

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
    sheet.cell_mut("A1").set_value(" TEST");
    sheet.cell_mut("B1").set_value("あいうえお");
    sheet.cell_mut("C1").set_value("漢字");
    sheet.cell_mut("E1").set_value("1");
    // ---
    sheet.cell_mut("A2").set_value("TEST ");
    sheet.cell_mut("B2").set_value("あいうえお");
    sheet.cell_mut("C2").set_value("漢字");
    // ---
    sheet.cell_mut("A3").set_value(" TEST ");
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
    let sheet = book.active_sheet();

    assert_eq!(sheet.cell("A1").unwrap().value(), "TEST");
    assert_eq!(sheet.cell("A2").unwrap().value(), " TEST ");

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

    let sheet = book.sheet_by_name_mut("Sheet1").unwrap();
    let cell = sheet.cell_mut("A1");
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
    book.sheet_by_name_mut(sheet_name)
        .unwrap()
        .move_range(range, row, column);

    // Checking to ensure cells that move into another cell overwrites it
    let range_2 = "A14:A14";
    book.sheet_by_name_mut(sheet_name)
        .unwrap()
        .move_range(range, row, column);
    book.sheet_by_name_mut(sheet_name)
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
    let img = book.sheet_by_name("Sheet1").unwrap().image("M17").unwrap();
    // dbg!(img.one_cell_anchor().is_some());
    // dbg!(img.two_cell_anchor().is_some());
    assert_eq!(img.image_name(), "image1.png");
    assert_eq!(img.coordinate(), "M17");
    assert_eq!(img.col(), 12);
    assert_eq!(img.row(), 16);

    let path = std::path::Path::new("./tests/result_files/issue_129.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn wb_with_shared_strings() {
    let path = std::path::Path::new("./tests/test_files/wb_with_shared_strings.xlsx");
    let book = reader::xlsx::read(path).unwrap();
    let sheet = book.sheet_by_name("Sheet To Read From").unwrap();

    assert_eq!(sheet.cell("A2").unwrap().value(), "11");
    assert_eq!(sheet.cell("A3").unwrap().value(), "22");
    assert_eq!(sheet.cell("A4").unwrap().value(), "ABCdef");
    assert_eq!(sheet.cell("A5").unwrap().value(), "ABCdef");
    assert_eq!(sheet.cell("A6").unwrap().value(), "ABCdef");

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

    let mut sheet = book.sheet_mut(2).unwrap();
    sheet
        .sheet_protection_mut()
        .set_password("password")
        .set_sheet(true);

    let path = std::path::Path::new("./tests/result_files/sheet_lock.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn workbooklock_test() {
    let path = std::path::Path::new("./tests/test_files/book_lock.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    book.workbook_protection_mut()
        .set_workbook_password("password");

    let path = std::path::Path::new("./tests/result_files/book_lock.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_147() {
    let path = std::path::Path::new("./tests/test_files/issue_147.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let from_sheet = book.sheet(0).unwrap();
    let source_cell = from_sheet.cell((2, 3)).unwrap();
    let mut tarcell = source_cell.clone();

    let mut to_sheet = book.sheet_mut(1).unwrap();
    to_sheet.set_cell(tarcell);

    let path = std::path::Path::new("./tests/result_files/issue_147.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn html_to_richtext_test() {
    let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let mut sheet = book.sheet_by_name_mut("Sheet1").unwrap();

    let html = r##"<font color="red">test</font><br><font class="test" color="#48D1CC">TE<b>S</b>T<br/>TEST</font>"##;
    let richtext = helper::html::html_to_richtext(html).unwrap();

    sheet.cell_mut("G16").set_rich_text(richtext);
    sheet
        .cell_mut("G16")
        .style_mut()
        .alignment_mut()
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
    let mut sheet = book.sheet_mut(0).unwrap();
    sheet.cell_mut("A1").set_value_number(value);
    sheet.style_mut("A1").set_numbering_format(numbering_format);

    let result = sheet.formatted_value("A1");
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
    let mut sheet = book.sheet_by_name_mut("LOV").unwrap();
    sheet.remove_row(2, 1);
    for (key, row) in sheet.row_dimensions_to_hashmap() {
        assert_eq!(key, &1);
        assert_eq!(row.row_num(), 1);
        assert_eq!(row.height(), 35.0);
    }
    let path = std::path::Path::new("./tests/result_files/issue_181.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_181_2() {
    let path = std::path::Path::new("./tests/test_files/issue_181_2.xlsx");

    let mut book = reader::xlsx::read(path).unwrap();
    let shee1: &mut Worksheet = book.sheet_mut(0).unwrap();
    let new_row_index = 4;
    shee1.insert_new_row(new_row_index, 5);
    shee1.cell_mut((1, new_row_index)).set_value("123");

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
        book.sheet(0).unwrap().cell("A1").unwrap().is_formula(),
        true
    );
}

#[test]
fn issue_187() {
    let path = std::path::Path::new("./tests/test_files/issue_187.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let mut sheet = book.sheet_mut(0).unwrap();

    let mut cell = sheet.cell("C4").unwrap().clone();
    cell.set_coordinate("B5");
    sheet.set_cell(cell);

    let mut cell = sheet.cell("C4").unwrap().clone();
    cell.set_coordinate("D6");
    sheet.set_cell(cell);

    let mut cell = sheet.cell("C4").unwrap().clone();
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

    let cell = book.sheet(0).unwrap().cell("A1").unwrap();

    let color = cell.style().font().unwrap().color().argb_str();

    assert_eq!("FFFF0000", color);
}

#[test]
fn issue_190() {
    let path = std::path::Path::new("./tests/test_files/issue_190.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    // remove
    book.sheet_mut(0).unwrap().remove_column("E", 1);
    book.sheet_mut(0).unwrap().remove_row(4, 1);

    // insert
    book.sheet_mut(1).unwrap().insert_new_column("E", 1);
    book.sheet_mut(1).unwrap().insert_new_row(4, 1);

    let path = std::path::Path::new("./tests/result_files/issue_190.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_194() {
    let path = std::path::Path::new("./tests/test_files/issue_194.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    book.sheet_mut(0).unwrap().insert_new_column("D", 1);

    assert_eq!(
        book.sheet(0).unwrap().cell("B2").unwrap().formula(),
        "SUM(B1)"
    );
    assert_eq!(
        book.sheet(0).unwrap().cell("C2").unwrap().formula(),
        "SUM(C1)"
    );
    assert_eq!(
        book.sheet(0).unwrap().cell("E2").unwrap().formula(),
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
    let mut cell = book.sheet_mut(0).unwrap().cell_mut("B1");
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
    let theme = book.theme();
    let color = book
        .sheet(0)
        .unwrap()
        .cell("A1")
        .unwrap()
        .style()
        .font()
        .unwrap()
        .color()
        .argb_with_theme(theme);
    assert_eq!(color, "A88570");
}

#[test]
fn issue_188_4() {
    let path = std::path::Path::new("./tests/test_files/issue_188_4.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    assert_eq!(
        book.sheet(0).unwrap().cell("H4").unwrap().formula(),
        "SUM(B4:G4)"
    );
    assert_eq!(
        book.sheet(0).unwrap().cell("H5").unwrap().formula(),
        "SUM(B5:G5)"
    );

    // remove
    book.sheet_mut(0).unwrap().remove_column("E", 1);

    assert_eq!(
        book.sheet(0).unwrap().cell("G4").unwrap().formula(),
        "SUM(B4:F4)"
    );
    assert_eq!(
        book.sheet(0).unwrap().cell("G5").unwrap().formula(),
        "SUM(B5:F5)"
    );

    let path = std::path::Path::new("./tests/result_files/issue_188_4.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_210() {
    let path = std::path::Path::new("./tests/test_files/issue_210.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let sheet = book.sheet(0).unwrap();
    for cell in sheet.cells() {
        if let Some(varA) = cell.style().alignment() {
            let horizontal = varA.horizontal().value_string();
            let vertical = varA.vertical().value_string();
            let rot = varA.text_rotation();
            let wrap = varA.wrap_text();
            // dbg!(vec![
            //    cell.coordinate().to_string(),
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
            .sheet(0)
            .unwrap()
            .cell(coordinate)
            .unwrap()
            .style()
            .background_color()
            .unwrap()
            .argb_with_theme(book.theme());
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
    book.sheet_mut(2)
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

    book.sheet_mut(0).unwrap().cell_mut("A1").set_value("TEST1");

    book.sheet_mut(0).unwrap().cell_mut("B1").set_value("TEST1");

    book.sheet_mut(0).unwrap().cell_mut("B2").set_value("TEST1");

    book.sheet_mut(0).unwrap().cell_mut("A2").set_value("TEST1");

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
    let mut sheet = book.sheet_mut(0).unwrap();
    let mut num = NumberingFormat::default();
    num.set_format_code("[<1]0;0");
    sheet.cell_mut("A1").style_mut().set_numbering_format(num);
    sheet.cell_mut("A1").set_value_number(1.3);
    assert_eq!("1", sheet.formatted_value("A1"));
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
    let sheet = book.sheet_by_name("Sheet2").unwrap();
    assert_eq!("3", sheet.cell("B6").unwrap().value());
}

#[test]
fn issue_230() {
    let mut wb = new_file();
    let sheet = wb.sheet_mut(0).unwrap();
    sheet.cell_mut("A1").set_value("12");
    sheet
        .style_mut("A1")
        .number_format_mut()
        .set_format_code("#\\ #");
    // assert_eq!("1 2", sheet.formatted_value("A1"));
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

    // book.sheet_mut(&0).unwrap().cleanup();

    let path = std::path::Path::new("./tests/result_files/issue_233.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_244() {
    let path = std::path::Path::new("./tests/test_files/issue_244.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();
    let sheet = book.sheet_mut(0).unwrap();

    let mut comment = Comment::default();
    comment.new_comment("B2");
    comment.set_text_string("TEST");

    let media_object = helper::binary::make_media_object("./images/sample1.png");
    comment
        .shape_mut()
        .fill_mut()
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
    let sheet = book.sheet_mut(0).unwrap();

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
    let mut sheet = book.sheet_mut(0).unwrap();
    sheet
        .sheet_views_mut()
        .sheet_view_list_mut()
        .get_mut(0)
        .unwrap()
        .set_show_grid_lines(false);

    let path = std::path::Path::new("./tests/result_files/issue_248.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

#[test]
fn issue_265() {
    let mut book = new_file();
    let mut sheet = book.sheet_mut(0).unwrap();
    sheet
        .cell_mut("A1")
        .style_mut()
        .set_background_color(umya_spreadsheet::Color::COLOR_BLACK_STR);
    sheet.cell_mut("A1").set_value("COLOR_BLACK");
    sheet
        .cell_mut("A2")
        .style_mut()
        .set_background_color(umya_spreadsheet::Color::COLOR_WHITE_STR);
    sheet.cell_mut("A2").set_value("COLOR_WHITE");
    sheet
        .cell_mut("A3")
        .style_mut()
        .set_background_color(umya_spreadsheet::Color::COLOR_RED_STR);
    sheet.cell_mut("A3").set_value("COLOR_RED");
    sheet
        .cell_mut("A4")
        .style_mut()
        .set_background_color(umya_spreadsheet::Color::COLOR_DARKRED_STR);
    sheet.cell_mut("A4").set_value("COLOR_DARKRED");
    sheet
        .cell_mut("A5")
        .style_mut()
        .set_background_color(umya_spreadsheet::Color::COLOR_BLUE_STR);
    sheet.cell_mut("A5").set_value("COLOR_BLUE");
    sheet
        .cell_mut("A6")
        .style_mut()
        .set_background_color(umya_spreadsheet::Color::COLOR_DARKBLUE_STR);
    sheet.cell_mut("A6").set_value("COLOR_DARKBLUE");
    sheet
        .cell_mut("A7")
        .style_mut()
        .set_background_color(umya_spreadsheet::Color::COLOR_GREEN_STR);
    sheet.cell_mut("A7").set_value("COLOR_GREEN");
    sheet
        .cell_mut("A8")
        .style_mut()
        .set_background_color(umya_spreadsheet::Color::COLOR_DARKGREEN_STR);
    sheet.cell_mut("A8").set_value("COLOR_DARKGREEN");
    sheet
        .cell_mut("A9")
        .style_mut()
        .set_background_color(umya_spreadsheet::Color::COLOR_YELLOW_STR);
    sheet.cell_mut("A9").set_value("COLOR_YELLOW");
    sheet
        .cell_mut("A10")
        .style_mut()
        .set_background_color(umya_spreadsheet::Color::COLOR_DARKYELLOW_STR);
    sheet.cell_mut("A10").set_value("COLOR_DARKYELLOW");

    let path = std::path::Path::new("./tests/result_files/issue_265.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn issue_268() {
    let source_path = std::path::Path::new("./tests/test_files/issue_268.xlsx");
    let source_book = reader::xlsx::read(source_path).unwrap();
    let mut target_book = new_file();
    let sheet_count = source_book.sheet_count();
    for i in 0..sheet_count {
        let sheet = source_book.sheet(i).unwrap();
        let copied_sheet = sheet.clone();
        let _ = target_book.add_sheet(copied_sheet);
    }

    let output_path = std::path::Path::new("./tests/result_files/issue_268.xlsx");
    writer::xlsx::write(&target_book, output_path).unwrap();

    println!("All sheets copied successfully!");
}

#[test]
fn issue_279() {
    let mut book = new_file();
    let mut sheet = book.sheet_mut(0).unwrap();
    sheet.cell_mut("A1").set_value("NaN");

    let path = std::path::Path::new("./tests/result_files/issue_279.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn issue_281() {
    let path = std::path::Path::new("./tests/test_files/issue_281.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/r_issue_281.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn issue_284() {
    let mut book = new_file();
    let mut sheet = book.sheet_mut(0).unwrap();

    let mut hyperlink1 = Hyperlink::default();
    hyperlink1.set_location(false);
    hyperlink1.set_url("http:test.com/1");
    sheet.cell_mut("A1").set_hyperlink(hyperlink1);

    let mut hyperlink2 = Hyperlink::default();
    hyperlink2.set_location(false);
    hyperlink2.set_url("http:test.com/2");
    sheet.cell_mut("A2").set_hyperlink(hyperlink2);

    let mut hyperlink3 = Hyperlink::default();
    hyperlink3.set_location(false);
    hyperlink3.set_url("http:test.com/3");
    sheet.cell_mut("A3").set_hyperlink(hyperlink3);

    let mut hyperlink4 = Hyperlink::default();
    hyperlink4.set_location(false);
    hyperlink4.set_url("http:test.com/4");
    sheet.cell_mut("A4").set_hyperlink(hyperlink4);

    let mut hyperlink11 = Hyperlink::default();
    hyperlink11.set_location(false);
    hyperlink11.set_url("http:test.com/11");
    sheet.cell_mut("B1").set_hyperlink(hyperlink11);

    let mut hyperlink12 = Hyperlink::default();
    hyperlink12.set_location(false);
    hyperlink12.set_url("http:test.com/12");
    sheet.cell_mut("B2").set_hyperlink(hyperlink12);

    let mut hyperlink13 = Hyperlink::default();
    hyperlink13.set_location(false);
    hyperlink13.set_url("http:test.com/13");
    sheet.cell_mut("B3").set_hyperlink(hyperlink13);

    let mut hyperlink14 = Hyperlink::default();
    hyperlink14.set_location(false);
    hyperlink14.set_url("http:test.com/14");
    sheet.cell_mut("B4").set_hyperlink(hyperlink14);

    let path = std::path::Path::new("./tests/result_files/issue_284.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn issue_285() {
    let path = std::path::Path::new("./tests/test_files/issue_285.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
    let mut sheet = book.sheet_mut(0).unwrap();
    sheet.cell_mut("B7").set_value("test");

    let path = std::path::Path::new("./tests/result_files/r_issue_285.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn issue_286() {
    let path = std::path::Path::new("./tests/test_files/issue_286.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/r_issue_286.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn issue_288() {
    let path = std::path::Path::new("./tests/test_files/issue_288.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/r_issue_288.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn issue_291() {
    let path = std::path::Path::new("./tests/test_files/issue_291.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    let path = std::path::Path::new("./tests/result_files/r_issue_291.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn issue_293() {
    let mut book = new_file();
    let mut sheet = book.sheet_mut(0).unwrap();

    sheet
        .sheet_views_mut()
        .sheet_view_list_mut()
        .get_mut(0)
        .unwrap()
        .set_zoom_scale(110)
        .set_zoom_scale_normal(110);

    let path = std::path::Path::new("./tests/result_files/r_issue_293.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn issue_296() {
    let path = std::path::Path::new("./tests/test_files/issue_296.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    let sheet1 = book.sheet_by_name("Sheet1").unwrap();
    if let Some(auto_filter) = sheet1.auto_filter() {
        println!(
            "auto_filter for from_sheet {} is {:?}",
            sheet1.name(),
            auto_filter
        );
    }

    let path = std::path::Path::new("./tests/result_files/r_issue_296.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
}

#[test]
fn issue_297() {
    let path = std::path::Path::new("./tests/test_files/issue_297.xlsx");
    let mut from_book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    let from_sheet = from_book.sheet_by_name("Sheet1").unwrap();
    let mut to_book = umya_spreadsheet::new_file_empty_worksheet();
    let to_sheet = from_sheet.clone();
    let _ = to_book.add_sheet(to_sheet).expect("error when add_sheet");

    let path = std::path::Path::new("./tests/result_files/r_issue_297.xlsx");
    let _ = umya_spreadsheet::writer::xlsx::write(&to_book, path);
}

#[test]
fn issue_298() {
    // reader
    let path = std::path::Path::new("./tests/test_files/issue_298.xlsx");
    // let book = reader::xlsx::lazy_read(path).unwrap();
    let book = reader::xlsx::read(path).unwrap();

    // let cells = book.lazy_read_sheet_cells(0).unwrap();

    // writer
    let path = std::path::Path::new("./tests/result_files/r_issue_298.xlsx");
    let _unused = writer::xlsx::write(&book, path);
}

fn workbook_to_xlsx_bytes(book: &Workbook) -> Vec<u8> {
    let mut output = Vec::new();
    writer::xlsx::write_writer(book, &mut output).unwrap();
    output
}

fn zip_entry_to_string(xlsx: &[u8], entry_name: &str) -> String {
    let cursor = std::io::Cursor::new(xlsx);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();
    let mut entry = archive.by_name(entry_name).unwrap();
    let mut xml = String::new();
    entry.read_to_string(&mut xml).unwrap();
    xml
}

fn cell_fragment(sheet_xml: &str, coordinate: &str) -> String {
    let start = sheet_xml
        .find(&format!("<c r=\"{coordinate}\""))
        .expect("cell not found in sheet xml");
    let fragment = &sheet_xml[start..];

    if let Some(end) = fragment.find("</c>") {
        return fragment[..(end + 4)].to_string();
    }

    let end = fragment
        .find("/>")
        .expect("cell node should be either empty or explicitly closed");
    fragment[..(end + 2)].to_string()
}

fn shared_formula_signatures(
    sheet_xml: &str,
) -> Vec<(String, Option<String>, Option<String>, Option<String>)> {
    let mut reader = quick_xml::Reader::from_str(sheet_xml);
    reader.config_mut().trim_text(true);

    let mut signatures = Vec::new();
    let mut buf = Vec::new();
    let mut current_cell = String::new();
    let mut shared_si: Option<String> = None;
    let mut shared_ref: Option<String> = None;
    let mut shared_text: Option<String> = None;
    let mut in_shared_formula = false;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(quick_xml::events::Event::Start(ref e)) => match e.name().into_inner() {
                b"c" => {
                    current_cell.clear();
                    for attribute in e.attributes().flatten() {
                        if attribute.key.into_inner() == b"r" {
                            current_cell = String::from_utf8_lossy(attribute.value.as_ref()).into();
                        }
                    }
                }
                b"f" => {
                    let mut formula_type: Option<String> = None;
                    shared_si = None;
                    shared_ref = None;
                    shared_text = None;
                    for attribute in e.attributes().flatten() {
                        let value = String::from_utf8_lossy(attribute.value.as_ref()).into_owned();
                        match attribute.key.into_inner() {
                            b"t" => formula_type = Some(value),
                            b"si" => shared_si = Some(value),
                            b"ref" => shared_ref = Some(value),
                            _ => {}
                        }
                    }
                    in_shared_formula = formula_type.as_deref() == Some("shared");
                }
                _ => {}
            },
            Ok(quick_xml::events::Event::Empty(ref e)) => {
                if e.name().into_inner() == b"f" {
                    let mut formula_type: Option<String> = None;
                    let mut si: Option<String> = None;
                    let mut reference: Option<String> = None;
                    for attribute in e.attributes().flatten() {
                        let value = String::from_utf8_lossy(attribute.value.as_ref()).into_owned();
                        match attribute.key.into_inner() {
                            b"t" => formula_type = Some(value),
                            b"si" => si = Some(value),
                            b"ref" => reference = Some(value),
                            _ => {}
                        }
                    }
                    if formula_type.as_deref() == Some("shared") {
                        signatures.push((current_cell.clone(), si, reference, None));
                    }
                }
            }
            Ok(quick_xml::events::Event::Text(e)) => {
                if in_shared_formula {
                    shared_text = Some(e.unescape().unwrap().into_owned());
                }
            }
            Ok(quick_xml::events::Event::End(ref e)) => {
                if e.name().into_inner() == b"f" {
                    if in_shared_formula {
                        signatures.push((
                            current_cell.clone(),
                            shared_si.clone(),
                            shared_ref.clone(),
                            shared_text.clone(),
                        ));
                    }
                    in_shared_formula = false;
                    shared_si = None;
                    shared_ref = None;
                    shared_text = None;
                }
            }
            Ok(quick_xml::events::Event::Eof) => break,
            Err(e) => panic!("failed to parse sheet xml: {e}"),
            _ => {}
        }
        buf.clear();
    }

    signatures
}

#[test]
fn formula_cached_values_are_written_with_typed_xml_and_roundtrip() {
    let mut book = new_file();
    let sheet = book.sheet_mut(0).unwrap();

    sheet
        .cell_mut("A1")
        .set_formula("1+1")
        .set_formula_result_default("2");
    sheet
        .cell_mut("A2")
        .set_formula("1=1")
        .set_formula_result_default("TRUE");
    sheet
        .cell_mut("A3")
        .set_formula("NA()")
        .set_formula_result_default("#N/A");
    sheet
        .cell_mut("A4")
        .set_formula("T(\"ok\")")
        .set_formula_result_default("ok");
    sheet
        .cell_mut("A5")
        .set_formula("1/0")
        .set_formula_result_default("");

    let xlsx = workbook_to_xlsx_bytes(&book);
    let sheet_xml = zip_entry_to_string(&xlsx, "xl/worksheets/sheet1.xml");

    let a1 = cell_fragment(&sheet_xml, "A1");
    assert!(a1.contains("<f>1+1</f>"));
    assert!(a1.contains("<v>2</v>"));
    assert!(!a1.contains("t=\"str\""));
    assert!(!a1.contains("t=\"b\""));
    assert!(!a1.contains("t=\"e\""));

    let a2 = cell_fragment(&sheet_xml, "A2");
    assert!(a2.contains("t=\"b\""));
    assert!(a2.contains("<v>1</v>"));

    let a3 = cell_fragment(&sheet_xml, "A3");
    assert!(a3.contains("t=\"e\""));
    assert!(a3.contains("<v>#N/A</v>"));

    let a4 = cell_fragment(&sheet_xml, "A4");
    assert!(a4.contains("t=\"str\""));
    assert!(a4.contains("<v>ok</v>"));

    let a5 = cell_fragment(&sheet_xml, "A5");
    assert!(a5.contains("<v/>"));

    let roundtrip = reader::xlsx::read_reader(std::io::Cursor::new(xlsx), true).unwrap();
    let roundtrip_sheet = roundtrip.sheet(0).unwrap();

    let a1_cell = roundtrip_sheet.cell("A1").unwrap();
    assert_eq!(a1_cell.formula(), "1+1");
    assert_eq!(a1_cell.raw_value(), &CellRawValue::Numeric(2f64));

    let a2_cell = roundtrip_sheet.cell("A2").unwrap();
    assert_eq!(a2_cell.formula(), "1=1");
    assert_eq!(a2_cell.raw_value(), &CellRawValue::Bool(true));

    let a3_cell = roundtrip_sheet.cell("A3").unwrap();
    assert_eq!(a3_cell.formula(), "NA()");
    assert_eq!(a3_cell.raw_value(), &CellRawValue::Error(CellErrorType::NA));

    let a4_cell = roundtrip_sheet.cell("A4").unwrap();
    assert_eq!(a4_cell.formula(), "T(\"ok\")");
    assert!(matches!(
        a4_cell.raw_value(),
        CellRawValue::String(value) if value.as_ref() == "ok"
    ));
}

#[test]
fn write_keeps_shared_formula_metadata_stable() {
    let source_path = std::path::Path::new("./tests/test_files/issue_194.xlsx");
    let source_bytes = std::fs::read(source_path).unwrap();
    let source_sheet_xml = zip_entry_to_string(&source_bytes, "xl/worksheets/sheet1.xml");
    let source_shared = shared_formula_signatures(&source_sheet_xml);
    assert!(!source_shared.is_empty());

    let workbook = reader::xlsx::read_reader(std::io::Cursor::new(source_bytes), true).unwrap();
    let rewritten = workbook_to_xlsx_bytes(&workbook);
    let rewritten_sheet_xml = zip_entry_to_string(&rewritten, "xl/worksheets/sheet1.xml");
    let rewritten_shared = shared_formula_signatures(&rewritten_sheet_xml);

    assert_eq!(rewritten_shared, source_shared);
}

#[test]
fn typed_formula_result_helpers_write_expected_types() {
    let mut book = new_file();
    let sheet = book.sheet_mut(0).unwrap();

    sheet
        .cell_mut("B1")
        .set_formula("10/2")
        .set_formula_result_number(5.0);
    sheet
        .cell_mut("B2")
        .set_formula("1=2")
        .set_formula_result_bool(false);
    sheet
        .cell_mut("B3")
        .set_formula("NA()")
        .set_formula_result_error(CellErrorType::NA);
    sheet
        .cell_mut("B4")
        .set_formula("T(\"value\")")
        .set_formula_result_string("value");
    sheet
        .cell_mut("B5")
        .set_formula("1/0")
        .set_formula_result_blank();

    let xlsx = workbook_to_xlsx_bytes(&book);
    let sheet_xml = zip_entry_to_string(&xlsx, "xl/worksheets/sheet1.xml");

    let b1 = cell_fragment(&sheet_xml, "B1");
    assert!(!b1.contains("t=\"str\""));
    assert!(b1.contains("<v>5</v>"));

    let b2 = cell_fragment(&sheet_xml, "B2");
    assert!(b2.contains("t=\"b\""));
    assert!(b2.contains("<v>0</v>"));

    let b3 = cell_fragment(&sheet_xml, "B3");
    assert!(b3.contains("t=\"e\""));
    assert!(b3.contains("<v>#N/A</v>"));

    let b4 = cell_fragment(&sheet_xml, "B4");
    assert!(b4.contains("t=\"str\""));
    assert!(b4.contains("<v>value</v>"));

    let b5 = cell_fragment(&sheet_xml, "B5");
    assert!(b5.contains("<v/>"));
}

// --- Panic safety: backslash zip paths (Windows-generated XLSX)
// ---------------

#[test]
fn backslash_paths_tdf131575() {
    let path = std::path::Path::new("./tests/test_files/tdf131575.xlsx");
    let _book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
}

#[test]
fn backslash_paths_tdf76115() {
    let path = std::path::Path::new("./tests/test_files/tdf76115.xlsx");
    let _book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
}

#[test]
fn backslash_paths_49609() {
    let path = std::path::Path::new("./tests/test_files/49609.xlsx");
    let _book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
}

// --- Panic safety: missing optional styles.xml
// --------------------------------

#[test]
fn missing_styles_56278() {
    let path = std::path::Path::new("./tests/test_files/56278.xlsx");
    let _book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
}

#[test]
fn missing_styles_tdf121887() {
    let path = std::path::Path::new("./tests/test_files/tdf121887.xlsx");
    let _book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
}

#[test]
fn missing_styles_59021() {
    let path = std::path::Path::new("./tests/test_files/59021.xlsx");
    let _book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
}

// --- Panic safety: arithmetic overflow / unwrap on None
// -----------------------

#[test]
fn overflow_functions_excel_2010() {
    let path = std::path::Path::new("./tests/test_files/functions-excel-2010.xlsx");
    let _book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
}

#[test]
fn overflow_formula_eval_test_data() {
    let path = std::path::Path::new("./tests/test_files/FormulaEvalTestData_Copy.xlsx");
    let _book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
}

#[test]
fn missing_attr_64450() {
    let path = std::path::Path::new("./tests/test_files/64450.xlsx");
    let _book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
}

#[test]
fn test_remove_row() {
    let path = std::path::Path::new("./tests/test_files/RemoveRow.xlsx");
    let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
    let sheet = book.sheet_mut(0).unwrap();
    sheet.remove_row(3, 1);
    let out = std::path::Path::new("./tests/result_files/RemoveRow.xlsx");
    umya_spreadsheet::writer::xlsx::write(&book, out).unwrap();
}

#[test]
fn alignment_indent_roundtrip() {
    // Author a workbook with a non-zero `alignment.indent`, write it out,
    // read it back, and assert the indent attribute survived the round-trip.
    // Prior to this fix the `indent` attribute was silently dropped on write
    // because the `Alignment` struct had no field for it.
    let mut book = umya_spreadsheet::new_file();
    let sheet = book.sheet_mut(0).unwrap();
    sheet
        .cell_mut("A1")
        .set_value("indented")
        .style_mut()
        .alignment_mut()
        .set_indent(3);

    let out = std::path::Path::new("./tests/result_files/alignment_indent_roundtrip.xlsx");
    umya_spreadsheet::writer::xlsx::write(&book, out).unwrap();

    let book2 = umya_spreadsheet::reader::xlsx::read(out).unwrap();
    let sheet2 = book2.sheet(0).unwrap();
    let cell = sheet2.cell("A1").unwrap();
    let alignment = cell.style().alignment().expect("alignment present");
    assert_eq!(alignment.indent(), 3, "indent must survive round-trip");
}

#[test]
fn alignment_indent_preserved_from_external_xlsx() {
    // Reproduces the original bug at runtime without depending on the new
    // `set_indent` / `indent()` API. The fixture xlsx
    // (`tests/test_files/alignment_indent_input.xlsx`) was authored by
    // openpyxl and contains `<alignment horizontal="left" indent="3"/>` in
    // its `xl/styles.xml`. We read it through umya, write it back out, then
    // inspect the resulting `xl/styles.xml` directly via the `zip` crate
    // (already a transitive dep) and grep for the indent attribute.
    //
    // Before this fix: the reader silently ignored the `indent` attribute
    // (no field to land it in) and the writer never emitted one, so the
    // output's `<alignment>` element had no `indent="..."` and this test
    // FAILED at runtime with `indent attribute missing` in the panic.
    //
    // After this fix: the field round-trips and the output xlsx contains
    // `indent="3"` on the alignment element, exactly as authored.
    use std::io::Read;
    let in_path = std::path::Path::new("./tests/test_files/alignment_indent_input.xlsx");
    let book = umya_spreadsheet::reader::xlsx::read(in_path).unwrap();

    let out_path = std::path::Path::new("./tests/result_files/alignment_indent_input.xlsx");
    umya_spreadsheet::writer::xlsx::write(&book, out_path).unwrap();

    let mut zip =
        zip::ZipArchive::new(std::fs::File::open(out_path).unwrap()).unwrap();
    let mut styles_xml = String::new();
    zip.by_name("xl/styles.xml")
        .unwrap()
        .read_to_string(&mut styles_xml)
        .unwrap();
    assert!(
        styles_xml.contains("indent=\"3\""),
        "indent attribute missing from round-tripped styles.xml; got:\n{styles_xml}"
    );
}
