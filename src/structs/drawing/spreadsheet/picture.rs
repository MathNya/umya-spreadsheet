// xdr:pic
use super::BlipFill;
use super::NonVisualPictureProperties;
use super::ShapeProperties;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use structs::raw::RawRelationships;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Picture {
    non_visual_picture_properties: NonVisualPictureProperties,
    blip_fill: BlipFill,
    shape_properties: ShapeProperties,
}
impl Picture {
    pub fn get_non_visual_picture_properties(&self) -> &NonVisualPictureProperties {
        &self.non_visual_picture_properties
    }

    pub fn get_non_visual_picture_properties_mut(&mut self) -> &mut NonVisualPictureProperties {
        &mut self.non_visual_picture_properties
    }

    pub fn set_non_visual_picture_properties(&mut self, value: NonVisualPictureProperties) {
        self.non_visual_picture_properties = value;
    }

    pub fn get_blip_fill(&self) -> &BlipFill {
        &self.blip_fill
    }

    pub fn get_blip_fill_mut(&mut self) -> &mut BlipFill {
        &mut self.blip_fill
    }

    pub fn set_blip_fill(&mut self, value: BlipFill) {
        self.blip_fill = value;
    }

    pub fn get_shape_properties(&self) -> &ShapeProperties {
        &self.shape_properties
    }

    pub fn get_shape_properties_mut(&mut self) -> &mut ShapeProperties {
        &mut self.shape_properties
    }

    pub fn set_shape_properties(&mut self, value: ShapeProperties) {
        self.shape_properties = value;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"xdr:nvPicPr" => {
                        self.non_visual_picture_properties.set_attributes(reader, e);
                    }
                    b"xdr:blipFill" => {
                        self.blip_fill
                            .set_attributes(reader, e, drawing_relationships);
                    }
                    b"xdr:spPr" => {
                        self.shape_properties.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xdr:pic" => return,
                    _ => (),
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
        let _ = &self.non_visual_picture_properties.write_to(writer);

        // xdr:blipFill
        let _ = &self.blip_fill.write_to(writer, r_id);

        // xdr:spPr
        let _ = &self.shape_properties.write_to(writer);

        write_end_tag(writer, "xdr:pic");
    }
}
