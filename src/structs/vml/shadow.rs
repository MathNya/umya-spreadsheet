use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::StringValue;
use structs::TrueFalseValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Shadow {
    on: TrueFalseValue,
    color: StringValue,
    obscured: TrueFalseValue,
}
impl Shadow {
    pub fn get_on(&self) -> &bool {
        self.on.get_value()
    }

    pub fn set_on(&mut self, value: bool) -> &mut Self {
        self.on.set_value(value);
        self
    }

    pub fn get_color(&self) -> &str {
        self.color.get_value()
    }

    pub fn set_color<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.color.set_value(value);
        self
    }

    pub fn get_obscured(&self) -> &bool {
        self.obscured.get_value()
    }

    pub fn set_obscured(&mut self, value: bool) -> &mut Self {
        self.obscured.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"on") {
            Some(v) => {
                self.on.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"color") {
            Some(v) => {
                self.color.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"obscured") {
            Some(v) => {
                self.obscured.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // v:shadow
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.on.has_value() {
            attributes.push(("on", self.on.get_value_string()));
        }
        if self.color.has_value() {
            attributes.push(("color", self.color.get_value_string()));
        }
        if self.obscured.has_value() {
            attributes.push(("obscured", self.obscured.get_value_string()));
        }
        write_start_tag(writer, "v:shadow", attributes, true);
    }
}
