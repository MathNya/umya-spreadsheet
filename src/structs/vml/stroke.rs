use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    structs::StringValue,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Stroke {
    color:      StringValue,
    color_2:    StringValue,
    dash_style: StringValue,
}

impl Stroke {
    #[must_use]
    pub fn get_color(&self) -> &str {
        self.color.value_str()
    }

    pub fn set_color<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.color.set_value(value);
        self
    }

    #[must_use]
    pub fn get_color_2(&self) -> &str {
        self.color_2.value_str()
    }

    pub fn set_color_2<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.color_2.set_value(value);
        self
    }

    #[must_use]
    pub fn get_dash_style(&self) -> &str {
        self.dash_style.value_str()
    }

    pub fn set_dash_style<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.dash_style.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, color, "color");
        set_string_from_xml!(self, e, color_2, "color2");
        set_string_from_xml!(self, e, dash_style, "dashstyle");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // v:stroke
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.color.has_value() {
            attributes.push(("color", self.color.value_str()).into());
        }
        if self.color_2.has_value() {
            attributes.push(("color2", self.color_2.value_str()).into());
        }
        if self.dash_style.has_value() {
            attributes.push(("dashstyle", self.dash_style.value_str()).into());
        }
        write_start_tag(writer, "v:stroke", attributes, true);
    }
}
