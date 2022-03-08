//! ## Example
//! ![Result Image](https://github.com/MathNya/umya-spreadsheet/raw/master/images/sample1.png)
//! ### Reader or New File
//! ```rust
//! use umya_spreadsheet::*;
//!
//! // reader
//! let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
//! let mut book = reader::xlsx::read(path).unwrap();
//! // or
//! // lazy reader
//! //  Delays the loading of the worksheet until it is needed.
//! //  When loading a file with a large amount of data, response improvement can be expected.
//! let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
//! let mut book = reader::xlsx::lazy_read(path).unwrap();
//! // or
//! // new file
//! let mut book = new_file();
//! ```
//! ### New worksheet
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//!
//! // new worksheet
//! let _ = book.new_sheet("Sheet2");
//! ```
//! ### Copy worksheet
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//!
//! let mut clone_sheet = book.get_sheet(0).unwrap().clone();
//! clone_sheet.set_title("New Sheet");
//! let _ = book.add_sheet(clone_sheet);
//! ```
//! ### Change value
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//! let _ = book.new_sheet("Sheet2");
//!
//! // change value
//! book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_mut("A1").set_value("TEST1");
//! book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_mut("B2").set_value_from_i32(1);
//! book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_mut("C3").set_value_from_bool(true);
//! // or
//! book.get_sheet_mut(1).get_cell_by_column_and_row_mut(1, 1).set_value("TEST1");
//! book.get_sheet_mut(1).get_cell_by_column_and_row_mut(2, 2).set_value_from_i32(1);
//! book.get_sheet_mut(1).get_cell_by_column_and_row_mut(3, 3).set_value_from_bool(true);
//! ```
//! ### Read value
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//! let _ = book.new_sheet("Sheet2");
//! book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_mut("A1").set_value("TEST1");
//!
//! // read value
//! let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_value("A1");
//! // or
//! let a1_value = book.get_sheet(1).unwrap().get_value_by_column_and_row(1, 1);
//! // or formatted value
//! let a1_value = book.get_sheet(1).unwrap().get_formatted_value("A1");
//! assert_eq!("TEST1", a1_value);  // TEST1
//! ```
//! ### Change style
//! more example is [**here**](Style).
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//! let _ = book.new_sheet("Sheet2");
//!
//! // add bottom border
//! book.get_sheet_by_name_mut("Sheet2").unwrap()
//! .get_style_mut("A1")
//! .get_borders_mut()
//! .get_bottom_mut()
//! .set_border_style(Border::BORDER_MEDIUM);
//! // or
//! book.get_sheet_mut(1)
//! .get_style_by_column_and_row_mut(1, 1)
//! .get_borders_mut()
//! .get_bottom_mut()
//! .set_border_style(Border::BORDER_MEDIUM);
//! ```
//! ### Insert or Remove Rows(or Columns)
//! ![Result Image](https://github.com/MathNya/umya-spreadsheet/raw/master/images/sample2.png)
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//!
//! // insert rows
//! book.insert_new_row("Sheet1", 2, 3);
//!
//! // insert columns
//! book.insert_new_colmun("Sheet1", "B", 3);
//! // or
//! book.insert_new_colmun_by_index("Sheet1", 2, 3);
//!
//! // remove rows
//! book.remove_row("Sheet1", 6, 2);
//!
//! // remove columns
//! book.remove_colmun("Sheet1", "F", 2);
//! // or
//! book.remove_colmun_by_index("Sheet1", 6, 2);
//! ```
//! ### Writer
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//! let _ = book.new_sheet("Sheet2");
//!
//! // writer
//! let path = std::path::Path::new("C:/spread_test_data/ccc.xlsx");
//! let _ = writer::xlsx::write(&book, path);
//! ```
//! ## Supported graph types
//! * AreaChart
//! * Area3DChart
//! * BarChart
//! * Bar3DChart
//! * BubbleChart
//! * DoughnutChart
//! * LineChart
//! * Line3DChart
//! * OfPieChart
//! * PieChart
//! * RadarChart
//! * ScatterChart
//!
//! Other types will be supported sequentially.
//!
//! ## Add Chart
//! ![Result Image](https://github.com/MathNya/umya-spreadsheet/raw/master/images/sample3.png)
//! ```rust
//! extern crate umya_spreadsheet;
//!
//! let mut book = umya_spreadsheet::new_file();
//!
//! // add chart
//! let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
//! let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
//! from_marker.set_coordinate("C1");
//! to_marker.set_coordinate("D11");
//! let area_chart_series_list = vec![
//!     "Sheet1!$A$1:$A$10",
//!     "Sheet1!$B$1:$B$10",
//! ];
//! let mut chart = umya_spreadsheet::structs::Chart::default();
//! chart.new_chart(
//!     umya_spreadsheet::structs::ChartType::LineChart,
//!     from_marker,
//!     to_marker,
//!     area_chart_series_list,
//! );
//! book.get_sheet_by_name_mut("Sheet1").unwrap().get_worksheet_drawing_mut().add_chart_collection(chart);
//! ```

extern crate chrono;
extern crate md5;
extern crate onig;
extern crate quick_xml;
extern crate regex;
extern crate thousands;
extern crate zip;

#[macro_use]
extern crate lazy_static;

pub mod helper;
pub mod reader;
pub mod structs;
pub mod writer;

pub use self::structs::*;

/// create new spreadsheet file.
/// # Arguments
/// # Return value
/// * Spreadsheet structs object.
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file();
/// ```
pub fn new_file() -> structs::Spreadsheet {
    let mut spreadsheet = structs::Spreadsheet::default();
    spreadsheet.set_theme(Theme::get_defalut_value());
    spreadsheet.set_stylesheet_defalut_value();
    let worksheet = spreadsheet.new_sheet("Sheet1").unwrap();
    worksheet.set_active_cell("A1");
    spreadsheet
}
