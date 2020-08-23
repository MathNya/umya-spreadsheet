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

#[test]
fn test_read_and_wite() {
    // reader
    let path = std::path::Path::new("C:/spread_test_data/aaa.xlsx");
    let mut book = reader::xlsx::read(path).unwrap();

    let _ = book.get_sheet_mut(0).get_cell_mut("A1").set_value("TEST1");
    let a1_value = book.get_sheet(0).get_cell("A1").unwrap().get_value();
    // TEST1
    dbg!(&a1_value);

    // writer
    let path = std::path::Path::new("C:/spread_test_data/bbb.xlsx");
    let _ = writer::xlsx::write(&book, path);
}