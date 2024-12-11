// brk
use crate::reader::driver::*;
use crate::structs::BooleanValue;
use crate::structs::UInt32Value;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Break {
    id: UInt32Value,
    max: UInt32Value,
    min: UInt32Value,
    manual_page_break: BooleanValue,
}

impl Break {
    #[inline]
    pub fn get_id(&self) -> &u32 {
        self.id.get_value()
    }

    #[inline]
    pub fn set_id(&mut self, value: u32) -> &mut Self {
        self.id.set_value(value);
        self
    }

    #[inline]
    pub fn get_max(&self) -> &u32 {
        self.max.get_value()
    }

    #[inline]
    pub fn set_max(&mut self, value: u32) -> &mut Self {
        self.max.set_value(value);
        self
    }

    #[inline]
    pub fn get_manual_page_break(&self) -> &bool {
        self.manual_page_break.get_value()
    }

    #[inline]
    pub fn set_manual_page_break(&mut self, value: bool) -> &mut Self {
        self.manual_page_break.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, id, "id");
        set_string_from_xml!(self, e, max, "max");
        set_string_from_xml!(self, e, min, "min");
        set_string_from_xml!(self, e, manual_page_break, "man");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // brk
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let id = self.id.get_value_string();
        attributes.push(("id", &id));

        let max = self.max.get_value_string();
        if self.max.has_value() {
            attributes.push(("max", &max));
        }

        let min = self.min.get_value_string();
        if self.min.has_value() {
            attributes.push(("min", &min));
        }

        let manual_page_break = self.manual_page_break.get_value_string();
        if self.manual_page_break.has_value() {
            attributes.push(("man", manual_page_break));
        }
        write_start_tag(writer, "brk", attributes, true);
    }
}
