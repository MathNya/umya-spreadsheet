use super::BooleanValue;
use super::DoubleValue;
use super::Style;
use super::Stylesheet;
use super::UInt32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use reader::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Column {
    pub(crate) col_num: UInt32Value,
    pub(crate) width: DoubleValue,
    pub(crate) hidden: BooleanValue,
    pub(crate) best_fit: BooleanValue,
    pub(crate) style: Style,
}
impl Column {
    pub fn get_col_num(&self) -> &u32 {
        &self.col_num.get_value()
    }

    pub(crate) fn set_col_num(&mut self, value: u32) -> &mut Self {
        self.col_num.set_value(value);
        self
    }

    pub fn get_width(&self) -> &f64 {
        &self.width.get_value()
    }

    pub fn set_width(&mut self, value: f64) -> &mut Self {
        self.width.set_value(value);
        self
    }

    pub fn get_hidden(&self) -> &bool {
        &self.hidden.get_value()
    }

    pub fn set_hidden(&mut self, value: bool) -> &mut Self {
        self.hidden.set_value(value);
        self
    }

    pub fn get_best_fit(&self) -> &bool {
        &self.best_fit.get_value()
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
            md5::compute(format!(
                "{}{}{}{}",
                &self.width.get_value_string(),
                &self.hidden.get_value_string(),
                &self.best_fit.get_value_string(),
                &self.style.get_hash_code(),
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
