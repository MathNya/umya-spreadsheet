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
