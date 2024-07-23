// xdr:cxnSp
use super::super::super::Anchor;
use super::NonVisualConnectionShapeProperties;
use super::ShapeProperties;
use super::ShapeStyle;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::raw::RawRelationships;
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
        drawing_relationships: Option<&RawRelationships>,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"xdr:nvCxnSpPr" => {
                        self.non_visual_connection_shape_properties
                            .set_attributes(reader, e);
                        }
                    b"xdr:spPr" => {
                        self.shape_properties.set_attributes(reader, e, drawing_relationships);
                    }
                    b"xdr:style" => {
                        self.shape_style.set_attributes(reader, e);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:cxnSp" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:cxnSp")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // xdr:cxnSp
        write_start_tag(writer, "xdr:cxnSp", vec![("macro", "")], false);

        // xdr:nvCxnSpPr
        self.non_visual_connection_shape_properties.write_to(writer);

        // xdr:spPr
        self.shape_properties.write_to(writer, rel_list);

        // xdr:style
        self.shape_style.write_to(writer);

        write_end_tag(writer, "xdr:cxnSp");
    }
}
