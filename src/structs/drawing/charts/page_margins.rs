// c:pageMargins
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::super::super::DoubleValue;
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct PageMargins {
    bottom: DoubleValue,
    left: DoubleValue,
    right: DoubleValue,
    top: DoubleValue,
    header: DoubleValue,
    footer: DoubleValue,
}
impl PageMargins {
    #[must_use]
    pub fn get_bottom(&self) -> f64 {
        self.bottom.get_value()
    }

    pub fn set_bottom(&mut self, value: f64) -> &mut Self {
        self.bottom.set_value(value);
        self
    }

    #[must_use]
    pub fn get_left(&self) -> f64 {
        self.left.get_value()
    }

    pub fn set_left(&mut self, value: f64) -> &mut Self {
        self.left.set_value(value);
        self
    }

    #[must_use]
    pub fn get_right(&self) -> f64 {
        self.right.get_value()
    }

    pub fn set_right(&mut self, value: f64) -> &mut Self {
        self.right.set_value(value);
        self
    }

    #[must_use]
    pub fn get_top(&self) -> f64 {
        self.top.get_value()
    }

    pub fn set_top(&mut self, value: f64) -> &mut Self {
        self.top.set_value(value);
        self
    }

    #[must_use]
    pub fn get_header(&self) -> f64 {
        self.header.get_value()
    }

    pub fn set_header(&mut self, value: f64) -> &mut Self {
        self.header.set_value(value);
        self
    }

    #[must_use]
    pub fn get_footer(&self) -> f64 {
        self.footer.get_value()
    }

    pub fn set_footer(&mut self, value: f64) -> &mut Self {
        self.footer.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.bottom.set_value_string(get_attribute(e, b"b").unwrap());
        self.left.set_value_string(get_attribute(e, b"l").unwrap());
        self.right.set_value_string(get_attribute(e, b"r").unwrap());
        self.top.set_value_string(get_attribute(e, b"t").unwrap());
        self.header.set_value_string(get_attribute(e, b"header").unwrap());
        self.footer.set_value_string(get_attribute(e, b"footer").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:pageMargins
        write_start_tag(
            writer,
            "c:pageMargins",
            vec![
                ("b", &self.bottom.get_value_string()),
                ("l", &self.left.get_value_string()),
                ("r", &self.right.get_value_string()),
                ("t", &self.top.get_value_string()),
                ("header", &self.header.get_value_string()),
                ("footer", &self.footer.get_value_string()),
            ],
            true,
        );
    }
}
