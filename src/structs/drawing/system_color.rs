// a:sysClr
use super::super::super::EnumValue;
use super::super::super::StringValue;
use super::SystemColorValues;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct SystemColor {
    val: EnumValue<SystemColorValues>,
    last_color: StringValue,
}
impl SystemColor {
    pub fn get_val(&self) -> &SystemColorValues {
        self.val.get_value()
    }

    pub fn set_val(&mut self, value: SystemColorValues) -> &mut Self {
        self.val.set_value(value);
        self
    }

    pub fn get_last_color(&self) -> &str {
        self.last_color.get_value()
    }

    pub fn set_last_color<S: Into<String>>(&mut self, value: S) {
        self.last_color.set_value(value.into());
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"val") {
            Some(v) => {
                self.val.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"lastClr") {
            Some(v) => {
                self.last_color.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:srgbClr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let val = self.val.get_value_string();
        if self.val.has_value() {
            attributes.push(("val", val));
        }
        let last_color = self.last_color.get_value_string();
        if self.last_color.has_value() {
            attributes.push(("lastClr", last_color));
        }
        write_start_tag(writer, "a:sysClr", attributes, true);
    }
}
