extern crate quick_xml;
extern crate tempdir;
extern crate walkdir;
extern crate zip;
extern crate regex;
extern crate md5;

#[macro_use]
extern crate lazy_static;

pub mod structs;
pub mod writer;
pub mod reader;
pub mod helper;

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
    let worksheet = spreadsheet.new_sheet("Sheet1").unwrap();
    worksheet.set_active_cell("A1");
    spreadsheet
}
