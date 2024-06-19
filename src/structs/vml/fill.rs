use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::StringValue;
use structs::TrueFalseValue;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Fill {
    color: StringValue,
    color_2: StringValue,
    on: TrueFalseValue,
    focus_size: StringValue,
}

impl Fill {
    pub fn get_color(&self) -> &str {
        self.color.get_value_str()
    }

    pub fn set_color<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.color.set_value(value);
        self
    }

    pub fn get_color_2(&self) -> &str {
        self.color_2.get_value_str()
    }

    pub fn set_color_2<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.color_2.set_value(value);
        self
    }

    pub fn get_on(&self) -> &bool {
        self.on.get_value()
    }

    pub fn set_on(&mut self, value: bool) -> &mut Self {
        self.on.set_value(value);
        self
    }

    pub fn get_focus_size(&self) -> &str {
        self.focus_size.get_value_str()
    }

    pub fn set_focus_size<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.focus_size.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, color, "color");
        set_string_from_xml!(self, e, color_2, "color2");
        set_string_from_xml!(self, e, on, "on");
        set_string_from_xml!(self, e, focus_size, "focussize");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // v:fill
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.color.has_value() {
            attributes.push(("color", self.color.get_value_str()));
        }
        if self.color_2.has_value() {
            attributes.push(("color2", self.color_2.get_value_str()));
        }
        if self.on.has_value() {
            attributes.push(("on", self.on.get_value_string()));
        }
        if self.focus_size.has_value() {
            attributes.push(("focussize", self.focus_size.get_value_str()));
        }
        write_start_tag(writer, "v:fill", attributes, true);
    }
}
