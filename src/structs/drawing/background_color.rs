// a:bgClr
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::SchemeColor;
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct BackgroundColor {
    scheme_color: SchemeColor,
}

impl BackgroundColor {
    #[must_use]
    pub fn get_scheme_color(&self) -> &SchemeColor {
        &self.scheme_color
    }

    pub fn get_scheme_color_mut(&mut self) -> &mut SchemeColor {
        &mut self.scheme_color
    }

    pub fn set_scheme_color(&mut self, value: SchemeColor) -> &mut BackgroundColor {
        self.scheme_color = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"a:schemeClr" {
                    self.scheme_color.set_attributes(reader, e, false);
                }
            },
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:schemeClr" {
                    self.scheme_color.set_attributes(reader, e, true);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:bgClr" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:bgClr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:bgClr
        write_start_tag(writer, "a:bgClr", vec![], false);

        // a:schemeClr
        self.scheme_color.write_to(writer);

        write_end_tag(writer, "a:bgClr");
    }
}
