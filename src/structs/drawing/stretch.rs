// a:stretch
use super::fill_rectangle::FillRectangle;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Stretch {
    fill_rectangle: Option<FillRectangle>,
}

impl Stretch {
    pub fn get_fill_rectangle(&self) -> Option<&FillRectangle> {
        self.fill_rectangle.as_ref()
    }

    pub fn get_fill_rectangle_mut(&mut self) -> Option<&mut FillRectangle> {
        self.fill_rectangle.as_mut()
    }

    pub fn set_fill_rectangle(&mut self, value: FillRectangle) {
        self.fill_rectangle = Some(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:fillRect" {
                    let mut fill_rectangle = FillRectangle::default();
                    fill_rectangle.set_attributes(reader, e);
                    self.set_fill_rectangle(fill_rectangle);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:stretch" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:stretch")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:stretch
        match &self.fill_rectangle {
            Some(v) => {
                write_start_tag(writer, "a:stretch", vec![], false);
                v.write_to(writer);
                write_end_tag(writer, "a:stretch");
            }
            None => {
                write_start_tag(writer, "a:stretch", vec![], true);
            }
        }
    }
}
