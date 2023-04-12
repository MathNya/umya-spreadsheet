// sheetFormatPr
use super::BooleanValue;
use super::ByteValue;
use super::DoubleValue;
use super::UInt32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct SheetFormatProperties {
    base_column_width: UInt32Value,
    custom_height: BooleanValue,
    default_column_width: DoubleValue,
    default_row_height: DoubleValue,
    dy_descent: DoubleValue,
    outline_level_column: ByteValue,
    outline_level_row: ByteValue,
    thick_bottom: BooleanValue,
    thick_top: BooleanValue,
}
impl SheetFormatProperties {
    pub fn get_base_column_width(&self) -> &u32 {
        self.base_column_width.get_value()
    }

    pub fn set_base_column_width(&mut self, value: u32) -> &mut Self {
        self.base_column_width.set_value(value);
        self
    }

    pub fn get_custom_height(&self) -> &bool {
        self.custom_height.get_value()
    }

    pub fn set_custom_height(&mut self, value: bool) -> &mut Self {
        self.custom_height.set_value(value);
        self
    }

    pub fn get_default_column_width(&self) -> &f64 {
        self.default_column_width.get_value()
    }

    pub fn set_default_column_width(&mut self, value: f64) -> &mut Self {
        self.default_column_width.set_value(value);
        self
    }

    pub fn get_default_row_height(&self) -> &f64 {
        self.default_row_height.get_value()
    }

    pub fn set_default_row_height(&mut self, value: f64) -> &mut Self {
        self.default_row_height.set_value(value);
        self
    }

    pub fn get_dy_descent(&self) -> &f64 {
        self.dy_descent.get_value()
    }

    pub fn set_dy_descent(&mut self, value: f64) -> &mut Self {
        self.dy_descent.set_value(value);
        self
    }

    pub fn get_outline_level_column(&self) -> &u8 {
        self.outline_level_column.get_value()
    }

    pub fn set_outline_level_column(&mut self, value: u8) -> &mut Self {
        self.outline_level_column.set_value(value);
        self
    }

    pub fn get_outline_level_row(&self) -> &u8 {
        self.outline_level_row.get_value()
    }

    pub fn set_outline_level_row(&mut self, value: u8) -> &mut Self {
        self.outline_level_row.set_value(value);
        self
    }

    pub fn get_thick_bottom(&self) -> &bool {
        self.thick_bottom.get_value()
    }

    pub fn set_thick_bottom(&mut self, value: bool) -> &mut Self {
        self.thick_bottom.set_value(value);
        self
    }

    pub fn get_thick_top(&self) -> &bool {
        self.thick_top.get_value()
    }

    pub fn set_thick_top(&mut self, value: bool) -> &mut Self {
        self.thick_top.set_value(value);
        self
    }

    pub(crate) fn set_defalut_value(&mut self) -> &mut Self {
        self.default_row_height.set_value(13.5);
        self.dy_descent.set_value(0.15);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"baseColWidth") {
            Some(v) => {
                self.base_column_width.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"customHeight") {
            Some(v) => {
                self.custom_height.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"defaultColWidth") {
            Some(v) => {
                self.default_column_width.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"defaultRowHeight") {
            Some(v) => {
                self.default_row_height.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"x14ac:dyDescent") {
            Some(v) => {
                self.dy_descent.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"outlineLevelCol") {
            Some(v) => {
                self.outline_level_column.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"outlineLevelRow") {
            Some(v) => {
                self.outline_level_row.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"thickBottom") {
            Some(v) => {
                self.thick_bottom.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"thickTop") {
            Some(v) => {
                self.thick_top.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // sheetFormatPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let str_base_column_width = self.base_column_width.get_value_string();
        if self.base_column_width.has_value() {
            attributes.push(("baseColWidth", &str_base_column_width));
        }

        let str_custom_height = self.custom_height.get_value_string();
        if self.custom_height.has_value() {
            attributes.push(("customHeight", &str_custom_height));
        }

        let str_default_column_width = self.default_column_width.get_value_string();
        if self.default_column_width.has_value() {
            attributes.push(("defaultColWidth", &str_default_column_width));
        }

        let str_default_row_height = self.default_row_height.get_value_string();
        if self.default_row_height.has_value() {
            attributes.push(("defaultRowHeight", &str_default_row_height));
        }

        let str_dy_descent = self.dy_descent.get_value_string();
        if self.dy_descent.has_value() {
            attributes.push(("x14ac:dyDescent", &str_dy_descent));
        }

        let str_outline_level_column = self.outline_level_column.get_value_string();
        if self.outline_level_column.has_value() {
            attributes.push(("outlineLevelCol", &str_outline_level_column));
        }

        let str_outline_level_row = self.outline_level_row.get_value_string();
        if self.outline_level_row.has_value() {
            attributes.push(("outlineLevelRow", &str_outline_level_row));
        }

        let str_thick_bottom = self.thick_bottom.get_value_string();
        if self.thick_bottom.has_value() {
            attributes.push(("thickBottom", &str_thick_bottom));
        }

        let str_thick_top = self.thick_top.get_value_string();
        if self.thick_top.has_value() {
            attributes.push(("thickTop", &str_thick_top));
        }

        write_start_tag(writer, "sheetFormatPr", attributes, true);
    }
}
