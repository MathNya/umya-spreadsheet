// a:camera
use super::super::EnumValue;
use super::PresetCameraValues;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Camera {
    preset: EnumValue<PresetCameraValues>,
}
impl Camera {
    pub fn get_preset(&self) -> &PresetCameraValues {
        self.preset.get_value()
    }

    pub fn set_preset(&mut self, value: PresetCameraValues) -> &mut Camera {
        self.preset.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"prst") {
            Some(v) => {
                self.preset.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:camera
        write_start_tag(
            writer,
            "a:camera",
            vec![("prst", self.preset.get_value_string())],
            true,
        );
    }
}
