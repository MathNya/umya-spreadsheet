use structs::DoubleValue;

use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct PageMargins {
    left: DoubleValue,
    right: DoubleValue,
    top: DoubleValue,
    bottom: DoubleValue,
    header: DoubleValue,
    footer: DoubleValue,
}
impl PageMargins {
    pub fn get_left(&self) -> &f64 {
        self.left.get_value()
    }

    pub fn set_left(&mut self, value: f64) -> &mut Self {
        self.left.set_value(value);
        self
    }

    pub fn get_right(&self) -> &f64 {
        self.right.get_value()
    }

    pub fn set_right(&mut self, value: f64) -> &mut Self {
        self.right.set_value(value);
        self
    }

    pub fn get_top(&self) -> &f64 {
        self.top.get_value()
    }

    pub fn set_top(&mut self, value: f64) -> &mut Self {
        self.top.set_value(value);
        self
    }

    pub fn get_bottom(&self) -> &f64 {
        self.bottom.get_value()
    }

    pub fn set_bottom(&mut self, value: f64) -> &mut Self {
        self.bottom.set_value(value);
        self
    }

    pub fn get_header(&self) -> &f64 {
        self.header.get_value()
    }

    pub fn set_header(&mut self, value: f64) -> &mut Self {
        self.header.set_value(value);
        self
    }

    pub fn get_footer(&self) -> &f64 {
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
        self.left
            .set_value_string(get_attribute(e, b"left").unwrap());
        self.right
            .set_value_string(get_attribute(e, b"right").unwrap());
        self.top.set_value_string(get_attribute(e, b"top").unwrap());
        self.bottom
            .set_value_string(get_attribute(e, b"bottom").unwrap());
        self.header
            .set_value_string(get_attribute(e, b"header").unwrap());
        self.footer
            .set_value_string(get_attribute(e, b"footer").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pageMargins
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let left = self.left.get_value_string();
        attributes.push(("left", &left));
        let right = self.right.get_value_string();
        attributes.push(("right", &right));
        let top = self.top.get_value_string();
        attributes.push(("top", &top));
        let bottom = self.bottom.get_value_string();
        attributes.push(("bottom", &bottom));
        let header = self.header.get_value_string();
        attributes.push(("header", &header));
        let footer = self.footer.get_value_string();
        attributes.push(("footer", &footer));
        write_start_tag(writer, "pageMargins", attributes, true);
    }
}
