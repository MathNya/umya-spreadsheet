// a:rot
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::super::Int32Value;
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rotation {
    latitude:   Int32Value,
    longitude:  Int32Value,
    revolution: Int32Value,
}

impl Rotation {
    #[must_use]
    pub fn get_latitude(&self) -> i32 {
        self.latitude.get_value()
    }

    pub fn set_latitude(&mut self, value: i32) -> &mut Self {
        self.latitude.set_value(value);
        self
    }

    #[must_use]
    pub fn get_longitude(&self) -> i32 {
        self.longitude.get_value()
    }

    pub fn set_longitude(&mut self, value: i32) -> &mut Self {
        self.longitude.set_value(value);
        self
    }

    #[must_use]
    pub fn get_revolution(&self) -> i32 {
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
        set_string_from_xml!(self, e, latitude, "lat");
        set_string_from_xml!(self, e, longitude, "lon");
        set_string_from_xml!(self, e, revolution, "rev");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let latitude = self.latitude.get_value_string();
        if self.latitude.has_value() {
            attributes.push(("lat", &latitude).into());
        }
        let longitude = self.longitude.get_value_string();
        if self.longitude.has_value() {
            attributes.push(("lon", &longitude).into());
        }
        let revolution = self.revolution.get_value_string();
        if self.latitude.has_value() {
            attributes.push(("rev", &revolution).into());
        }
        write_start_tag(writer, "a:rot", attributes, true);
    }
}
