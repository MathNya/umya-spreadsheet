// a:rot
use super::super::Int32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rotation {
    latitude: Int32Value,
    longitude: Int32Value,
    revolution: Int32Value,
}
impl Rotation {
    pub fn get_latitude(&self) -> &i32 {
        self.latitude.get_value()
    }

    pub fn set_latitude(&mut self, value: i32) -> &mut Self {
        self.latitude.set_value(value);
        self
    }

    pub fn get_longitude(&self) -> &i32 {
        self.longitude.get_value()
    }

    pub fn set_longitude(&mut self, value: i32) -> &mut Self {
        self.longitude.set_value(value);
        self
    }

    pub fn get_revolution(&self) -> &i32 {
        self.revolution.get_value()
    }

    pub fn set_revolution(&mut self, value: i32) -> &mut Self {
        self.revolution.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"lat") {
            Some(v) => {
                self.latitude.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"lon") {
            Some(v) => {
                self.longitude.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"rev") {
            Some(v) => {
                self.revolution.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let latitude = self.latitude.get_value_string();
        if self.latitude.has_value() {
            attributes.push(("lat", &latitude));
        }
        let longitude = self.longitude.get_value_string();
        if self.longitude.has_value() {
            attributes.push(("lon", &longitude));
        }
        let revolution = self.revolution.get_value_string();
        if self.latitude.has_value() {
            attributes.push(("rev", &revolution));
        }
        write_start_tag(writer, "a:rot", attributes, true);
    }
}
