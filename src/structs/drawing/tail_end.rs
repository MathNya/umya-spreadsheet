// a:tailEnd
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    reader::driver::{
        get_attribute,
        xml_read_loop,
    },
    structs::StringValue,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct TailEnd {
    t_type: StringValue,
    width:  StringValue,
    length: StringValue,
}

impl TailEnd {
    #[inline]
    #[must_use]
    pub fn get_type(&self) -> &str {
        self.t_type.value_str()
    }

    #[inline]
    pub fn set_type<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.t_type.set_value(value.into());
        self
    }

    #[inline]
    #[must_use]
    pub fn width(&self) -> &str {
        self.width.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use width()")]
    pub fn get_width(&self) -> &str {
        self.width()
    }

    #[inline]
    pub fn set_width<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.width.set_value(value.into());
        self
    }

    #[inline]
    #[must_use]
    pub fn length(&self) -> &str {
        self.length.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use length()")]
    pub fn get_length(&self) -> &str {
        self.length()
    }

    #[inline]
    pub fn set_length<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.length.set_value(value.into());
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
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

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:tailEnd" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:tailEnd")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:tailEnd
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.t_type.has_value() {
            attributes.push(("type", (self.t_type.value_str())).into());
        }
        if self.width.has_value() {
            attributes.push(("w", (self.width.value_str())).into());
        }
        if self.length.has_value() {
            attributes.push(("len", (self.length.value_str())).into());
        }
        write_start_tag(writer, "a:tailEnd", attributes, true);
    }
}
