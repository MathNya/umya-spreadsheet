// a:lin
use super::super::super::BooleanValue;
use super::super::super::Int32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct LinearGradientFill {
    angle: Int32Value,
    scaled: BooleanValue,
}
impl LinearGradientFill {
    pub fn get_angle(&self) -> &i32 {
        self.angle.get_value()
    }

    pub fn set_angle(&mut self, value: i32) -> &mut LinearGradientFill {
        self.angle.set_value(value);
        self
    }

    pub fn get_scaled(&self) -> &bool {
        self.scaled.get_value()
    }

    pub fn set_scaled(&mut self, value: bool) -> &mut LinearGradientFill {
        self.scaled.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"ang") {
            Some(v) => {
                self.angle.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"scaled") {
            Some(v) => {
                self.scaled.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lin
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let ang = self.angle.get_value_string();
        if &self.angle.has_value() == &true {
            attributes.push(("ang", &ang));
        }
        if &self.scaled.has_value() == &true {
            attributes.push(("scaled", self.scaled.get_value_string()));
        }
        write_start_tag(writer, "a:lin", attributes, true);
    }
}
