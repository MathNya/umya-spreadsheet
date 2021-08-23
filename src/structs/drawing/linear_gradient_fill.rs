// a:lin
use super::super::super::Int32Value;
use super::super::super::BooleanValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct LinearGradientFill {
    angle: Int32Value,
    scaled: BooleanValue,
}
impl LinearGradientFill {
    pub fn get_angle(&self)-> &i32 {
        &self.angle.get_value()
    }
    
    pub fn set_angle(&mut self, value:i32)-> &mut LinearGradientFill {
        self.angle.set_value(value);
        self
    }

    pub fn get_scaled(&self)-> &bool {
        &self.scaled.get_value()
    }
    
    pub fn set_scaled(&mut self, value:bool)-> &mut LinearGradientFill {
        self.scaled.set_value(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        self.angle.set_value_string(get_attribute(e, b"ang").unwrap());
        self.scaled.set_value_string(get_attribute(e, b"scaled").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lin
        write_start_tag(writer, "a:lin", vec![
            ("ang", &self.angle.get_value_string()),
            ("scaled", &self.scaled.get_value_string()),
        ], true);
    }
}
