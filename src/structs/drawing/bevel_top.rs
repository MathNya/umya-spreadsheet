// a:bevelT
use super::super::EnumValue;
use super::super::Int64Value;
use super::BevelPresetValues;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct BevelTop {
    width: Int64Value,
    height: Int64Value,
    preset: EnumValue<BevelPresetValues>,
}
impl BevelTop {
    pub fn get_width(&self) -> &i64 {
        self.width.get_value()
    }

    pub fn set_width(&mut self, value: i64) -> &mut BevelTop {
        self.width.set_value(value);
        self
    }

    pub fn get_height(&self) -> &i64 {
        self.height.get_value()
    }

    pub fn set_height(&mut self, value: i64) -> &mut BevelTop {
        self.height.set_value(value);
        self
    }

    pub fn get_preset(&self) -> &BevelPresetValues {
        self.preset.get_value()
    }

    pub fn set_preset(&mut self, value: BevelPresetValues) -> &mut BevelTop {
        self.preset.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"w") {
            Some(v) => {
                self.width.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"h") {
            Some(v) => {
                self.height.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"prst") {
            Some(v) => {
                self.preset.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:bevelT
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let width = self.width.get_value_string();
        if &self.width.has_value() == &true {
            attributes.push(("w", &width));
        }
        let height = self.height.get_value_string();
        if &self.height.has_value() == &true {
            attributes.push(("h", &height));
        }
        if &self.preset.has_value() == &true {
            attributes.push(("prst", self.preset.get_value_string()));
        }

        write_start_tag(writer, "a:bevelT", attributes, true);
    }
}
