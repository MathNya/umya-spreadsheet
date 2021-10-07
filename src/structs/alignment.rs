// alignment
use super::EnumValue;
use super::HorizontalAlignmentValues;
use super::VerticalAlignmentValues;
use super::BooleanValue;
use reader::driver::*;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Clone, Debug)]
pub struct Alignment {
    horizontal: EnumValue<HorizontalAlignmentValues>,
    vertical: EnumValue<VerticalAlignmentValues>,
    wrap_text: BooleanValue,
}
impl Alignment {
    pub fn get_horizontal(&self)-> &HorizontalAlignmentValues {
        &self.horizontal.get_value()
    }

    pub fn set_horizontal(&mut self, value:HorizontalAlignmentValues) {
        self.horizontal.set_value(value);
    }

    pub fn get_vertical(&self)-> &VerticalAlignmentValues {
        &self.vertical.get_value()
    }

    pub fn set_vertical(&mut self, value:VerticalAlignmentValues) {
        self.vertical.set_value(value);
    }

    pub fn get_wrap_text(&self)-> &bool {
        &self.wrap_text.get_value()
    }

    pub fn set_wrap_text(&mut self, value:bool) {
        self.wrap_text.set_value(value);
    }

    pub(crate) fn get_hash_code(&self)-> String {
        format!("{:x}", md5::compute(format!("{}{}{}",
        &self.horizontal.get_hash_string(),
        &self.vertical.get_hash_string(),
        &self.wrap_text.get_hash_string(),
        )))
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        match get_attribute(e, b"horizontal") {
            Some(v) => {
                self.horizontal.set_value_string(v);
            },
            None => {},
        }
        match get_attribute(e, b"vertical") {
            Some(v) => {
                self.vertical.set_value_string(v);
            },
            None => {},
        }
        match get_attribute(e, b"wrapText") {
            Some(v) => {
                self.wrap_text.set_value_string(v);
            },
            None => {},
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // alignment
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.horizontal.has_value() {
            attributes.push(("horizontal", &self.horizontal.get_value_string()));
        }
        if self.vertical.has_value() {
            attributes.push(("vertical", &self.vertical.get_value_string()));
        }
        if self.wrap_text.has_value() {
            attributes.push(("wrapText", &self.wrap_text.get_value_string()));
        }
        write_start_tag(writer, "alignment", attributes, true);
   }
}