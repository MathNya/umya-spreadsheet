// xdr:pic
use super::super::super::Anchor;
use super::NonVisualPictureProperties;
use super::BlipFill;
use super::ShapeProperties;
use writer::driver::*;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;
use tempdir::TempDir;

#[derive(Default, Debug)]
pub struct Picture {
    anchor: Anchor,
    non_visual_picture_properties: NonVisualPictureProperties,
    blip_fill: BlipFill,
    shape_properties: ShapeProperties,
}
impl Picture {
    pub fn get_anchor(&self) -> &Anchor {
        &self.anchor
    }

    pub fn get_anchor_mut(&mut self) -> &mut Anchor {
        &mut self.anchor
    }

    pub fn set_anchor(&mut self, value:Anchor) {
        self.anchor = value;
    }

    pub fn get_non_visual_picture_properties(&self) -> &NonVisualPictureProperties {
        &self.non_visual_picture_properties
    }

    pub fn get_non_visual_picture_properties_mut(&mut self) -> &mut NonVisualPictureProperties {
        &mut self.non_visual_picture_properties
    }

    pub fn set_non_visual_picture_properties(&mut self, value:NonVisualPictureProperties) {
        self.non_visual_picture_properties = value;
    }

    pub fn get_blip_fill(&self) -> &BlipFill {
        &self.blip_fill
    }

    pub fn get_blip_fill_mut(&mut self) -> &mut BlipFill {
        &mut self.blip_fill
    }

    pub fn set_blip_fill(&mut self, value:BlipFill) {
        self.blip_fill = value;
    }

    pub fn get_shape_properties(&self) -> &ShapeProperties {
        &self.shape_properties
    }

    pub fn get_shape_properties_mut(&mut self) -> &mut ShapeProperties {
        &mut self.shape_properties
    }

    pub fn set_shape_properties(&mut self, value:ShapeProperties) {
        self.shape_properties = value;
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart,
        dir: &TempDir,
        target: &str,
    ) {
        let mut buf = Vec::new();
    
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"xdr:nvPicPr" => {
                            &mut self.non_visual_picture_properties.set_attributes(reader, e);
                        },
                        b"xdr:blipFill" => {
                            &mut self.blip_fill.set_attributes(reader, e, dir, target);
                        },
                        b"xdr:spPr" => {
                            &mut self.shape_properties.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"xdr:pic" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:pic"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &i32) {
        // xdr:pic
        write_start_tag(writer, "xdr:pic", vec![], false);

        // xdr:nvPicPr
        &self.non_visual_picture_properties.write_to(writer);

        // xdr:blipFill
        &self.blip_fill.write_to(writer, r_id);

        // xdr:spPr
        &self.shape_properties.write_to(writer);

        write_end_tag(writer, "xdr:pic");
    }
}
