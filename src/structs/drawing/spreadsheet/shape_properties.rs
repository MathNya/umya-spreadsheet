// xdr:spPr
use super::super::BlipFill;
use super::super::EffectList;
use super::super::ExtensionList;
use super::super::NoFill;
use super::super::Outline;
use super::super::PresetGeometry;
use super::super::SolidFill;
use super::super::Transform2D;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::raw::RawRelationships;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ShapeProperties {
    transform2d: Option<Transform2D>,
    preset_geometry: PresetGeometry,
    blip_fill: Option<BlipFill>,
    solid_fill: Option<SolidFill>,
    outline: Option<Outline>,
    effect_list: Option<EffectList>,
    no_fill: Option<NoFill>,
    extension_list: Option<ExtensionList>,
}
impl ShapeProperties {
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

    pub fn get_geometry(&self) -> &PresetGeometry {
        &self.preset_geometry
    }

    pub fn get_geometry_mut(&mut self) -> &mut PresetGeometry {
        &mut self.preset_geometry
    }

    pub fn set_geometry(&mut self, value: PresetGeometry) -> &mut Self {
        self.preset_geometry = value;
        self
    }

    pub fn get_blip_fill(&self) -> Option<&BlipFill> {
        self.blip_fill.as_ref()
    }

    pub fn get_blip_fill_mut(&mut self) -> Option<&mut BlipFill> {
        self.blip_fill.as_mut()
    }

    pub fn set_blip_fill(&mut self, value: BlipFill) -> &mut Self {
        self.blip_fill = Some(value);
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

    pub fn get_extension_list(&self) -> Option<&ExtensionList> {
        self.extension_list.as_ref()
    }

    pub fn get_extension_list_mut(&mut self) -> Option<&mut ExtensionList> {
        self.extension_list.as_mut()
    }

    pub fn set_extension_list(&mut self, value: ExtensionList) -> &mut Self {
        self.extension_list = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"a:xfrm" => {
                        let mut obj = Transform2D::default();
                        obj.set_attributes(reader, e);
                        self.set_transform2d(obj);
                    }
                    b"a:prstGeom" => {
                        self.preset_geometry.set_attributes(reader, e);
                    }
                    b"a:blipFill" => {
                        let mut obj = BlipFill::default();
                        obj.set_attributes(reader, e, drawing_relationships);
                        self.set_blip_fill(obj);
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
                    b"a:extLst" => {
                        let mut obj = ExtensionList::default();
                        obj.set_attributes(reader, e);
                        self.set_extension_list(obj);
                    }
                    _ => (),
                }
            },
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:noFill" {
                    let mut obj = NoFill::default();
                    obj.set_attributes(reader, e);
                    self.set_no_fill(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:spPr" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:spPr")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // xdr:spPr
        write_start_tag(writer, "xdr:spPr", vec![], false);

        // a:xfrm
        if let Some(v) = &self.transform2d {
            v.write_to(writer);
        }

        // a:prstGeom
        self.preset_geometry.write_to(writer);

        // a:blipFill
        if let Some(v) = &self.blip_fill {
            v.write_to(writer, rel_list);
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

        write_end_tag(writer, "xdr:spPr");
    }
}
