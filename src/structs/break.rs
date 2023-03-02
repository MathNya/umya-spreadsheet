// brk
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::BooleanValue;
use structs::UInt32Value;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Break {
    id: UInt32Value,
    max: UInt32Value,
    min: UInt32Value,
    manual_page_break: BooleanValue,
}
impl Break {
    pub fn get_id(&self) -> &u32 {
        self.id.get_value()
    }

    pub fn set_id(&mut self, value: u32) -> &mut Self {
        self.id.set_value(value);
        self
    }

    pub fn get_max(&self) -> &u32 {
        self.max.get_value()
    }

    pub fn set_max(&mut self, value: u32) -> &mut Self {
        self.max.set_value(value);
        self
    }

    pub fn get_manual_page_break(&self) -> &bool {
        self.manual_page_break.get_value()
    }

    pub fn set_manual_page_break(&mut self, value: bool) -> &mut Self {
        self.manual_page_break.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"id") {
            Some(v) => {
                self.id.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"max") {
            Some(v) => {
                self.max.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"min") {
            Some(v) => {
                self.min.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"man") {
            Some(v) => {
                self.manual_page_break.set_value_string(v);
            }
            None => {}
        }
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
