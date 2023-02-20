// xdr:spPr
use super::super::EffectList;
use super::super::NoFill;
use super::super::Outline;
use super::super::PresetGeometry;
use super::super::SolidFill;
use super::super::Transform2D;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ShapeProperties {
    transform2d: Option<Transform2D>,
    preset_geometry: PresetGeometry,
    solid_fill: Option<SolidFill>,
    outline: Option<Outline>,
    effect_list: Option<EffectList>,
    no_fill: Option<NoFill>,
}
impl ShapeProperties {
    pub fn get_geometry(&self) -> &PresetGeometry {
        &self.preset_geometry
    }

    pub fn get_geometry_mut(&mut self) -> &mut PresetGeometry {
        &mut self.preset_geometry
    }

    pub fn set_geometry(&mut self, value: PresetGeometry) -> &mut ShapeProperties {
        self.preset_geometry = value;
        self
    }

    pub fn get_transform2d(&self) -> &Option<Transform2D> {
        &self.transform2d
    }

    pub fn get_transform2d_mut(&mut self) -> &mut Option<Transform2D> {
        &mut self.transform2d
    }

    pub fn set_transform2d(&mut self, value: Transform2D) -> &mut ShapeProperties {
        self.transform2d = Some(value);
        self
    }

    pub fn get_solid_fill(&self) -> &Option<SolidFill> {
        &self.solid_fill
    }

    pub fn get_solid_fill_mut(&mut self) -> &mut Option<SolidFill> {
        &mut self.solid_fill
    }

    pub fn set_solid_fill(&mut self, value: SolidFill) -> &mut ShapeProperties {
        self.solid_fill = Some(value);
        self
    }

    pub fn get_outline(&self) -> &Option<Outline> {
        &self.outline
    }

    pub fn get_outline_mut(&mut self) -> &mut Option<Outline> {
        &mut self.outline
    }

    pub fn set_outline(&mut self, value: Outline) -> &mut ShapeProperties {
        self.outline = Some(value);
        self
    }

    pub fn get_effect_list(&self) -> &Option<EffectList> {
        &self.effect_list
    }

    pub fn get_effect_list_mut(&mut self) -> &mut Option<EffectList> {
        &mut self.effect_list
    }

    pub fn set_effect_list(&mut self, value: EffectList) -> &mut ShapeProperties {
        self.effect_list = Some(value);
        self
    }

    pub fn get_no_fill(&self) -> &Option<NoFill> {
        &self.no_fill
    }

    pub fn get_no_fill_mut(&mut self) -> &mut Option<NoFill> {
        &mut self.no_fill
    }

    pub fn set_no_fill(&mut self, value: NoFill) -> &mut ShapeProperties {
        self.no_fill = Some(value);
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
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:xfrm" => {
                        let mut obj = Transform2D::default();
                        obj.set_attributes(reader, e);
                        self.set_transform2d(obj);
                    }
                    b"a:prstGeom" => {
                        self.preset_geometry.set_attributes(reader, e);
                    }
                    b"a:ln" => {
                        let mut outline = Outline::default();
                        outline.set_attributes(reader, e);
                        self.set_outline(outline);
                    }
                    b"a:solidFill" => {
                        let mut solid_fill = SolidFill::default();
                        solid_fill.set_attributes(reader, e);
                        self.set_solid_fill(solid_fill);
                    }
                    b"a:effectLst" => {
                        let mut effect_list = EffectList::default();
                        effect_list.set_attributes(reader, e, false);
                        self.set_effect_list(effect_list);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:noFill" => {
                        let mut obj = NoFill::default();
                        obj.set_attributes(reader, e);
                        self.set_no_fill(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xdr:spPr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:spPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:spPr
        write_start_tag(writer, "xdr:spPr", vec![], false);

        // a:xfrm
        match &self.transform2d {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:prstGeom
        let _ = &self.preset_geometry.write_to(writer);

        // a:solidFill
        match &self.solid_fill {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:noFill
        match &self.no_fill {
            Some(v) => v.write_to(writer),
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

        write_end_tag(writer, "xdr:spPr");
    }
}
