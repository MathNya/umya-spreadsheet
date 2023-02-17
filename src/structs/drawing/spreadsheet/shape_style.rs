// xdr:style
use super::super::StyleMatrixReferenceType;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ShapeStyle {
    line_reference: Option<StyleMatrixReferenceType>,
    fill_reference: Option<StyleMatrixReferenceType>,
    effect_reference: Option<StyleMatrixReferenceType>,
    font_reference: Option<StyleMatrixReferenceType>,
}
impl ShapeStyle {
    pub fn get_line_reference(&self) -> &Option<StyleMatrixReferenceType> {
        &self.line_reference
    }

    pub fn set_line_reference(&mut self, value: StyleMatrixReferenceType) {
        self.line_reference = Some(value);
    }

    pub fn get_fill_reference(&self) -> &Option<StyleMatrixReferenceType> {
        &self.fill_reference
    }

    pub fn set_fill_reference(&mut self, value: StyleMatrixReferenceType) {
        self.fill_reference = Some(value);
    }

    pub fn get_effect_reference(&self) -> &Option<StyleMatrixReferenceType> {
        &self.effect_reference
    }

    pub fn set_effect_reference(&mut self, value: StyleMatrixReferenceType) {
        self.effect_reference = Some(value);
    }

    pub fn get_font_reference(&self) -> &Option<StyleMatrixReferenceType> {
        &self.font_reference
    }

    pub fn set_font_reference(&mut self, value: StyleMatrixReferenceType) {
        self.font_reference = Some(value);
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
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xdr:style" => {
                        return;
                    }
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:style"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:style
        write_start_tag(writer, "xdr:style", vec![], false);

        // a:lnRef
        match &self.line_reference {
            Some(style) => {
                style.write_to(writer, "a:lnRef");
            }
            None => {}
        }

        // a:fillRef
        match &self.fill_reference {
            Some(style) => {
                style.write_to(writer, "a:fillRef");
            }
            None => {}
        }

        // a:effectRef
        match &self.effect_reference {
            Some(style) => {
                style.write_to(writer, "a:effectRef");
            }
            None => {}
        }

        // a:fontRef
        match &self.font_reference {
            Some(style) => {
                style.write_to(writer, "a:fontRef");
            }
            None => {}
        }

        write_end_tag(writer, "xdr:style");
    }
}
