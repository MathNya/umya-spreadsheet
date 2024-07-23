// a:lnRef
use super::SchemeColor;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct StyleMatrixReferenceType {
    index: String,
    scheme_color: Option<SchemeColor>,
}

impl StyleMatrixReferenceType {
    pub fn get_index(&self) -> &str {
        &self.index
    }

    pub fn set_index<S: Into<String>>(&mut self, value: S) {
        self.index = value.into();
    }

    pub fn get_scheme_color(&self) -> Option<&SchemeColor> {
        self.scheme_color.as_ref()
    }

    pub fn set_scheme_color(&mut self, value: SchemeColor) {
        self.scheme_color = Some(value);
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
                    b"a:lnRef" => {
                        return;
                    }
                    b"a:fillRef" => {
                        return;
                    }
                    b"a:effectRef" => {
                        return;
                    }
                    b"a:fontRef" => {
                        return;
                    }
                    _ => (),
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:lnRef")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        if let Some(color) = &self.scheme_color {
            write_start_tag(writer, tag_name, vec![("idx", &self.index)], false);
            // a:schemeClr
            color.write_to(writer);
            write_end_tag(writer, tag_name);
        } else {
            write_start_tag(writer, tag_name, vec![("idx", &self.index)], true);
        }
    }
}
