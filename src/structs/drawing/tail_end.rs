// a:tailEnd
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::StringValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct TailEnd {
    t_type: StringValue,
    width: StringValue,
    length: StringValue,
}

impl TailEnd {
    pub fn get_type(&self) -> &str {
        self.t_type.get_value()
    }

    pub fn set_type<S: Into<String>>(&mut self, value: S) {
        self.t_type.set_value(value.into());
    }

    pub fn get_width(&self) -> &str {
        self.width.get_value()
    }

    pub fn set_width<S: Into<String>>(&mut self, value: S) {
        self.width.set_value(value.into());
    }

    pub fn get_length(&self) -> &str {
        self.length.get_value()
    }

    pub fn set_length<S: Into<String>>(&mut self, value: S) {
        self.length.set_value(value.into());
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        if let Some(v) = get_attribute(e, b"type") {
            self.set_type(v);
        }

        if let Some(v) = get_attribute(e, b"w") {
            self.set_width(v);
        }

        if let Some(v) = get_attribute(e, b"len") {
            self.set_length(v);
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:tailEnd
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.t_type.has_value() {
            attributes.push(("type", (self.t_type.get_value_str())));
        }
        if self.width.has_value() {
            attributes.push(("w", (self.width.get_value_str())));
        }
        if self.length.has_value() {
            attributes.push(("len", (self.length.get_value_str())));
        }
        write_start_tag(writer, "a:tailEnd", attributes, true);
    }
}
