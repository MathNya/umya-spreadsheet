// a:bevelB
use super::super::EnumValue;
use super::super::Int64Value;
use super::BevelPresetValues;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct BevelBottom {
    width: Int64Value,
    height: Int64Value,
    preset: EnumValue<BevelPresetValues>,
}

impl BevelBottom {
    #[inline]
    pub fn get_width(&self) -> &i64 {
        self.width.get_value()
    }

    #[inline]
    pub fn set_width(&mut self, value: i64) -> &mut BevelBottom {
        self.width.set_value(value);
        self
    }

    #[inline]
    pub fn get_height(&self) -> &i64 {
        self.height.get_value()
    }

    #[inline]
    pub fn set_height(&mut self, value: i64) -> &mut BevelBottom {
        self.height.set_value(value);
        self
    }

    #[inline]
    pub fn get_preset(&self) -> &BevelPresetValues {
        self.preset.get_value()
    }

    #[inline]
    pub fn set_preset(&mut self, value: BevelPresetValues) -> &mut BevelBottom {
        self.preset.set_value(value);
        self
    }

    #[inline]
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
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let width = self.width.get_value_string();
        if self.width.has_value() {
            attributes.push(("w", &width));
        }
        let height = self.height.get_value_string();
        if self.height.has_value() {
            attributes.push(("h", &height));
        }
        if self.preset.has_value() {
            attributes.push(("prst", self.preset.get_value_string()));
        }

        write_start_tag(writer, "a:bevelB", attributes, true);
    }
}
