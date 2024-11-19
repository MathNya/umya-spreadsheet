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
    #[inline]
    pub fn get_horizontal_centered(&self) -> &bool {
        self.horizontal_centered.get_value()
    }

    #[inline]
    pub fn set_horizontal_centered(&mut self, value: bool) -> &mut Self {
        self.horizontal_centered.set_value(value);
        self
    }

    #[inline]
    pub fn get_vertical_centered(&self) -> &bool {
        self.vertical_centered.get_value()
    }

    #[inline]
    pub fn set_vertical_centered(&mut self, value: bool) -> &mut Self {
        self.vertical_centered.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn has_param(&self) -> bool {
        self.horizontal_centered.has_value() || self.vertical_centered.has_value()
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, horizontal_centered, "horizontalCentered");
        set_string_from_xml!(self, e, vertical_centered, "verticalCentered");
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
