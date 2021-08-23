// a:camera
use super::PresetCameraValues;
use super::super::EnumValue;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Camera {
    preset: EnumValue<PresetCameraValues>,
}
impl Camera {
    pub fn get_preset(&self) -> &PresetCameraValues {
        &self.preset.get_value()
    }

    pub fn set_preset(&mut self, value:PresetCameraValues) -> &mut Camera {
        self.preset.set_value(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        match get_attribute(e, b"prst") {
            Some(v) => {&mut self.preset.set_value_string(v);},
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:camera
        write_start_tag(writer, "a:camera", vec![
            ("prst", &self.preset.get_value_string())
        ], true);
    }
}
