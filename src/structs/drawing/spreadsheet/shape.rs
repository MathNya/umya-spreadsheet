use super::super::super::anchor::Anchor;
use super::non_visual_shape_properties::NonVisualShapeProperties;
use super::shape_properties::ShapeProperties;
use super::shape_style::ShapeStyle;
use super::text_body::TextBody;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Shape {
    anchor: Anchor,
    non_visual_shape_properties: NonVisualShapeProperties,
    shape_properties: ShapeProperties,
    shape_style: ShapeStyle,
    text_body: TextBody,
}
impl Shape {
    pub fn get_anchor(&self) -> &Anchor {
        &self.anchor
    }

    pub fn get_anchor_mut(&mut self) -> &mut Anchor {
        &mut self.anchor
    }

    pub fn set_anchor(&mut self, value:Anchor) {
        self.anchor = value;
    }

    pub fn get_non_visual_shape_properties(&self) -> &NonVisualShapeProperties {
        &self.non_visual_shape_properties
    }

    pub fn get_non_visual_shape_properties_mut(&mut self) -> &mut NonVisualShapeProperties {
        &mut self.non_visual_shape_properties
    }

    pub fn set_non_visual_shape_properties(&mut self, value:NonVisualShapeProperties) {
        self.non_visual_shape_properties = value;
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

    pub fn get_shape_style(&self) -> &ShapeStyle {
        &self.shape_style
    }

    pub fn get_shape_style_mut(&mut self) -> &mut ShapeStyle {
        &mut self.shape_style
    }

    pub fn set_shape_style(&mut self, value:ShapeStyle) {
        self.shape_style = value;
    }

    pub fn get_text_body(&self) -> &TextBody {
        &self.text_body
    }

    pub fn get_text_body_mut(&mut self) -> &mut TextBody {
        &mut self.text_body
    }

    pub fn set_text_body(&mut self, value:TextBody) {
        self.text_body = value;
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
                        b"xdr:nvSpPr" => { &mut self.non_visual_shape_properties.set_attributes(reader, e); }
                        b"xdr:spPr" => { &mut self.shape_properties.set_attributes(reader, e); },
                        b"xdr:style" => { &mut self.shape_style.set_attributes(reader, e); },
                        b"xdr:txBody" => { &mut self.text_body.set_attributes(reader, e); },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"xdr:sp" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:sp"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:sp
        write_start_tag(writer, "xdr:sp", vec![
            ("macro", ""),
            ("textlink", ""),
        ], false);

        // xdr:nvSpPr
        &self.non_visual_shape_properties.write_to(writer);

        // xdr:spPr
        &self.shape_properties.write_to(writer);

        // xdr:style
        &self.shape_style.write_to(writer);

        // xdr:txBody
        &self.text_body.write_to(writer);

        write_end_tag(writer, "xdr:sp");
    }
}
