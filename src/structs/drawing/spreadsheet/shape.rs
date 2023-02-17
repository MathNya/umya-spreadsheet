use super::super::super::Anchor;
use super::NonVisualShapeProperties;
use super::ShapeProperties;
use super::ShapeStyle;
use super::TextBody;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Shape {
    anchor: Anchor,
    non_visual_shape_properties: NonVisualShapeProperties,
    shape_properties: ShapeProperties,
    shape_style: Option<ShapeStyle>,
    text_body: Option<TextBody>,
}
impl Shape {
    pub fn get_anchor(&self) -> &Anchor {
        &self.anchor
    }

    pub fn get_anchor_mut(&mut self) -> &mut Anchor {
        &mut self.anchor
    }

    pub fn set_anchor(&mut self, value: Anchor) {
        self.anchor = value;
    }

    pub fn get_non_visual_shape_properties(&self) -> &NonVisualShapeProperties {
        &self.non_visual_shape_properties
    }

    pub fn get_non_visual_shape_properties_mut(&mut self) -> &mut NonVisualShapeProperties {
        &mut self.non_visual_shape_properties
    }

    pub fn set_non_visual_shape_properties(&mut self, value: NonVisualShapeProperties) {
        self.non_visual_shape_properties = value;
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

    pub fn get_shape_style(&self) -> &Option<ShapeStyle> {
        &self.shape_style
    }

    pub fn get_shape_style_mut(&mut self) -> &mut Option<ShapeStyle> {
        &mut self.shape_style
    }

    pub fn set_shape_style(&mut self, value: ShapeStyle) {
        self.shape_style = Some(value);
    }

    pub fn get_text_body(&self) -> &Option<TextBody> {
        &self.text_body
    }

    pub fn get_text_body_mut(&mut self) -> &mut Option<TextBody> {
        &mut self.text_body
    }

    pub fn set_text_body(&mut self, value: TextBody) {
        self.text_body = Some(value);
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
                    b"xdr:nvSpPr" => {
                        self.non_visual_shape_properties.set_attributes(reader, e);
                    }
                    b"xdr:spPr" => {
                        self.shape_properties.set_attributes(reader, e);
                    }
                    b"xdr:style" => {
                        let mut obj = ShapeStyle::default();
                        obj.set_attributes(reader, e);
                        self.set_shape_style(obj);
                    }
                    b"xdr:txBody" => {
                        let mut obj = TextBody::default();
                        obj.set_attributes(reader, e);
                        self.set_text_body(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xdr:sp" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:sp"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, ole_id: &usize) {
        // xdr:sp
        write_start_tag(
            writer,
            "xdr:sp",
            vec![("macro", ""), ("textlink", "")],
            false,
        );

        // xdr:nvSpPr
        let _ = &self.non_visual_shape_properties.write_to(writer, ole_id);

        // xdr:spPr
        let _ = &self.shape_properties.write_to(writer);

        // xdr:style
        match &self.shape_style {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // xdr:txBody
        match &self.text_body {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        write_end_tag(writer, "xdr:sp");
    }
}
