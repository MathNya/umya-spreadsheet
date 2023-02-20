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
    pub fn get_pattern_fill(&self) -> &Option<PatternFill> {
        &self.pattern_fill
    }

    pub fn get_pattern_fill_mut(&mut self) -> &mut Option<PatternFill> {
        &mut self.pattern_fill
    }

    pub fn set_pattern_fill(&mut self, value: PatternFill) -> &mut Self {
        self.pattern_fill = Some(value);
        self
    }

    pub fn get_transform2d(&self) -> &Option<Transform2D> {
        &self.transform2d
    }

    pub fn get_transform2d_mut(&mut self) -> &mut Option<Transform2D> {
        &mut self.transform2d
    }

    pub fn set_transform2d(&mut self, value: Transform2D) -> &mut Self {
        self.transform2d = Some(value);
        self
    }

    pub fn get_geometry(&self) -> &Option<PresetGeometry> {
        &self.preset_geometry
    }

    pub fn get_geometry_mut(&mut self) -> &mut Option<PresetGeometry> {
        &mut self.preset_geometry
    }

    pub fn set_geometry(&mut self, value: PresetGeometry) -> &mut Self {
        self.preset_geometry = Some(value);
        self
    }

    pub fn get_solid_fill(&self) -> &Option<SolidFill> {
        &self.solid_fill
    }

    pub fn get_solid_fill_mut(&mut self) -> &mut Option<SolidFill> {
        &mut self.solid_fill
    }

    pub fn set_solid_fill(&mut self, value: SolidFill) -> &mut Self {
        self.solid_fill = Some(value);
        self
    }

    pub fn get_no_fill(&self) -> &Option<NoFill> {
        &self.no_fill
    }

    pub fn get_no_fill_mut(&mut self) -> &mut Option<NoFill> {
        &mut self.no_fill
    }

    pub fn set_no_fill(&mut self, value: NoFill) -> &mut Self {
        self.no_fill = Some(value);
        self
    }

    pub fn get_outline(&self) -> &Option<Outline> {
        &self.outline
    }

    pub fn get_outline_mut(&mut self) -> &mut Option<Outline> {
        &mut self.outline
    }

    pub fn set_outline(&mut self, value: Outline) -> &mut Self {
        self.outline = Some(value);
        self
    }

    pub fn get_effect_list(&self) -> &Option<EffectList> {
        &self.effect_list
    }

    pub fn get_effect_list_mut(&mut self) -> &mut Option<EffectList> {
        &mut self.effect_list
    }

    pub fn set_effect_list(&mut self, value: EffectList) -> &mut Self {
        self.effect_list = Some(value);
        self
    }

    pub fn get_scene_3d_type(&self) -> &Option<Scene3DType> {
        &self.scene_3d_type
    }

    pub fn get_scene_3d_type_mut(&mut self) -> &mut Option<Scene3DType> {
        &mut self.scene_3d_type
    }

    pub fn set_scene_3d_type(&mut self, value: Scene3DType) -> &mut Self {
        self.scene_3d_type = Some(value);
        self
    }

    pub fn get_shape_3d_type(&self) -> &Option<Shape3DType> {
        &self.shape_3d_type
    }

    pub fn get_shape_3d_type_mut(&mut self) -> &mut Option<Shape3DType> {
        &mut self.shape_3d_type
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
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().0 {
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
                },
                Ok(Event::Empty(ref e)) => match e.name().0 {
                    b"a:noFill" => {
                        let mut obj = NoFill::default();
                        obj.set_attributes(reader, e);
                        self.set_no_fill(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:spPr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:spPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:spPr
        write_start_tag(writer, "c:spPr", vec![], false);

        // a:pattFill
        match &self.pattern_fill {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:xfrm
        match &self.transform2d {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:prstGeom
        match &self.preset_geometry {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:solidFill
        match &self.solid_fill {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:noFill
        match &self.no_fill {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:ln
        match &self.outline {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:effectLst
        match &self.effect_list {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:scene3d
        match &self.scene_3d_type {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:sp3d
        match &self.shape_3d_type {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "c:spPr");
    }
}
