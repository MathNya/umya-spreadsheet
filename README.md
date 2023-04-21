# umya-spreadsheet
![Result Image](./images/title.png)

[![Crates.io](https://img.shields.io/crates/v/umya-spreadsheet)](https://crates.io/crates/umya-spreadsheet)
[![Crates.io](https://img.shields.io/crates/l/umya-spreadsheet)](https://github.com/MathNya/umya-spreadsheet#license)
[![Join the chat at https://gitter.im/MathNya/umya-spreadsheet](https://badges.gitter.im/MathNya/umya-spreadsheet.svg)](https://gitter.im/MathNya/umya-spreadsheet?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

## Description
**umya-spreadsheet** is a library written in pure Rust and read and write xlsx file.

## Chatting
Please use [Gitter](https://gitter.im/MathNya/umya-spreadsheet) for brief chats.

## New feature
### ver 0.9.2
#### **new function get_lazy_read_sheet_cells**
Cell information can now be retrieved from a worksheet loaded with lazy_read.
```rust
let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
let mut book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();
let cells = book.get_lazy_read_sheet_cells(&0).unwrap();
let value = cells.get_cell_value((&5, &12)).get_value();
let value = cells.get_cell_value("E12").get_value();
```

#### **set_value_from_string is deprecated**
Cell.set_value_from_string had different behavior in different versions.  
This function is deprecated.  
From now on, set_value or set_value_string.

### ver 0.9.1
Changed file compression logic when writing.  
The file size is smaller than before, but the processing time is longer.  
If you want to use the previous logic, use this method.
```rust
umya_spreadsheet::writer::xlsx::write_light(&book, path);
umya_spreadsheet::writer::xlsx::write_with_password_light(&book, path, "password");
```
### ver 0.9
The way cells are referenced has changed.
```rust
// old
let value = worksheet.get_value("A1");
let value = worksheet.get_value_by_column_and_row(&1, &1);
// This one has been deprecated.
// It will eventually disappear.

// NEW
let value = worksheet.get_value("A1");
let value = worksheet.get_value((1, 1));
let value = worksheet.get_value((&1, &1));
```

### ver 0.8
A password can now be set when saving a file.
```rust
let path = std::path::Path::new("./tests/result_files/bbb.xlsx");
let _ = umya_spreadsheet::writer::xlsx::write_with_password(&book, path, "password");
```
```rust
let from_path = std::path::Path::new("./tests/test_files/aaa.xlsx");
let to_path = std::path::Path::new("./tests/result_files/bbb.xlsx");
let _ = umya_spreadsheet::writer::xlsx::set_password(&from_path, &to_path, "password");
```

## Usage
### Installation
Add the following code to Cargo.toml
```toml
[dependencies]
umya-spreadsheet = "0.9"
```
Add the following code to main.rs
```rust
extern crate umya_spreadsheet;
```
### Read file
```rust
let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
```
### Read file (Lazy)
Delays the loading of the worksheet until it is needed.  
When loading a file with a large amount of data, response improvement can be expected.
```rust
let path = std::path::Path::new("./tests/test_files/aaa.xlsx");
let mut book = umya_spreadsheet::reader::xlsx::lazy_read(path).unwrap();
```
### New file
```rust
let mut book = umya_spreadsheet::new_file();
```
### Write file
```rust
let path = std::path::Path::new("./tests/result_files/bbb.xlsx");
let _ = umya_spreadsheet::writer::xlsx::write(&book, path);
```
### Write file with password
```rust
let path = std::path::Path::new("./tests/result_files/bbb.xlsx");
let _ = umya_spreadsheet::writer::xlsx::write_with_password(&book, path, "password");
```
```rust
let from_path = std::path::Path::new("./tests/test_files/aaa.xlsx");
let to_path = std::path::Path::new("./tests/result_files/bbb.xlsx");
let _ = umya_spreadsheet::writer::xlsx::set_password(&from_path, &to_path, "password");
```
### Change Value
```rust
let mut book = umya_spreadsheet::new_file();
let _ = book.new_sheet("Sheet2");
book.get_sheet_by_name_mut("Sheet2").unwrap().get_cell_mut("A1").set_value("TEST1");
```
### Change Style
```rust
let mut book = umya_spreadsheet::new_file();
let mut style = book.get_sheet_by_name_mut("Sheet1").unwrap().get_style_mut("A1");
// fill color on red.
style.set_background_color(umya_spreadsheet::Color::COLOR_RED);
```
### New Chart
```rust
let mut book = umya_spreadsheet::new_file();
// Add Chart
let mut from_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
from_marker.set_coordinate("C1");
let mut to_marker = umya_spreadsheet::structs::drawing::spreadsheet::MarkerType::default();
to_marker.set_coordinate("D11");
let area_chart_series_list = vec![
    "Sheet1!$A$1:$A$10",
    "Sheet1!$B$1:$B$10",
];
let mut chart = umya_spreadsheet::structs::Chart::default();
chart.new_chart(
    umya_spreadsheet::structs::ChartType::LineChart,
    from_marker,
    to_marker,
    area_chart_series_list,
);
book.get_sheet_by_name_mut("Sheet1").unwrap()
    .add_chart(chart);
```
See the next chapter for implementation status and more detailed usage.

## Support Status
| Function | detail | example |
| --- | --- | --- |
| file reader | xlsx, xlsm | [**here**](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/reader/xlsx/fn.read.html). |
| file lazy_reader | xlsx, xlsm | [**here**](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/reader/xlsx/fn.lazy_read.html). |
| file writer | xlsx, xlsm | [**here**](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/writer/xlsx/fn.write.html). |
|  | csv | [**here**](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/writer/csv/fn.write.html). |
| file writer with password | xlsx, xlsm | [**here**](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/writer/xlsx/fn.write_with_password.html). |
| worksheet | read, new, copy | [**here**](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/). |
| cell value | read, edit, formated value. | [**here**](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/). |
| cell style | read, edit | [**here**](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/structs/struct.Style.html).  |
| columns | read, edit, auto width | [**here**](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/structs/struct.Column.html).  |
| rows | read, edit |  |
| charts | read, edit | [**here**](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/structs/struct.Chart.html).  |
| drawings | read, edit(Still might be inconvenient.) |  |
| images | read, edit | [**here**](https://docs.rs/umya-spreadsheet/latest/umya_spreadsheet/structs/struct.Image.html). |
| ole objects | read, edit(Still might be inconvenient.) |  |

## License
MIT
