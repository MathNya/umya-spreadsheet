// field
use crate::reader::driver::{get_attribute, set_string_from_xml};
use crate::structs::Int32Value;
use crate::writer::driver::write_start_tag;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Field {
    x: Int32Value,
}
impl Field {
    #[must_use]
    pub fn get_data_field(&self) -> i32 {
        self.x.get_value()
    }

    pub fn set_data_field(&mut self, value: i32) -> &mut Self {
        self.x.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, x, "x");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pivotField
        write_start_tag(
            writer,
            "field",
            vec![("x", self.x.get_value_string().as_str())],
            true,
        );
    }
}
