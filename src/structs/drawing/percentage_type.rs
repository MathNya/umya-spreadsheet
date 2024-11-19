use super::super::Int32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PercentageType {
    val: Int32Value,
}

impl PercentageType {
    #[inline]
    pub fn get_val(&self) -> &i32 {
        self.val.get_value()
    }

    #[inline]
    pub fn set_val(&mut self, value: i32) -> &mut Self {
        self.val.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, val, "val");
    }

    #[inline]
    pub(crate) fn write_to_lum(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lum")
    }

    #[inline]
    pub(crate) fn write_to_lum_mod(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lumMod")
    }

    #[inline]
    pub(crate) fn write_to_lum_off(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lumOff")
    }

    #[inline]
    pub(crate) fn write_to_sat(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:sat")
    }

    #[inline]
    pub(crate) fn write_to_sat_mod(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:satMod")
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tab_name: &str) {
        if self.val.has_value() {
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            let val = self.val.get_value_string();
            attributes.push(("val", &val));
            write_start_tag(writer, tab_name, attributes, true);
        }
    }
}
