// xdr:cxnSp
use std::io::Cursor;

use quick_xml::{
    events::{BytesStart, Event},
    Reader, Writer,
};

use super::{
    super::super::Anchor, NonVisualConnectionShapeProperties, ShapeProperties, ShapeStyle,
};
use crate::{
    reader::driver::xml_read_loop,
    structs::raw::RawRelationships,
    writer::driver::{write_end_tag, write_start_tag},
};

#[derive(Clone, Default, Debug)]
pub struct ConnectionShape {
    anchor: Anchor,
    non_visual_connection_shape_properties: NonVisualConnectionShapeProperties,
    shape_properties: ShapeProperties,
    shape_style: ShapeStyle,
}

impl ConnectionShape {
    #[inline]
    #[must_use]
    pub fn get_anchor(&self) -> &Anchor {
        &self.anchor
    }

    #[inline]
    pub fn get_anchor_mut(&mut self) -> &mut Anchor {
        &mut self.anchor
    }

    #[inline]
    pub fn set_anchor(&mut self, value: Anchor) {
        self.anchor = value;
    }

    #[inline]
    #[must_use]
    pub fn get_non_visual_connection_shape_properties(
        &self,
    ) -> &NonVisualConnectionShapeProperties {
        &self.non_visual_connection_shape_properties
    }

    #[inline]
    pub fn get_non_visual_connection_shape_properties_mut(
        &mut self,
    ) -> &mut NonVisualConnectionShapeProperties {
        &mut self.non_visual_connection_shape_properties
    }

    #[inline]
    pub fn set_non_visual_connection_shape_properties(
        &mut self,
        value: NonVisualConnectionShapeProperties,
    ) {
        self.non_visual_connection_shape_properties = value;
    }

    #[inline]
    #[must_use]
    pub fn get_shape_properties(&self) -> &ShapeProperties {
        &self.shape_properties
    }

    #[inline]
    pub fn get_shape_properties_mut(&mut self) -> &mut ShapeProperties {
        &mut self.shape_properties
    }

    #[inline]
    pub fn set_shape_properties(&mut self, value: ShapeProperties) {
        self.shape_properties = value;
    }

    #[inline]
    #[must_use]
    pub fn get_shape_style(&self) -> &ShapeStyle {
        &self.shape_style
    }

    #[inline]
    pub fn get_shape_style_mut(&mut self) -> &mut ShapeStyle {
        &mut self.shape_style
    }

    #[inline]
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
        write_start_tag(writer, "xdr:cxnSp", vec![("macro", "").into()], false);

        // xdr:nvCxnSpPr
        self.non_visual_connection_shape_properties.write_to(writer);

        // xdr:spPr
        self.shape_properties.write_to(writer, rel_list);

        // xdr:style
        self.shape_style.write_to(writer);

        write_end_tag(writer, "xdr:cxnSp");
    }
}
