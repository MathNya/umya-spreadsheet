// c:rich
use super::super::BodyProperties;
use super::super::ListStyle;
use super::super::Paragraph;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct RichText {
    body_properties: BodyProperties,
    list_style: ListStyle,
    paragraph: Vec<Paragraph>,
}
impl RichText {
    pub fn get_body_properties(&self) -> &BodyProperties {
        &self.body_properties
    }

    pub fn get_body_properties_mut(&mut self) -> &mut BodyProperties {
        &mut self.body_properties
    }

    pub fn set_body_properties(&mut self, value: BodyProperties) {
        self.body_properties = value;
    }

    pub fn get_list_style(&self) -> &ListStyle {
        &self.list_style
    }

    pub fn get_list_style_mut(&mut self) -> &mut ListStyle {
        &mut self.list_style
    }

    pub fn set_list_style(&mut self, value: ListStyle) {
        self.list_style = value;
    }

    pub fn get_paragraph(&self) -> &Vec<Paragraph> {
        &self.paragraph
    }

    pub fn get_paragraph_mut(&mut self) -> &mut Vec<Paragraph> {
        &mut self.paragraph
    }

    pub fn add_paragraph(&mut self, value: Paragraph) {
        self.paragraph.push(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().0 {
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
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().0 {
                    b"a:bodyPr" => {
                        let mut body_properties = BodyProperties::default();
                        body_properties.set_attributes(reader, e, true);
                        self.set_body_properties(body_properties);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:rich" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:rich"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:rich
        write_start_tag(writer, "c:rich", vec![], false);

        // a:bodyPr
        self.body_properties.write_to(writer);

        // a:lstStyle
        write_start_tag(writer, "a:lstStyle", vec![], true);

        for content in &self.paragraph {
            content.write_to(writer);
        }

        write_end_tag(writer, "c:rich");
    }
}
