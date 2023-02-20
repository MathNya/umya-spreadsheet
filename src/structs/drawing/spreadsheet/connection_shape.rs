// xdr:cxnSp
use super::super::super::Anchor;
use super::NonVisualConnectionShapeProperties;
use super::ShapeProperties;
use super::ShapeStyle;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ConnectionShape {
    anchor: Anchor,
    non_visual_connection_shape_properties: NonVisualConnectionShapeProperties,
    shape_properties: ShapeProperties,
    shape_style: ShapeStyle,
}
impl ConnectionShape {
    pub fn get_anchor(&self) -> &Anchor {
        &self.anchor
    }

    pub fn get_anchor_mut(&mut self) -> &mut Anchor {
        &mut self.anchor
    }

    pub fn set_anchor(&mut self, value: Anchor) {
        self.anchor = value;
    }

    pub fn get_non_visual_connection_shape_properties(
        &self,
    ) -> &NonVisualConnectionShapeProperties {
        &self.non_visual_connection_shape_properties
    }

    pub fn get_non_visual_connection_shape_properties_mut(
        &mut self,
    ) -> &mut NonVisualConnectionShapeProperties {
        &mut self.non_visual_connection_shape_properties
    }

    pub fn set_non_visual_connection_shape_properties(
        &mut self,
        value: NonVisualConnectionShapeProperties,
    ) {
        self.non_visual_connection_shape_properties = value;
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

    pub fn get_shape_style(&self) -> &ShapeStyle {
        &self.shape_style
    }

    pub fn get_shape_style_mut(&mut self) -> &mut ShapeStyle {
        &mut self.shape_style
    }

    pub fn set_shape_style(&mut self, value: ShapeStyle) {
        self.shape_style = value;
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
                    b"xdr:nvCxnSpPr" => {
                        self.non_visual_connection_shape_properties
                            .set_attributes(reader, e);
                    }
                    b"xdr:spPr" => {
                        self.shape_properties.set_attributes(reader, e);
                    }
                    b"xdr:style" => {
                        self.shape_style.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xdr:cxnSp" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:cxnSp"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:cxnSp
        write_start_tag(writer, "xdr:cxnSp", vec![("macro", "")], false);

        // xdr:nvCxnSpPr
        let _ = &self.non_visual_connection_shape_properties.write_to(writer);

        // xdr:spPr
        let _ = &self.shape_properties.write_to(writer);

        // xdr:style
        let _ = &self.shape_style.write_to(writer);

        write_end_tag(writer, "xdr:cxnSp");
    }
}
