// sharedItems
use structs::BooleanValue;
use structs::DoubleValue;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct SharedItems {
    contains_semi_mixed_types: BooleanValue,
    contains_string: BooleanValue,
    contains_number: BooleanValue,
    contains_integer: BooleanValue,
    min_value: DoubleValue,
    max_value: DoubleValue,
}
impl SharedItems {
    pub fn get_contains_semi_mixed_types(&self) -> &bool {
        self.contains_semi_mixed_types.get_value()
    }

    pub fn set_contains_semi_mixed_types(&mut self, value: bool) -> &mut Self {
        self.contains_semi_mixed_types.set_value(value);
        self
    }

    pub fn get_contains_string(&self) -> &bool {
        self.contains_string.get_value()
    }

    pub fn set_contains_string(&mut self, value: bool) -> &mut Self {
        self.contains_string.set_value(value);
        self
    }

    pub fn get_contains_number(&self) -> &bool {
        self.contains_number.get_value()
    }

    pub fn set_contains_number(&mut self, value: bool) -> &mut Self {
        self.contains_number.set_value(value);
        self
    }

    pub fn get_contains_integer(&self) -> &bool {
        self.contains_integer.get_value()
    }

    pub fn set_contains_integer(&mut self, value: bool) -> &mut Self {
        self.contains_integer.set_value(value);
        self
    }

    pub fn get_min_value(&self) -> &f64 {
        self.min_value.get_value()
    }

    pub fn set_min_value(&mut self, value: f64) -> &mut Self {
        self.min_value.set_value(value);
        self
    }

    pub fn get_max_value(&self) -> &f64 {
        self.max_value.get_value()
    }

    pub fn set_max_value(&mut self, value: f64) -> &mut Self {
        self.max_value.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, contains_semi_mixed_types, "containsSemiMixedTypes");
        set_string_from_xml!(self, e, contains_string, "containsString");
        set_string_from_xml!(self, e, contains_number, "containsNumber");
        set_string_from_xml!(self, e, contains_integer, "containsInteger");
        set_string_from_xml!(self, e, min_value, "minValue");
        set_string_from_xml!(self, e, max_value, "maxValue");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // sharedItems
        write_start_tag(writer, "sharedItems", vec![
            ("containsSemiMixedTypes", self.contains_semi_mixed_types.get_value_string()),
            ("containsString", self.contains_string.get_value_string()),
            ("containsNumber", self.contains_number.get_value_string()),
            ("containsInteger", self.contains_integer.get_value_string()),
            ("minValue", self.min_value.get_value_string().as_str()),
            ("maxValue", self.max_value.get_value_string().as_str()),
        ], true);
    }
}
