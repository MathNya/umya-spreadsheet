use std::io::Cursor;

use quick_xml::{
    Reader, Writer,
    events::{BytesStart, Event},
};

use super::run_properties::RunProperties;
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{write_end_tag, write_start_tag, write_text_node},
};

#[derive(Clone, Default, Debug)]
pub struct Run {
    text: Box<str>,
    run_properties: RunProperties,
}

impl Run {
    #[inline]
    #[must_use]
    pub fn get_text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn set_text<S: Into<String>>(&mut self, value: S) {
        self.text = value.into().into_boxed_str();
    }

    #[inline]
    #[must_use]
    pub fn get_run_properties(&self) -> &RunProperties {
        &self.run_properties
    }

    #[inline]
    pub fn get_run_properties_mut(&mut self) -> &mut RunProperties {
        &mut self.run_properties
    }

    #[inline]
    pub fn set_run_properties(&mut self, value: RunProperties) {
        self.run_properties = value;
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().0 == b"a:rPr" {
                    self.run_properties.set_attributes(reader, e, false);
                }
            },
            Event::Empty(ref e) => {
                if e.name().0 == b"a:rPr" {
                    self.run_properties.set_attributes(reader, e, true);
                }
            },
            Event::Text(e) => {
                self.set_text(e.unescape().unwrap());
            },
            Event::End(ref e) => {
                if e.name().0 == b"a:r" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:r")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:r
        write_start_tag(writer, "a:r", vec![], false);

        // a:rPr
        self.run_properties.write_to_rpr(writer);

        // a:t
        write_start_tag(writer, "a:t", vec![], false);
        write_text_node(writer, &*self.text);
        write_end_tag(writer, "a:t");

        write_end_tag(writer, "a:r");
    }
}
