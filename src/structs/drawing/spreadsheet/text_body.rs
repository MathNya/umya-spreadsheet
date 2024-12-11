use super::super::BodyProperties;
use super::super::ListStyle;
use super::super::Paragraph;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub struct TextBody {
    body_properties: BodyProperties,
    list_style: ListStyle,
    paragraph: ThinVec<Paragraph>,
}

impl TextBody {
    #[inline]
    pub fn get_body_properties(&self) -> &BodyProperties {
        &self.body_properties
    }

    #[inline]
    pub fn get_body_properties_mut(&mut self) -> &mut BodyProperties {
        &mut self.body_properties
    }

    #[inline]
    pub fn set_body_properties(&mut self, value: BodyProperties) {
        self.body_properties = value;
    }

    #[inline]
    pub fn get_list_style(&self) -> &ListStyle {
        &self.list_style
    }

    #[inline]
    pub fn get_list_style_mut(&mut self) -> &mut ListStyle {
        &mut self.list_style
    }

    #[inline]
    pub fn set_list_style(&mut self, value: ListStyle) {
        self.list_style = value;
    }

    #[inline]
    pub fn get_paragraph(&self) -> &[Paragraph] {
        &self.paragraph
    }

    #[inline]
    pub fn get_paragraph_mut(&mut self) -> &mut ThinVec<Paragraph> {
        &mut self.paragraph
    }

    #[inline]
    pub fn add_paragraph(&mut self, value: Paragraph) {
        self.paragraph.push(value);
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
                    b"a:p" => {
                        let mut paragraph = Paragraph::default();
                        paragraph.set_attributes(reader, e);
                        self.add_paragraph(paragraph);
                    }
                    b"a:bodyPr" => {
                        let mut body_properties = BodyProperties::default();
                        body_properties.set_attributes(reader, e, false);
                        self.set_body_properties(body_properties);
                    }
                    b"a:lstStyle" => {
                        let mut obj = ListStyle::default();
                        obj.set_attributes(reader, e);
                        self.set_list_style(obj);
                    }
                    _ => (),
                }
            },
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:bodyPr" {
                    let mut body_properties = BodyProperties::default();
                    body_properties.set_attributes(reader, e, true);
                    self.set_body_properties(body_properties);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:txBody" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:txBody")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:txBody
        write_start_tag(writer, "xdr:txBody", vec![], false);

        // a:bodyPr
        self.body_properties.write_to(writer);

        // a:lstStyle
        self.list_style.write_to(writer);

        for content in &self.paragraph {
            content.write_to(writer);
        }

        write_end_tag(writer, "xdr:txBody");
    }
}
