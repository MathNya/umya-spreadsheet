// a:lnRef
use std::io::Cursor;

use quick_xml::{
    events::{BytesStart, Event},
    Reader, Writer,
};

use super::SchemeColor;
use crate::{
    reader::driver::{get_attribute, xml_read_loop},
    writer::driver::{write_end_tag, write_start_tag},
};

#[derive(Clone, Default, Debug)]
pub struct StyleMatrixReferenceType {
    index: Box<str>,
    scheme_color: Option<Box<SchemeColor>>,
}

impl StyleMatrixReferenceType {
    #[inline]
    #[must_use]
    pub fn get_index(&self) -> &str {
        &self.index
    }

    #[inline]
    pub fn set_index<S: Into<String>>(&mut self, value: S) {
        self.index = value.into().into_boxed_str();
    }

    #[inline]
    #[must_use]
    pub fn get_scheme_color(&self) -> Option<&SchemeColor> {
        self.scheme_color.as_deref()
    }

    #[inline]
    pub fn set_scheme_color(&mut self, value: SchemeColor) {
        self.scheme_color = Some(Box::new(value));
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        self.set_index(get_attribute(e, b"idx").unwrap());

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"a:schemeClr" {
                    let mut scheme_color = SchemeColor::default();
                    scheme_color.set_attributes(reader, e, false);
                    self.set_scheme_color(scheme_color);
                }
            },
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:schemeClr" {
                    let mut scheme_color = SchemeColor::default();
                    scheme_color.set_attributes(reader, e, true);
                    self.set_scheme_color(scheme_color);
                }
            },
            Event::End(ref e) => {
                match e.name().into_inner() {
                    b"a:lnRef"     |
                    b"a:fillRef"   |
                    b"a:effectRef" |
                    b"a:fontRef" => return,
                    _ => (),
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:lnRef")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        if let Some(color) = &self.scheme_color {
            write_start_tag(writer, tag_name, vec![("idx", &self.index).into()], false);
            // a:schemeClr
            color.write_to(writer);
            write_end_tag(writer, tag_name);
        } else {
            write_start_tag(writer, tag_name, vec![("idx", &self.index).into()], true);
        }
    }
}
