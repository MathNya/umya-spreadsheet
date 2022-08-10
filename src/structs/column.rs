use super::BooleanValue;
use super::DoubleValue;
use super::Style;
use super::Stylesheet;
use super::UInt32Value;
use md5::Digest;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use reader::driver::*;
use structs::Cells;

/// # Examples
/// ## set auto width
/// ```rust
/// use umya_spreadsheet::*;
/// let mut book = new_file();
/// let mut worksheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
/// worksheet.get_column_dimension_mut("A").set_auto_width(true);
/// ```
/// ## set manual width
/// ```rust
/// use umya_spreadsheet::*;
/// let mut book = new_file();
/// let mut worksheet = book.get_sheet_by_name_mut("Sheet1").unwrap();
/// worksheet.get_column_dimension_mut("A").set_width(60f64);
/// ```
#[derive(Clone, Debug)]
pub struct Column {
    col_num: UInt32Value,
    pub(crate) width: DoubleValue,
    pub(crate) hidden: BooleanValue,
    pub(crate) best_fit: BooleanValue,
    style: Style,
    auto_width: BooleanValue,
}
impl Default for Column {
    fn default() -> Self {
        let mut width = DoubleValue::default();
        width.set_value(8.38f64);
        Self {
            col_num: UInt32Value::default(),
            width,
            hidden: BooleanValue::default(),
            best_fit: BooleanValue::default(),
            style: Style::default(),
            auto_width: BooleanValue::default(),
        }
    }
}
impl Column {
    pub fn get_col_num(&self) -> &u32 {
        self.col_num.get_value()
    }

    pub fn set_col_num(&mut self, value: u32) -> &mut Self {
        self.col_num.set_value(value);
        self
    }

    pub fn get_width(&self) -> &f64 {
        self.width.get_value()
    }

    pub fn set_width(&mut self, value: f64) -> &mut Self {
        self.width.set_value(value);
        self
    }

    pub fn get_hidden(&self) -> &bool {
        self.hidden.get_value()
    }

    pub fn set_hidden(&mut self, value: bool) -> &mut Self {
        self.hidden.set_value(value);
        self
    }

    pub fn get_best_fit(&self) -> &bool {
        self.best_fit.get_value()
    }

    pub fn set_best_fit(&mut self, value: bool) -> &mut Self {
        self.best_fit.set_value(value);
        self
    }

    pub fn get_style(&self) -> &Style {
        &self.style
    }

    pub fn get_style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    pub fn set_style(&mut self, value: Style) -> &mut Self {
        self.style = value;
        self
    }

    pub fn get_auto_width(&self) -> &bool {
        self.auto_width.get_value()
    }

    pub fn set_auto_width(&mut self, value: bool) -> &mut Self {
        self.auto_width.set_value(value);
        self
    }

    pub(crate) fn calculation_auto_width(&mut self, cells: &Cells) -> &mut Self {
        if self.get_auto_width() == &false {
            return self;
        }

        let mut column_width_max = 0f64;

        // default font size len.
        let column_font_size = match self.get_style().get_font() {
            Some(font) => *font.get_font_size().get_val(),
            None => 11f64,
        };

        let mut cell_list = cells.get_collection_by_column(self.get_col_num());
        cell_list.sort_by(|a, b| {
            a.get_coordinate()
                .get_row_num()
                .cmp(b.get_coordinate().get_row_num())
        });
        for cell in cell_list {
            let column_width = cell.get_width_point(&column_font_size);

            if column_width > column_width_max {
                column_width_max = column_width;
            }
        }

        // set default width if empty column.
        if column_width_max == 0f64 {
            column_width_max = 8.38f64;
        }

        self.set_width(column_width_max);
        self
    }

    pub(crate) fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
    ) {
        if self.col_num.get_value() >= root_col_num {
            self.col_num
                .set_value(self.col_num.get_value() + offset_col_num);
        }
    }

    pub(crate) fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
    ) {
        if self.col_num.get_value() >= root_col_num {
            self.col_num
                .set_value(self.col_num.get_value() - offset_col_num);
        }
    }

    pub(crate) fn get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}{}",
                &self.width.get_value_string(),
                &self.hidden.get_value_string(),
                &self.best_fit.get_value_string(),
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
        stylesheet: &Stylesheet,
    ) {
        match get_attribute(e, b"width") {
            Some(v) => {
                self.width.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"hidden") {
            Some(v) => {
                self.hidden.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"bestFit") {
            Some(v) => {
                self.best_fit.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"style") {
            Some(v) => {
                let style = stylesheet.get_style(v.parse::<usize>().unwrap());
                self.set_style(style);
            }
            None => {}
        }
    }
}
