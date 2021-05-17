// xdr:spPr
use super::super::transform2d::Transform2D;
use super::super::preset_geometry::PresetGeometry;
use super::super::solid_fill::SolidFill;
use super::super::outline::Outline;
use super::super::effect_list::EffectList;
use writer::driver::*;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct ShapeProperties {
    transform2d: Transform2D,
    preset_geometry: PresetGeometry,
    solid_fill: Option<SolidFill>,
    outline: Option<Outline>,
    effect_list: Option<EffectList>,
}
impl ShapeProperties {
    pub fn get_geometry(&self) -> &PresetGeometry {
        &self.preset_geometry
    }

    pub fn get_geometry_mut(&mut self) -> &mut PresetGeometry {
        &mut self.preset_geometry
    }

    pub fn set_geometry(&mut self, value:PresetGeometry) {
        self.preset_geometry = value;
    }

    pub fn get_transform2d(&self) -> &Transform2D {
        &self.transform2d
    }

    pub fn get_transform2d_mut(&mut self) -> &mut Transform2D {
        &mut self.transform2d
    }

    pub fn set_transform2d(&mut self, value:Transform2D) {
        self.transform2d = value;
    }

    pub fn get_solid_fill(&self) -> &Option<SolidFill> {
        &self.solid_fill
    }

    pub fn get_solid_fill_mut(&mut self) -> &mut Option<SolidFill> {
        &mut self.solid_fill
    }

    pub fn set_solid_fill(&mut self, value:SolidFill) {
        self.solid_fill = Some(value);
    }

    pub fn get_outline(&self) -> &Option<Outline> {
        &self.outline
    }

    pub fn get_outline_mut(&mut self) -> &mut Option<Outline> {
        &mut self.outline
    }

    pub fn set_outline(&mut self, value:Outline) {
        self.outline = Some(value);
    }

    pub fn get_effect_list(&self) -> &Option<EffectList> {
        &self.effect_list
    }

    pub fn get_effect_list_mut(&mut self) -> &mut Option<EffectList> {
        &mut self.effect_list
    }

    pub fn set_effect_list(&mut self, value:EffectList) {
        self.effect_list = Some(value);
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"a:xfrm" => {
                            &mut self.transform2d.set_attributes(reader, e);
                        },
                        b"a:prstGeom" => {
                            &mut self.preset_geometry.set_attributes(reader, e);
                        },
                        b"a:ln" => {
                            let mut outline = Outline::default();
                            outline.set_attributes(reader, e);
                            &mut self.set_outline(outline);
                        },
                        b"a:solidFill" => {
                            let mut solid_fill = SolidFill::default();
                            solid_fill.set_attributes(reader, e);
                            &mut self.set_solid_fill(solid_fill);
                        },
                        b"a:effectLst" => {
                            let mut effect_list = EffectList::default();
                            effect_list.set_attributes(reader, e);
                            &mut self.set_effect_list(effect_list);
                        }
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"xdr:spPr" => return,
                        _ => (),
                    }
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
        &self.transform2d.write_to(writer, "a:xfrm");
    
        // a:prstGeom
        &self.preset_geometry.write_to(writer);
    
        // a:solidFill
        match &self.solid_fill {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }
    
        // a:ln
        match &self.outline {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }

        // a:effectLst
        match &self.effect_list {
            Some(v) => {
                v.write_to(writer);
            },
            None => {}
        }
    
        write_end_tag(writer, "xdr:spPr");
    }
}
