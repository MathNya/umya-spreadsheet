// item
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
        EnumValue,
        ItemValues,
        UInt32Value,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Item {
    index:                 UInt32Value,
    item_type:             EnumValue<ItemValues>,
}
impl Item {
    #[inline]
    #[must_use]
    pub fn index(&self) -> u32 {
        self.index.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use index()")]
    pub fn get_index(&self) -> u32 {
        self.index()
    }

    #[inline]
    pub fn set_index(&mut self, value: u32) -> &mut Self {
        self.index.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn item_type(&self) -> &ItemValues {
        self.item_type.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use item_type()")]
    pub fn get_item_type(&self) -> &ItemValues {
        self.item_type()
    }

    #[inline]
    pub fn set_item_type(&mut self, value: ItemValues) -> &mut Self {
        self.item_type.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, index, "x");
        set_string_from_xml!(self, e, item_type, "t");
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // item
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let index_str = self.index.value_string();
        if self.index.has_value() {
            attributes.push(("x", &index_str).into());
        }
        let item_type_str = self.item_type.value_string();
        if self.item_type.has_value() {
            attributes.push(("t", item_type_str).into());
        }
        write_start_tag(writer, "item", attributes, true);
    }
}
