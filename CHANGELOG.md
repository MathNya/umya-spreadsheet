# Change Detail -> 3.1.1
### Bug Fixes and Improvements
* [#299 Page scaling resets to the default setting](https://github.com/MathNya/umya-spreadsheet/issues/299)
* [#357 Preserve t="str" on empty-string formula results](https://github.com/MathNya/umya-spreadsheet/pull/357)
* [#358 fix: keep worksheet drawing r:id in sync with its rels](https://github.com/MathNya/umya-spreadsheet/pull/358)
* [#359 fix: preserve row thickTop](https://github.com/MathNya/umya-spreadsheet/pull/359)
* [#360 Fix Excel theme tint color matching](https://github.com/MathNya/umya-spreadsheet/pull/360)
* [#361 Fix literal-only zero number format sections](https://github.com/MathNya/umya-spreadsheet/pull/361)
* [#362 Preserve whitespace in literal-only number format sections](https://github.com/MathNya/umya-spreadsheet/pull/362)
* [#363 autoFilter tag added to existing table objects regardless of what it was set to before.](https://github.com/MathNya/umya-spreadsheet/issues/363)
* [#365 fix: preserve color selectors and avoid loaded-sheet style cloning](https://github.com/MathNya/umya-spreadsheet/pull/365)
* [#367 Resolve workbook relationship namespace prefixes](https://github.com/MathNya/umya-spreadsheet/pull/367)
* [#369 Fix CDATA decoding in XML text content](https://github.com/MathNya/umya-spreadsheet/pull/369)
* [#370 Prefixed worksheet elements silently lose cells and header/footer content](https://github.com/MathNya/umya-spreadsheet/issues/370)
* [#371 Resolve worksheet namespace prefixes in streaming readers](https://github.com/MathNya/umya-spreadsheet/pull/371)
* [#373 Preserve explicit zero page margins and expose presence](https://github.com/MathNya/umya-spreadsheet/pull/373)
* [#374 Panic when loading merged cell with hyperlink](https://github.com/MathNya/umya-spreadsheet/issues/374)
* [#375 Keep the 1904 date system when reading and writing](https://github.com/MathNya/umya-spreadsheet/pull/375)
* [#376 Don't panic on quoted text around a number placeholder](https://github.com/MathNya/umya-spreadsheet/pull/376)
* [#377 Read files with dangling references instead of panicking](https://github.com/MathNya/umya-spreadsheet/pull/377)
* [#379 Keep the fit to page scaling option when reading and writing](https://github.com/MathNya/umya-spreadsheet/pull/379)

# Change Detail -> 3.1.0

### Pivot Tables Update.
* We have enhanced read and write operations for Pivot Tables.
(As a result, this update includes some breaking changes.)

### Bug Fixed
* We have fixed a bug that caused the grid lines to disappear.

* Data validation / formula character data containing XML numeric character references (e.g. `&#8211;`) was truncated. `formula1`/`formula2` text is now accumulated across successive `Text` and `GeneralRef` events from quick-xml.

### Fixes for other minor bugs

# Change Detail -> 3.0.0
### We have changed the name of the Getter.
```rust
// ver2.3.3
book.get_sheet_mut(&0).unwrap().get_cell_mut("A1").get_value();

// ver3.0.0
book.sheet_mut(&0).unwrap().cell_mut("A1").value();
```
The get_xx() functions are now deprecated, but they can still be used.
They will be removed at some point.

### “Spreadsheet” has been renamed to “Workbook.”
* ver2.3.3 https://github.com/MathNya/umya-spreadsheet/blob/2.3.3/src/structs/spreadsheet.rs
* ver3.0.0 https://github.com/MathNya/umya-spreadsheet/blob/3.0.0/src/structs/workbook.rs

### Processing is now faster.

### Minor bug fixes

# Change Detail -> 2.3.2
### Bug Fixed #290,#291

# Change Detail -> 2.2.0
### Increased processing speed and reduced memory consumption.(Thank you. [schungx](https://github.com/schungx),[mxsrm](https://github.com/mxsrm))
The return type has been changed in some functions.
Please be aware of this.

### copy_row_styling(),copy_col_styling() is now available.
Copies the style of the specified column or row.
```rust
let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
let sheet = book.sheet_mut(&0).unwrap();
sheet.copy_row_styling(&3, &5, None, None);
sheet.copy_col_styling(&3, &5, None, None);
```
### The function to create a new comment has been implemented.
```rust
let mut book = umya_spreadsheet::reader::xlsx::read(path).unwrap();
let sheet = book.sheet_mut(&0).unwrap();
let mut comment = Comment::default();
comment.new_comment("B2");
comment.set_text_string("TEST");
sheet.add_comments(comment);
```
### Minor bug fixes

# Change Detail v1.2.7 -> v2.0.0

--- failure enum_missing: pub enum removed or renamed ---

Description:
A publicly-visible enum cannot be imported by its prior path. A `pub use` may have been removed, or the enum itself may have been renamed or removed entirely.
        ref: https://doc.rust-lang.org/cargo/reference/semver.html#item-remove
       impl: https://github.com/obi1kenobi/cargo-semver-checks/tree/v0.32.0/src/lints/enum_missing.ron

Failed in:  
  enum umya_spreadsheet::writer::xlsx::XlsxError, previously in file src\writer\xlsx.rs:39  
  enum umya_spreadsheet::writer::csv::XlsxError, previously in file src\writer\csv.rs:12  
  enum umya_spreadsheet::reader::xlsx::XlsxError, previously in file src\reader\xlsx.rs:38  

--- failure enum_variant_added: enum variant added on exhaustive enum ---

Description:  
A publicly-visible enum without #[non_exhaustive] has a new variant.  
        ref: https://doc.rust-lang.org/cargo/reference/semver.html#enum-variant-new  
       impl: https://github.com/obi1kenobi/cargo-semver-checks/tree/v0.32.0/src/lints/enum_variant_added.ron  

Failed in:  
  variant CellRawValue:Empty in umya-spreadsheet\src\structs\cell_raw_value.rs:15  
  variant CellRawValue:Empty in umya-spreadsheet\src\structs\cell_raw_value.rs:15  

--- failure enum_variant_missing: pub enum variant removed or renamed ---

Description:  
A publicly-visible enum has at least one variant that is no longer available under its prior name. It may have been renamed or removed entirely.  
        ref: https://doc.rust-lang.org/cargo/reference/semver.html#item-remove  
       impl: https://github.com/obi1kenobi/cargo-semver-checks/tree/v0.32.0/src/lints/enum_variant_missing.ron  

Failed in:  
  variant CellRawValue::Str, previously in file src\structs\cell_raw_value.rs:8  
  variant CellRawValue::Inline, previously in file src\structs\cell_raw_value.rs:13  
  variant CellRawValue::Null, previously in file src\structs\cell_raw_value.rs:15  
  variant CellRawValue::Str, previously in file src\structs\cell_raw_value.rs:8  
  variant CellRawValue::Inline, previously in file src\structs\cell_raw_value.rs:13  
  variant CellRawValue::Null, previously in file src\structs\cell_raw_value.rs:15  

--- failure function_parameter_count_changed: pub fn parameter count changed ---

Description:  
A publicly-visible function now takes a different number of parameters.  
        ref: https://doc.rust-lang.org/cargo/reference/semver.html#fn-change-arity  
       impl: https://github.com/obi1kenobi/cargo-semver-checks/tree/v0.32.0/src/lints/function_parameter_count_changed.ron  

Failed in:  
  umya_spreadsheet::helper::formula::adjustment_remove_formula_coordinate now takes 8 parameters instead of 7, in umya-spreadsheet\src\helper\formula.rs:825  
  umya_spreadsheet::helper::formula::adjustment_insert_formula_coordinate now takes 8 parameters instead of 7, in umya-spreadsheet\src\helper\formula.rs:774  

--- failure inherent_method_missing: pub method removed or renamed ---

Description:  
A publicly-visible method or associated fn is no longer available under its prior name. It may have been renamed or removed entirely.  
        ref: https://doc.rust-lang.org/cargo/reference/semver.html#item-remove  
       impl: https://github.com/obi1kenobi/cargo-semver-checks/tree/v0.32.0/src/lints/inherent_method_missing.ron  

Failed in:  
  Transform2D::get_x, previously in file src\structs\drawing\transform2d.rs:21  
  Transform2D::set_x, previously in file src\structs\drawing\transform2d.rs:25  
  Transform2D::get_y, previously in file src\structs\drawing\transform2d.rs:29  
  Transform2D::set_y, previously in file src\structs\drawing\transform2d.rs:33  
  Transform2D::get_width, previously in file src\structs\drawing\transform2d.rs:37  
  Transform2D::set_width, previously in file src\structs\drawing\transform2d.rs:41  
  Transform2D::get_height, previously in file src\structs\drawing\transform2d.rs:45  
  Transform2D::set_height, previously in file src\structs\drawing\transform2d.rs:49  
  Transform2D::get_x, previously in file src\structs\drawing\transform2d.rs:21  
  Transform2D::set_x, previously in file src\structs\drawing\transform2d.rs:25  
  Transform2D::get_y, previously in file src\structs\drawing\transform2d.rs:29  
  Transform2D::set_y, previously in file src\structs\drawing\transform2d.rs:33  
  Transform2D::get_width, previously in file src\structs\drawing\transform2d.rs:37  
  Transform2D::set_width, previously in file src\structs\drawing\transform2d.rs:41  
  Transform2D::get_height, previously in file src\structs\drawing\transform2d.rs:45  
  Transform2D::set_height, previously in file src\structs\drawing\transform2d.rs:49  
  CellValue::set_formula_attributes, previously in file src\structs\cell_value.rs:27  
  CellValue::get_formula_attributes, previously in file src\structs\cell_value.rs:31  
  CellValue::set_formula_attributes, previously in file src\structs\cell_value.rs:27  
  CellValue::get_formula_attributes, previously in file src\structs\cell_value.rs:31  

--- failure method_parameter_count_changed: pub method parameter count changed ---

Description:  
A publicly-visible method now takes a different number of parameters.  
        ref: https://doc.rust-lang.org/cargo/reference/semver.html#fn-change-arity  
       impl: https://github.com/obi1kenobi/cargo-semver-checks/tree/v0.32.0/src/lints/method_parameter_count_changed.ron  

Failed in:  
  umya_spreadsheet::Cell::set_error now takes 2 parameters instead of 1, in umya-spreadsheet\src\structs\cell.rs:149  
  umya_spreadsheet::structs::Cell::set_error now takes 2 parameters instead of 1, in umya-spreadsheet\src\structs\cell.rs:149  
  umya_spreadsheet::CellValue::set_error now takes 2 parameters instead of 1, in umya-spreadsheet\src\structs\cell_value.rs:153  
  umya_spreadsheet::structs::CellValue::set_error now takes 2 parameters instead of 1, in umya-spreadsheet\src\structs\cell_value.rs:153  
     Summary semver requires new major version: 6 major and 0 minor checks failed
    Finished [  16.915s] umya-spreadsheet
