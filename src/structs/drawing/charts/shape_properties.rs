// c:spPr
use super::super::EffectList;
use super::super::NoFill;
use super::super::Outline;
use super::super::PatternFill;
use super::super::PresetGeometry;
use super::super::Scene3DType;
use super::super::Shape3DType;
use super::super::SolidFill;
use super::super::Transform2D;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ShapeProperties {
    pattern_fill: Option<PatternFill>,
    transform2d: Option<Transform2D>,
    preset_geometry: Option<PresetGeometry>,
    solid_fill: Option<SolidFill>,
    no_fill: Option<NoFill>,
    outline: Option<Outline>,
    effect_list: Option<EffectList>,
    scene_3d_type: Option<Scene3DType>,
    shape_3d_type: Option<Shape3DType>,
}

impl ShapeProperties {
    pub fn get_pattern_fill(&self) -> Option<&PatternFill> {
        self.pattern_fill.as_ref()
    }

    pub fn get_pattern_fill_mut(&mut self) -> Option<&mut PatternFill> {
        self.pattern_fill.as_mut()
    }

    pub fn set_pattern_fill(&mut self, value: PatternFill) -> &mut Self {
        self.pattern_fill = Some(value);
        self
    }

    pub fn get_transform2d(&self) -> Option<&Transform2D> {
        self.transform2d.as_ref()
    }

    pub fn get_transform2d_mut(&mut self) -> Option<&mut Transform2D> {
        self.transform2d.as_mut()
    }

    pub fn set_transform2d(&mut self, value: Transform2D) -> &mut Self {
        self.transform2d = Some(value);
        self
    }

    pub fn get_geometry(&self) -> Option<&PresetGeometry> {
        self.preset_geometry.as_ref()
    }

    pub fn get_geometry_mut(&mut self) -> Option<&mut PresetGeometry> {
        self.preset_geometry.as_mut()
    }

    pub fn set_geometry(&mut self, value: PresetGeometry) -> &mut Self {
        self.preset_geometry = Some(value);
        self
    }

    pub fn get_solid_fill(&self) -> Option<&SolidFill> {
        self.solid_fill.as_ref()
    }

    pub fn get_solid_fill_mut(&mut self) -> Option<&mut SolidFill> {
        self.solid_fill.as_mut()
    }

    pub fn set_solid_fill(&mut self, value: SolidFill) -> &mut Self {
        self.solid_fill = Some(value);
        self
    }

    pub fn get_no_fill(&self) -> Option<&NoFill> {
        self.no_fill.as_ref()
    }

    pub fn get_no_fill_mut(&mut self) -> Option<&mut NoFill> {
        self.no_fill.as_mut()
    }

    pub fn set_no_fill(&mut self, value: NoFill) -> &mut Self {
        self.no_fill = Some(value);
        self
    }

    pub fn get_outline(&self) -> Option<&Outline> {
        self.outline.as_ref()
    }

    pub fn get_outline_mut(&mut self) -> Option<&mut Outline> {
        self.outline.as_mut()
    }

    pub fn set_outline(&mut self, value: Outline) -> &mut Self {
        self.outline = Some(value);
        self
    }

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
                match e.name().0 {
                    b"a:pattFill" => {
                        let mut obj = PatternFill::default();
                        obj.set_attributes(reader, e);
                        self.set_pattern_fill(obj);
                    }
                    b"a:xfrm" => {
                        let mut obj = Transform2D::default();
                        obj.set_attributes(reader, e);
                        self.set_transform2d(obj);
                    }
                    b"a:prstGeom" => {
                        let mut obj = PresetGeometry::default();
                        obj.set_attributes(reader, e);
                        self.set_geometry(obj);
                    }
                    b"a:ln" => {
                        let mut obj = Outline::default();
                        obj.set_attributes(reader, e);
                        self.set_outline(obj);
                    }
                    b"a:solidFill" => {
                        let mut obj = SolidFill::default();
                        obj.set_attributes(reader, e);
                        self.set_solid_fill(obj);
                    }
                    b"a:effectLst" => {
                        let mut obj = EffectList::default();
                        obj.set_attributes(reader, e, false);
                        self.set_effect_list(obj);
                    }
                    b"a:scene3d" => {
                        let mut obj = Scene3DType::default();
                        obj.set_attributes(reader, e);
                        self.set_scene_3d_type(obj);
                    }
                    b"a:sp3d" => {
                        let mut obj = Shape3DType::default();
                        obj.set_attributes(reader, e);
                        self.set_shape_3d_type(obj);
                    }
                    _ => (),
                }
            },
            Event::Empty(ref e) => {
                if e.name().0 == b"a:noFill" {
                    let mut obj = NoFill::default();
                    obj.set_attributes(reader, e);
                    self.set_no_fill(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:spPr" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:spPr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:spPr
        write_start_tag(writer, "c:spPr", vec![], false);

        // a:pattFill
        if let Some(v) = &self.pattern_fill {
            v.write_to(writer);
        }

        // a:xfrm
        if let Some(v) = &self.transform2d {
            v.write_to(writer);
        }

        // a:prstGeom
        if let Some(v) = &self.preset_geometry {
            v.write_to(writer);
        }

        // a:solidFill
        if let Some(v) = &self.solid_fill {
            v.write_to(writer);
        }

        // a:noFill
        if let Some(v) = &self.no_fill {
            v.write_to(writer);
        }

        // a:ln
        if let Some(v) = &self.outline {
            v.write_to(writer);
        }

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

        write_end_tag(writer, "c:spPr");
    }
}
