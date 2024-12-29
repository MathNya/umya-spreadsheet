// a:bevelB
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    super::{
        EnumValue,
        Int64Value,
    },
    BevelPresetValues,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct BevelBottom {
    width:  Int64Value,
    height: Int64Value,
    preset: EnumValue<BevelPresetValues>,
}

impl BevelBottom {
    #[must_use]
    pub fn get_width(&self) -> i64 {
        self.width.get_value()
    }

    pub fn set_width(&mut self, value: i64) -> &mut BevelBottom {
        self.width.set_value(value);
        self
    }

    #[must_use]
    pub fn get_height(&self) -> i64 {
        self.height.get_value()
    }

    pub fn set_height(&mut self, value: i64) -> &mut BevelBottom {
        self.height.set_value(value);
        self
    }

    #[must_use]
    pub fn get_preset(&self) -> &BevelPresetValues {
        self.preset.get_value()
    }

    pub fn set_preset(&mut self, value: BevelPresetValues) -> &mut BevelBottom {
        self.preset.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, width, "w");
        set_string_from_xml!(self, e, height, "h");
        set_string_from_xml!(self, e, preset, "prst");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:bevelB
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let width = self.width.get_value_string();
        if self.width.has_value() {
            attributes.push(("w", &width).into());
        }
        let height = self.height.get_value_string();
        if self.height.has_value() {
            attributes.push(("h", &height).into());
        }
        if self.preset.has_value() {
            attributes.push(("prst", self.preset.get_value_string()).into());
        }

        write_start_tag(writer, "a:bevelB", attributes, true);
    }
}
