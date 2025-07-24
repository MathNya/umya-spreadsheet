// a:lin
use super::super::super::BooleanValue;
use super::super::super::Int32Value;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct LinearGradientFill {
    angle: Int32Value,
    scaled: BooleanValue,
}

impl LinearGradientFill {
    #[inline]
    pub fn get_angle(&self) -> &i32 {
        self.angle.get_value()
    }

    #[inline]
    pub fn set_angle(&mut self, value: i32) -> &mut LinearGradientFill {
        self.angle.set_value(value);
        self
    }

    #[inline]
    pub fn get_scaled(&self) -> &bool {
        self.scaled.get_value()
    }

    #[inline]
    pub fn set_scaled(&mut self, value: bool) -> &mut LinearGradientFill {
        self.scaled.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, angle, "ang");
        set_string_from_xml!(self, e, scaled, "scaled");

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:lin" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:lin")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lin
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let ang = self.angle.get_value_string();
        if self.angle.has_value() {
            attributes.push(("ang", &ang));
        }
        if self.scaled.has_value() {
            attributes.push(("scaled", self.scaled.get_value_string()));
        }
        write_start_tag(writer, "a:lin", attributes, true);
    }
}
