use super::EffectList;
use super::Scene3DType;
use super::Shape3DType;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use std::vec;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct EffectStyle {
    effect_list: Option<EffectList>,
    scene_3d_type: Option<Scene3DType>,
    shape_3d_type: Option<Shape3DType>,
}

impl EffectStyle {
    pub fn get_effect_list(&self) -> Option<&EffectList> {
        self.effect_list.as_ref()
    }

    pub fn get_effect_list_mut(&mut self) -> Option<&mut EffectList> {
        self.effect_list.as_mut()
    }

    pub fn set_effect_list(&mut self, value: EffectList) -> &mut Self {
        self.effect_list = Some(value);
        self
    }

    pub fn get_scene_3d_type(&self) -> Option<&Scene3DType> {
        self.scene_3d_type.as_ref()
    }

    pub fn get_scene_3d_type_mut(&mut self) -> Option<&mut Scene3DType> {
        self.scene_3d_type.as_mut()
    }

    pub fn set_scene_3d_type(&mut self, value: Scene3DType) -> &mut Self {
        self.scene_3d_type = Some(value);
        self
    }

    pub fn get_shape_3d_type(&self) -> Option<&Shape3DType> {
        self.shape_3d_type.as_ref()
    }

    pub fn get_shape_3d_type_mut(&mut self) -> Option<&mut Shape3DType> {
        self.shape_3d_type.as_mut()
    }

    pub fn set_shape_3d_type(&mut self, value: Shape3DType) -> &mut Self {
        self.shape_3d_type = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                b"a:effectLst" => {
                    let mut obj = EffectList::default();
                    obj.set_attributes(reader, e, false);
                    self.effect_list = Some(obj);
                }
                b"a:scene3d" => {
                    let mut obj = Scene3DType::default();
                    obj.set_attributes(reader, e);
                    self.scene_3d_type = Some(obj);
                }
                b"a:sp3d" => {
                    let mut obj = Shape3DType::default();
                    obj.set_attributes(reader, e);
                    self.shape_3d_type = Some(obj);
                }
                _ => (),
                }
            },
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:effectLst" {
                    let mut obj = EffectList::default();
                    obj.set_attributes(reader, e, true);
                    self.set_effect_list(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:effectStyle" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:effectStyle")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(writer, "a:effectStyle", vec![], false);

        // a:effectLst
        if let Some(v) = &self.effect_list {
            v.write_to(writer);
        }

        // a:scene3d
        if let Some(v) = &self.scene_3d_type {
            v.write_to(writer);
        }

        // a:sp3d
        if let Some(v) = &self.shape_3d_type {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:effectStyle");
    }
}
