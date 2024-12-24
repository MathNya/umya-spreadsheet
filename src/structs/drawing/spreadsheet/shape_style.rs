// xdr:style
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::super::StyleMatrixReferenceType;
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct ShapeStyle {
    line_reference:   Option<Box<StyleMatrixReferenceType>>,
    fill_reference:   Option<Box<StyleMatrixReferenceType>>,
    effect_reference: Option<Box<StyleMatrixReferenceType>>,
    font_reference:   Option<Box<StyleMatrixReferenceType>>,
}

impl ShapeStyle {
    #[inline]
    #[must_use]
    pub fn get_line_reference(&self) -> Option<&StyleMatrixReferenceType> {
        self.line_reference.as_deref()
    }

    #[inline]
    pub fn set_line_reference(&mut self, value: StyleMatrixReferenceType) {
        self.line_reference = Some(Box::new(value));
    }

    #[inline]
    #[must_use]
    pub fn get_fill_reference(&self) -> Option<&StyleMatrixReferenceType> {
        self.fill_reference.as_deref()
    }

    #[inline]
    pub fn set_fill_reference(&mut self, value: StyleMatrixReferenceType) {
        self.fill_reference = Some(Box::new(value));
    }

    #[inline]
    #[must_use]
    pub fn get_effect_reference(&self) -> Option<&StyleMatrixReferenceType> {
        self.effect_reference.as_deref()
    }

    #[inline]
    pub fn set_effect_reference(&mut self, value: StyleMatrixReferenceType) {
        self.effect_reference = Some(Box::new(value));
    }

    #[inline]
    #[must_use]
    pub fn get_font_reference(&self) -> Option<&StyleMatrixReferenceType> {
        self.font_reference.as_deref()
    }

    #[inline]
    pub fn set_font_reference(&mut self, value: StyleMatrixReferenceType) {
        self.font_reference = Some(Box::new(value));
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"a:lnRef" => {
                        let mut style_matrix_reference_type = StyleMatrixReferenceType::default();
                        style_matrix_reference_type.set_attributes(reader, e, false);
                        self.set_line_reference(style_matrix_reference_type);
                    }
                    b"a:fillRef" => {
                        let mut style_matrix_reference_type = StyleMatrixReferenceType::default();
                        style_matrix_reference_type.set_attributes(reader, e, false);
                        self.set_fill_reference(style_matrix_reference_type);
                    }
                    b"a:effectRef" => {
                        let mut style_matrix_reference_type = StyleMatrixReferenceType::default();
                        style_matrix_reference_type.set_attributes(reader, e, false);
                        self.set_effect_reference(style_matrix_reference_type);
                    }
                    b"a:fontRef" => {
                        let mut style_matrix_reference_type = StyleMatrixReferenceType::default();
                        style_matrix_reference_type.set_attributes(reader, e, false);
                        self.set_font_reference(style_matrix_reference_type);
                    }
                    _ => (),
                }
            },
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"a:lnRef" => {
                        let mut style_matrix_reference_type = StyleMatrixReferenceType::default();
                        style_matrix_reference_type.set_attributes(reader, e, true);
                        self.set_line_reference(style_matrix_reference_type);
                    }
                    b"a:fillRef" => {
                        let mut style_matrix_reference_type = StyleMatrixReferenceType::default();
                        style_matrix_reference_type.set_attributes(reader, e, true);
                        self.set_fill_reference(style_matrix_reference_type);
                    }
                    b"a:effectRef" => {
                        let mut style_matrix_reference_type = StyleMatrixReferenceType::default();
                        style_matrix_reference_type.set_attributes(reader, e, true);
                        self.set_effect_reference(style_matrix_reference_type);
                    }
                    b"a:fontRef" => {
                        let mut style_matrix_reference_type = StyleMatrixReferenceType::default();
                        style_matrix_reference_type.set_attributes(reader, e, true);
                        self.set_font_reference(style_matrix_reference_type);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:style" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:style")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:style
        write_start_tag(writer, "xdr:style", vec![], false);

        // a:lnRef
        if let Some(style) = &self.line_reference {
            style.write_to(writer, "a:lnRef");
        }

        // a:fillRef
        if let Some(style) = &self.fill_reference {
            style.write_to(writer, "a:fillRef");
        }

        // a:effectRef
        if let Some(style) = &self.effect_reference {
            style.write_to(writer, "a:effectRef");
        }

        // a:fontRef
        if let Some(style) = &self.font_reference {
            style.write_to(writer, "a:fontRef");
        }

        write_end_tag(writer, "xdr:style");
    }
}
