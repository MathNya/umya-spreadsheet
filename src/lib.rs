//! # umya-spreadsheet
//!
//! A pure `rust` library for reading and writing Microsoft Excel (xlsx) files.
//!
//! ## Example
//!
//! ![Result Image](https://github.com/MathNya/umya-spreadsheet/raw/master/images/sample1.png)
//!
//! ### Reader or New File
//!
//! ```rust
//! use umya_spreadsheet::*;
//!
//! // Reader
//! let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
//! let mut book = reader::xlsx::read(path).unwrap();
//!
//! // Lazy Reader
//! // Delays loading of worksheets until they are needed.
//! // Can improve performance when loading large files.
//! let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
//! let mut book = reader::xlsx::lazy_read(path).unwrap();
//!
//! // New file
//! let mut book = new_file();
//! ```
//!
//! ### New Worksheet
//!
//! ```rust
//! use umya_spreadsheet::*;
//!
//! let mut book = new_file();
//!
//! // New worksheet
//! let _unused = book.new_sheet("Sheet2");
//! ```
//!
//! ### Copy Worksheet
//!
//! ```rust
//! use umya_spreadsheet::*;
//!
//! let mut book = new_file();
//!
//! let mut clone_sheet = book.get_sheet(0).unwrap().clone();
//! clone_sheet.set_name("New Sheet");
//! let _unused = book.add_sheet(clone_sheet);
//! ```
//!
//! ### Change Value
//!
//! ```rust
//! use umya_spreadsheet::*;
//!
//! let mut book = new_file();
//! let _unused = book.new_sheet("Sheet2");
//!
//! // Change value using string cell address
//! book.get_sheet_by_name_mut("Sheet2")
//!     .unwrap()
//!     .get_cell_mut("A1")
//!     .set_value("TEST1");
//! book.get_sheet_by_name_mut("Sheet2")
//!     .unwrap()
//!     .get_cell_mut("B2")
//!     .set_value_from_i32(1);
//! book.get_sheet_by_name_mut("Sheet2")
//!     .unwrap()
//!     .get_cell_mut("C3")
//!     .set_value_from_bool(true);
//!
//! // Change value using tuple cell address
//! book.get_sheet_mut(1)
//!     .unwrap()
//!     .get_cell_mut((1, 1))
//!     .set_value("TEST1");
//! book.get_sheet_mut(1)
//!     .unwrap()
//!     .get_cell_mut((2, 2))
//!     .set_value_from_i32(1);
//! book.get_sheet_mut(1)
//!     .unwrap()
//!     .get_cell_mut((3, 3))
//!     .set_value_from_bool(true);
//! ```
//!
//! ### Read Value
//!
//! ```rust
//! use umya_spreadsheet::*;
//!
//! let mut book = new_file();
//! let _unused = book.new_sheet("Sheet2");
//! book.get_sheet_by_name_mut("Sheet2")
//!     .unwrap()
//!     .get_cell_mut("A1")
//!     .set_value("TEST1");
//!
//! // Read value by string cell address
//! let a1_value = book.get_sheet_by_name("Sheet2").unwrap().get_value("A1");
//!
//! // Read value by tuple cell address
//! let a1_value = book.get_sheet(1).unwrap().get_value((1, 1));
//!
//! // Read formatted value by string cell address
//! let a1_value = book.get_sheet(1).unwrap().get_formatted_value("A1");
//!
//! assert_eq!("TEST1", a1_value);
//! ```
//!
//! ### Change Style
//!
//! More examples can be found in the [Style](crate::structs::style) module.
//!
//! ```rust
//! use umya_spreadsheet::*;
//!
//! let mut book = new_file();
//! let _unused = book.new_sheet("Sheet2");
//!
//! // Add a bottom border using string cell address
//! book.get_sheet_by_name_mut("Sheet2")
//!     .unwrap()
//!     .get_style_mut("A1")
//!     .get_borders_mut()
//!     .get_bottom_mut()
//!     .set_border_style(Border::BORDER_MEDIUM);
//!
//! // Add a bottom border using tuple cell address
//! book.get_sheet_mut(1)
//!     .unwrap()
//!     .get_style_mut((1, 1))
//!     .get_borders_mut()
//!     .get_bottom_mut()
//!     .set_border_style(Border::BORDER_MEDIUM);
//! ```
//!
//! ### Insert or Remove Rows/Columns
//!
//! ![Result Image](https://github.com/MathNya/umya-spreadsheet/raw/master/images/sample2.png)
//!
//! ```rust
//! use umya_spreadsheet::*;
//!
//! let mut book = new_file();
//!
//! // Insert rows
//! book.insert_new_row("Sheet1", &2, &3);
//!
//! // Insert columns by column name
//! book.insert_new_column("Sheet1", "B", &3);
//!
//! // Insert columns by index
//! book.insert_new_column_by_index("Sheet1", &2, &3);
//!
//! // Remove rows
//! book.remove_row("Sheet1", &6, &2);
//!
//! // Remove columns by column name
//! book.remove_column("Sheet1", "F", &2);
//!
//! // Remove columns by index
//! book.remove_column_by_index("Sheet1", &6, &2);
//! ```
//!
//! ### Writer
//!
//! ```rust
//! use umya_spreadsheet::*;
//!
//! let mut book = new_file();
//! let _unused = book.new_sheet("Sheet2");
//!
//! // Write to a file
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
#![allow(dead_code, unused_macros)]
#![deny(clippy::correctness, clippy::trivially_copy_pass_by_ref)]
#![warn(
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::cargo,
    clippy::suspicious
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::struct_field_names
)]

extern crate chrono;
extern crate fancy_regex;
extern crate imagesize;
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
mod version;
pub mod writer;

#[allow(unused_imports)]
pub use version::*;

pub use self::structs::*;

/// Creates a new workbook with default settings.
///
/// Returns a new `Workbook` instance initialized with:
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
pub fn new_file() -> Workbook {
    let mut wb = Workbook::default();
    wb.set_theme(drawing::Theme::default_value());
    wb.set_stylesheet_default_value();
    let worksheet = wb.new_sheet("Sheet1").unwrap();
    worksheet.set_active_cell("A1");
    let mut sheet_view = SheetView::default();
    sheet_view.set_workbook_view_id(0);
    let mut sheet_views = SheetViews::default();
    sheet_views.add_sheet_view_list_mut(sheet_view);
    worksheet.set_sheets_views(sheet_views);
    wb.set_active_sheet(0);
    wb
}

/// Creates a new empty workbook without any worksheets.
///
/// This function initializes a new workbook with default theme and
/// stylesheet settings. At least one worksheet must be added before generating
/// a valid file.
///
/// # Returns
/// A new `Workbook` instance with default configuration but no worksheets.
///
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file_empty_worksheet();
/// ```
#[must_use]
pub fn new_file_empty_worksheet() -> Workbook {
    let mut wb = Workbook::default();
    wb.set_theme(drawing::Theme::default_value());
    wb.set_stylesheet_default_value();
    wb
}
