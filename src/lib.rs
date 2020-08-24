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

pub fn new_file()->structs::spreadsheet::Spreadsheet {
    let mut spreadsheet = structs::spreadsheet::Spreadsheet::default();
    let worksheet = spreadsheet.new_sheet();
    worksheet.set_title("Sheet1");
    worksheet.set_sheet_id("1");
    worksheet.set_active_cell("A1");
    spreadsheet
}