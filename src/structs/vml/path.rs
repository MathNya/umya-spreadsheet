use super::office::ConnectValues;
use crate::reader::driver::*;
use crate::structs::EnumValue;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Path {
    connection_point_type: EnumValue<ConnectValues>,
}
impl Path {
    pub fn get_connection_point_type(&self) -> &ConnectValues {
        self.connection_point_type.get_value()
    }

    pub fn set_connection_point_type(&mut self, value: ConnectValues) -> &mut Self {
        self.connection_point_type.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, connection_point_type, "o:connecttype");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // v:path
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.connection_point_type.has_value() {
            attributes.push((
                "o:connecttype",
                self.connection_point_type.get_value_string(),
            ));
        }
        write_start_tag(writer, "v:path", attributes, true);
    }
}
