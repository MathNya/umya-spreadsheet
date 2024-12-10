// xdr:sp
use super::super::super::Anchor;
use super::NonVisualShapeProperties;
use super::ShapeProperties;
use super::ShapeStyle;
use super::TextBody;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::structs::raw::RawRelationships;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Shape {
    anchor: Anchor,
    non_visual_shape_properties: NonVisualShapeProperties,
    shape_properties: ShapeProperties,
    shape_style: Option<Box<ShapeStyle>>,
    text_body: Option<Box<TextBody>>,
}

impl Shape {
    #[inline]
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
    pub fn get_non_visual_shape_properties(&self) -> &NonVisualShapeProperties {
        &self.non_visual_shape_properties
    }

    pub fn get_non_visual_shape_properties_mut(&mut self) -> &mut NonVisualShapeProperties {
        &mut self.non_visual_shape_properties
    }

    #[inline]
    pub fn set_non_visual_shape_properties(&mut self, value: NonVisualShapeProperties) {
        self.non_visual_shape_properties = value;
    }

    #[inline]
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

    pub fn get_shape_style(&self) -> Option<&ShapeStyle> {
        self.shape_style.as_deref()
    }

    #[inline]
    pub fn get_shape_style_mut(&mut self) -> Option<&mut ShapeStyle> {
        self.shape_style.as_deref_mut()
    }

    #[inline]
    pub fn set_shape_style(&mut self, value: ShapeStyle) {
        self.shape_style = Some(Box::new(value));
    }

    #[inline]
    pub fn get_text_body(&self) -> Option<&TextBody> {
        self.text_body.as_deref()
    }

    #[inline]
    pub fn get_text_body_mut(&mut self) -> Option<&mut TextBody> {
        self.text_body.as_deref_mut()
    }

    #[inline]
    pub fn set_text_body(&mut self, value: TextBody) {
        self.text_body = Some(Box::new(value));
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
                        b"xdr:nvSpPr" => {
                            self.non_visual_shape_properties.set_attributes(reader, e);
                        }
                        b"xdr:spPr" => {
                            self.shape_properties.set_attributes(reader, e, drawing_relationships);
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
                    }
                },
                Event::End(ref e) => {
                    if e.name().into_inner() == b"xdr:sp" {
                        return;
                    }
                },
                Event::Eof => panic!("Error: Could not find {} end element", "xdr:sp")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
        ole_id: &usize,
    ) {
        // xdr:sp
        write_start_tag(
            writer,
            "xdr:sp",
            vec![("macro", ""), ("textlink", "")],
            false,
        );

        // xdr:nvSpPr
        self.non_visual_shape_properties.write_to(writer, ole_id);

        // xdr:spPr
        self.shape_properties.write_to(writer, rel_list);

        // xdr:style
        if let Some(v) = &self.shape_style {
            v.write_to(writer);
        }

        // xdr:txBody
        if let Some(v) = &self.text_body {
            v.write_to(writer);
        }

        write_end_tag(writer, "xdr:sp");
    }
}
