// a:sp3d
use super::super::EnumValue;
use super::BevelBottom;
use super::BevelTop;
use super::PresetMaterialTypeValues;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Shape3DType {
    preset_material: EnumValue<PresetMaterialTypeValues>,
    bevel_top: Option<BevelTop>,
    bevel_bottom: Option<BevelBottom>,
}

impl Shape3DType {
    pub fn get_preset_material(&self) -> &PresetMaterialTypeValues {
        self.preset_material.get_value()
    }

    pub fn set_preset_material(&mut self, value: PresetMaterialTypeValues) -> &mut Shape3DType {
        self.preset_material.set_value(value);
        self
    }

    pub fn get_bevel_top(&self) -> Option<&BevelTop> {
        self.bevel_top.as_ref()
    }

    pub fn get_bevel_top_mut(&mut self) -> Option<&mut BevelTop> {
        self.bevel_top.as_mut()
    }

    pub fn set_bevel_top(&mut self, value: BevelTop) {
        self.bevel_top = Some(value);
    }

    pub fn get_bevel_bottom(&self) -> Option<&BevelBottom> {
        self.bevel_bottom.as_ref()
    }

    pub fn get_bevel_bottom_mut(&mut self) -> Option<&mut BevelBottom> {
        self.bevel_bottom.as_mut()
    }

    pub fn set_bevel_bottom(&mut self, value: BevelBottom) {
        self.bevel_bottom = Some(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, preset_material, "prstMaterial");

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"a:bevelT" => {
                        let mut obj = BevelTop::default();
                        obj.set_attributes(reader, e);
                        self.set_bevel_top(obj);
                    }
                    b"a:bevelB" => {
                        let mut obj = BevelBottom::default();
                        obj.set_attributes(reader, e);
                        self.set_bevel_bottom(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:sp3d" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:sp3d")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:sp3d
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let preset_material = self.preset_material.get_value_string();
        if self.preset_material.has_value() {
            attributes.push(("prstMaterial", preset_material));
        }
        write_start_tag(writer, "a:sp3d", attributes, false);

        // a:bevelT
        if let Some(v) = &self.bevel_top {
            v.write_to(writer)
        }

        // a:bevelB
        if let Some(v) = &self.bevel_bottom {
            v.write_to(writer)
        }

        write_end_tag(writer, "a:sp3d");
    }
}
