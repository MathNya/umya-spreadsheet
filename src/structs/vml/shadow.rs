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
    structs::{
        StringValue,
        TrueFalseValue,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Shadow {
    on:       TrueFalseValue,
    color:    StringValue,
    obscured: TrueFalseValue,
}
impl Shadow {
    #[must_use]
    pub fn on(&self) -> bool {
        self.on.value()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use on()")]
    pub fn get_on(&self) -> bool {
        self.on()
    }

    pub fn set_on(&mut self, value: bool) -> &mut Self {
        self.on.set_value(value);
        self
    }

    #[must_use]
    pub fn color(&self) -> &str {
        self.color.value_str()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use color()")]
    pub fn get_color(&self) -> &str {
        self.color()
    }

    pub fn set_color<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.color.set_value(value);
        self
    }

    #[must_use]
    pub fn obscured(&self) -> bool {
        self.obscured.value()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use obscured()")]
    pub fn get_obscured(&self) -> bool {
        self.obscured()
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
        set_string_from_xml!(self, e, on, "on");
        set_string_from_xml!(self, e, color, "color");
        set_string_from_xml!(self, e, obscured, "obscured");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // v:shadow
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.on.has_value() {
            attributes.push(("on", self.on.value_string()).into());
        }
        if self.color.has_value() {
            attributes.push(("color", self.color.value_str()).into());
        }
        if self.obscured.has_value() {
            attributes.push(("obscured", self.obscured.value_string()).into());
        }
        write_start_tag(writer, "v:shadow", attributes, true);
    }
}
