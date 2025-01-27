// dataField
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    structs::{
        Int32Value,
        StringValue,
        UInt32Value,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct DataField {
    name:        StringValue,
    fie_id:      UInt32Value,
    base_fie_id: Int32Value,
    base_item:   UInt32Value,
}
impl DataField {
    #[inline]
    #[must_use]
    pub fn get_name(&self) -> &str {
        self.name.value_str()
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn set_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_fie_id(&self) -> u32 {
        self.fie_id.value()
    }

    #[inline]
    pub fn set_fie_id(&mut self, value: u32) -> &mut Self {
        self.fie_id.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_base_fie_id(&self) -> i32 {
        self.base_fie_id.value()
    }

    #[inline]
    pub fn set_base_fie_id(&mut self, value: i32) -> &mut Self {
        self.base_fie_id.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_base_item(&self) -> u32 {
        self.base_item.value()
    }

    #[inline]
    pub fn set_base_item(&mut self, value: u32) -> &mut Self {
        self.base_item.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, name, "name");
        set_string_from_xml!(self, e, fie_id, "fld");
        set_string_from_xml!(self, e, base_fie_id, "baseField");
        set_string_from_xml!(self, e, base_item, "baseItem");
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // dataField
        write_start_tag(
            writer,
            "dataField",
            vec![
                ("name", self.name.value_str()).into(),
                ("fld", self.fie_id.value_string().as_str()).into(),
                ("baseField", self.base_fie_id.value_string().as_str()).into(),
                ("baseItem", self.base_item.value_string().as_str()).into(),
            ],
            true,
        );
    }
}
