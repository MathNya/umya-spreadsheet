# umya-spreadsheet
[![Crates.io](https://img.shields.io/crates/v/umya-spreadsheet)](https://crates.io/crates/umya-spreadsheet)

## Description
**umya-spreadsheet** is a library written in pure Rust and read and write xlsx file.

## Caution
This is BETA version.
- It contains many bugs.
- Often changes are not backward compatible.

## Example
```rust
extern crate umya_spreadsheet;

// reader
let path = std::path::Path::new("C:/spread_test_data/aaa.xlsx");
let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
// or
// new file
let mut book = umya_spreadsheet::new_file();

// change value
let _ = book.get_sheet_mut(0).get_cell_mut("A1").set_value("TEST1");

// read value
let a1_value = book.get_sheet(0).get_cell("A1").unwrap().get_value();
assert_eq!("TEST1", a1_value);  // TEST1

// writer
let path = std::path::Path::new("C:/spread_test_data/bbb.xlsx");
let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
```
## License
MIT
