extern crate quick_xml;
extern crate tempdir;
extern crate walkdir;
extern crate zip;
extern crate regex;
extern crate md5;
extern crate thousands;
extern crate onig;
extern crate chrono;

#[macro_use]
extern crate lazy_static;

pub mod structs;
pub mod writer;
pub mod reader;
pub mod helper;

pub use self::structs::spreadsheet::*;
pub use self::structs::worksheet::*;
pub use self::structs::properties::*;
pub use self::structs::cell::*;
pub use self::structs::cells::*;
pub use self::structs::hyperlink::*;
pub use self::structs::row_dimension::*;
pub use self::structs::column_dimension::*;
pub use self::structs::shadow::*;
pub use self::structs::color::*;
pub use self::structs::page_setup::*;
pub use self::structs::page_margins::*;
pub use self::structs::header_footer::*;
pub use self::structs::header_footer_drawing::*;
pub use self::structs::sheet_view::*;
pub use self::structs::auto_filter::*;
pub use self::structs::column::*;
pub use self::structs::security::*;
pub use self::structs::calculation::*;
pub use self::structs::style::*;
pub use self::structs::font::*;
pub use self::structs::fill::*;
pub use self::structs::borders::*;
pub use self::structs::border::*;
pub use self::structs::alignment::*;
pub use self::structs::number_format::*;
pub use self::structs::conditional::*;
pub use self::structs::protection::*;
pub use self::structs::rich_text::*;
pub use self::structs::text_element::*;
pub use self::structs::picture::*;
pub use self::structs::theme::*;
pub use self::structs::cell_style::*;
pub use self::structs::defined_name::*;
pub use self::structs::comment::*;
pub use self::structs::styles::*;
pub use self::structs::coordinate::*;
pub use self::structs::range::*;
pub use self::structs::conditional_set::*;
pub use self::structs::address::*;
pub use self::structs::anchor::*;
pub use self::structs::drawing::*;

/// create new spreadsheet file.
/// # Arguments
/// # Return value
/// * Spreadsheet structs object.
/// # Examples
/// ```
/// let mut book = umya_spreadsheet::new_file();
/// ```
pub fn new_file()->structs::spreadsheet::Spreadsheet {
    let mut spreadsheet = structs::spreadsheet::Spreadsheet::default();
    spreadsheet.set_theme(Theme::get_defalut_value());
    let worksheet = spreadsheet.new_sheet("Sheet1").unwrap();
    worksheet.set_active_cell("A1");
    spreadsheet
}
