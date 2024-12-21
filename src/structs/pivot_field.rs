// pivotField
use crate::reader::driver::{get_attribute, set_string_from_xml};
use crate::structs::BooleanValue;
use crate::writer::driver::write_start_tag;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct PivotField {
    data_field: BooleanValue,
    show_all: BooleanValue,
}
impl PivotField {
    #[inline]
    #[must_use]
    pub fn get_data_field(&self) -> bool {
        self.data_field.get_value()
    }

    #[inline]
    pub fn set_data_field(&mut self, value: bool) -> &mut Self {
        self.data_field.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_show_all(&self) -> bool {
        self.show_all.get_value()
    }

    #[inline]
    pub fn set_show_all(&mut self, value: bool) -> &mut Self {
        self.show_all.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, data_field, "dataField");
        set_string_from_xml!(self, e, show_all, "showAll");
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // pivotField
        write_start_tag(
            writer,
            "pivotField",
            vec![
                ("dataField", self.data_field.get_value_string()),
                ("showAll", self.show_all.get_value_string()),
            ],
            true,
        );
    }
}
