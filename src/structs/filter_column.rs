// filterColumn
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    BooleanValue,
    UInt32Value,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct FilterColumn {
    column_id: UInt32Value,
    hidden_button: BooleanValue,
}

impl FilterColumn {
    #[inline]
    #[must_use]
    pub fn column_id(&self) -> u32 {
        self.column_id.value()
    }

    #[inline]
    pub fn set_font(&mut self, value: u32) -> &mut Self {
        self.column_id.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn hidden_button(&self) -> bool {
        self.hidden_button.value()
    }

    #[inline]
    pub fn set_hidden_button(&mut self, value: bool) -> &mut Self {
        self.hidden_button.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, column_id, "colId");
        set_string_from_xml!(self, e, hidden_button, "hiddenButton");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.column_id.has_value() {
            attributes.push(("colId", self.column_id.value_string()).into());
        }
        if self.hidden_button.has_value() {
            attributes.push(("hiddenButton", self.hidden_button.value_string()).into());
        }

        // filterColumn
        write_start_tag(
            writer,
            "filterColumn",
            attributes,
            true,
        );
    }
}
