// a:srcRect
use std::io::Cursor;

use quick_xml::{events::BytesStart, Reader, Writer};

use crate::{reader::driver::get_attribute_value, writer::driver::write_start_tag, StringValue};

#[derive(Clone, Default, Debug)]
pub struct SourceRectangle {
    t: StringValue,
    l: StringValue,
    r: StringValue,
    b: StringValue,
}
impl SourceRectangle {
    #[inline]
    pub fn set_t<S: Into<String>>(&mut self, value: S) {
        self.t.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn get_t(&self) -> Option<&str> {
        self.t.get_value()
    }

    #[inline]
    pub fn set_l<S: Into<String>>(&mut self, value: S) {
        self.l.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn get_l(&self) -> Option<&str> {
        self.l.get_value()
    }

    #[inline]
    pub fn set_r<S: Into<String>>(&mut self, value: S) {
        self.r.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn get_r(&self) -> Option<&str> {
        self.r.get_value()
    }

    #[inline]
    pub fn set_b<S: Into<String>>(&mut self, value: S) {
        self.b.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn get_b(&self) -> Option<&str> {
        self.b.get_value()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        for attr in e.attributes().with_checks(false).flatten() {
            match attr.key.0 {
                b"t" => self.set_t(get_attribute_value(&attr).unwrap()),
                b"l" => self.set_l(get_attribute_value(&attr).unwrap()),
                b"r" => self.set_r(get_attribute_value(&attr).unwrap()),
                b"b" => self.set_b(get_attribute_value(&attr).unwrap()),
                _ => {}
            }
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:srcRect
        let mut attributes: crate::structs::AttrCollection = Vec::new();

        if let Some(v) = self.t.get_value() {
            attributes.push(("t", v).into());
        }
        if let Some(v) = self.l.get_value() {
            attributes.push(("l", v).into());
        }
        if let Some(v) = self.r.get_value() {
            attributes.push(("r", v).into());
        }
        if let Some(v) = self.b.get_value() {
            attributes.push(("b", v).into());
        }
        write_start_tag(writer, "a:srcRect", attributes, true);
    }
}
