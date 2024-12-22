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
//! //  Delays the loading of the worksheet until it is needed.//! //  When loading a file with a large amount of data, response improvement can be expected.
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
//! let _unused = book.new_sheet("Sheet2");
//! ```
//! ### Copy worksheet
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//!
//! let mut clone_sheet = book.get_sheet(0).unwrap().clone();
//! clone_sheet.set_name("New Sheet");
//! let _unused = book.add_sheet(clone_sheet);
//! ```
//! ### Change value
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//! let _unused =  book.new_sheet("Sheet2");
//!
//! // change value
//! book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_mut("A1").set_value("TEST1");
//! book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_mut("B2").set_value_from_i32(1);
//! book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_mut("C3").set_value_from_bool(true);
//! // or
//! book.get_sheet_mut(1).unwrap().get_cell_mut((1, 1)).set_value("TEST1");
//! book.get_sheet_mut(1).unwrap().get_cell_mut((2, 2)).set_value_from_i32(1));
//! book.get_sheet_mut(1).unwrap().get_cell_mut((3, 3)).set_value_from_bool(true));
//! ```
//! ### Read value
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//! let _unused = book.new_sheet("Sheet2");
//! book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_mut("A1").set_value("TEST1");
//!
//! // read value
//! let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_value("A1");
//! // or
//! let a1_value = book.get_sheet(1).unwrap().get_value((1, 1));
//! // or formatted value
//! let a1_value = book.get_sheet(1).unwrap().get_formatted_value("A1");
//! assert_eq!("TEST1", a1_value); // TEST1
//! ```
//! ### Change style
//! more example is [**here**](Style).
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//! let _unused = book.new_sheet("Sheet2");
//!
//! // add bottom border
//! book.get_sheet_by_name_mut("Sheet2")
//!     .unwrap()
//!     .get_style_mut("A1")
//!     .get_borders_mut()
//!     .get_bottom_mut()
//!     .set_border_style(Border::BORDER_MEDIUM);
//! // or
//! book.get_sheet_mut(1)
//!     .unwrap()
//!     .get_style_mut((1, 1))
//!     .get_borders_mut()
//!     .get_bottom_mut()
//!     .set_border_style(Border::BORDER_MEDIUM);
//! ```
//! ### Insert or Remove Rows(or Columns)
//! ![Result Image](https://github.com/MathNya/umya-spreadsheet/raw/master/images/sample2.png)
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//!
//! // insert rows
//! book.insert_new_row("Sheet1", &2, &3);
//!
//! // insert columns
//! book.insert_new_column("Sheet1", "B", &3);
//! // or
//! book.insert_new_column_by_index("Sheet1", &2, &3);
//!
//! // remove rows
//! book.remove_row("Sheet1", &6, &2);
//!
//! // remove columns
//! book.remove_column("Sheet1", "F", &2);
//! // or
//! book.remove_column_by_index("Sheet1", &6, &2);
//! ```
//! ### Writer
//! ```rust
//! use umya_spreadsheet::*;
//! let mut book = new_file();
//! let _unused = book.new_sheet("Sheet2");
//!
//! // writer
//! let path = std::path::Path::new("C:/spread_test_data/ccc.xlsx");
//! let _unused = writer::xlsx::write(&book, path);
//! ```

#![deny(
    explicit_outlives_requirements,
    let_underscore_drop,
    meta_variable_misuse,
    non_ascii_idents,
    non_local_definitions,
    redundant_imports,
    redundant_lifetimes,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unit_bindings,
    unsafe_code,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    variant_size_differences
)]
#![allow(dead_code)]

#![deny(clippy::correctness)]
#![warn(
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
    clippy::suspicious
)]
#![allow(clippy::module_name_repetitions)]

extern crate chrono;
extern crate fancy_regex;
#[cfg(feature = "image")] extern crate image;
extern crate md5;
extern crate quick_xml;
extern crate thousands;
extern crate zip;

extern crate aes;
extern crate base64;
extern crate byteorder;
extern crate cbc;
extern crate cfb;
extern crate hmac;
extern crate html_parser;
extern crate rand;
extern crate sha2;

pub mod helper;
pub mod reader;
pub mod structs;
pub mod traits;
pub mod writer;

pub use self::structs::*;

/// Creates a new spreadsheet with default settings.
///
/// Returns a new `Spreadsheet` instance initialized with:
/// - Default theme
/// - Default stylesheet
/// - One worksheet named "Sheet1"
/// - Active cell set to "A1"
/// - Sheet view configured with workbook view ID 0
///
/// # Panics
///
/// Panics if unable to create a new worksheet named "Sheet1". This should never
/// happen with default settings since it's the first worksheet in a new
/// spreadsheet.
#[must_use]
pub fn new_file() -> Spreadsheet {
    let mut spreadsheet = Spreadsheet::default();
    spreadsheet.set_theme(drawing::Theme::get_default_value());
    spreadsheet.set_stylesheet_default_value();
    let worksheet = spreadsheet.new_sheet("Sheet1").unwrap();
    worksheet.set_active_cell("A1");
    let mut sheet_view = SheetView::default();
    sheet_view.set_workbook_view_id(0);
    let mut sheet_views = SheetViews::default();
    sheet_views.add_sheet_view_list_mut(sheet_view);
    worksheet.set_sheets_views(sheet_views);
    spreadsheet.set_active_sheet(0);
    spreadsheet
}

/// Creates a new empty spreadsheet without any worksheets.
///
/// This function initializes a new spreadsheet with default theme and
/// stylesheet settings. At least one worksheet must be added before generating
/// a valid file.
///
/// # Returns
/// A new `Spreadsheet` instance with default configuration but no worksheets.
///
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file_empty_worksheet();
/// ```
#[must_use]
pub fn new_file_empty_worksheet() -> Spreadsheet {
    let mut spreadsheet = Spreadsheet::default();
    spreadsheet.set_theme(drawing::Theme::get_default_value());
    spreadsheet.set_stylesheet_default_value();
    spreadsheet
}
