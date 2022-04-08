use super::BooleanValue;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct PrintOptions {
    horizontal_centered: BooleanValue,
    vertical_centered: BooleanValue,
}
impl PrintOptions {
    pub fn get_horizontal_centered(&self) -> &bool {
        self.horizontal_centered.get_value()
    }

    pub fn set_horizontal_centered(&mut self, value: bool) -> &mut Self {
        self.horizontal_centered.set_value(value);
        self
    }

    pub fn get_vertical_centered(&self) -> &bool {
        self.vertical_centered.get_value()
    }

    pub fn set_vertical_centered(&mut self, value: bool) -> &mut Self {
        self.vertical_centered.set_value(value);
        self
    }

    pub(crate) fn has_param(&self) -> bool {
        if self.horizontal_centered.has_value() {
            return true;
        }
        if self.vertical_centered.has_value() {
            return true;
        }
        false
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"horizontalCentered") {
            Some(v) => {
                self.horizontal_centered.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"verticalCentered") {
            Some(v) => {
                self.vertical_centered.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if self.has_param() {
            // printOptions
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            if self.horizontal_centered.has_value() {
                attributes.push((
                    "horizontalCentered",
                    self.horizontal_centered.get_value_string(),
                ));
            }
            if self.vertical_centered.has_value() {
                attributes.push((
                    "verticalCentered",
                    self.vertical_centered.get_value_string(),
                ));
            }
            write_start_tag(writer, "printOptions", attributes, true);
        }
    }
}
