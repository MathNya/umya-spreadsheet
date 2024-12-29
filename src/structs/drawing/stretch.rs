// a:stretch
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::fill_rectangle::FillRectangle;
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Stretch {
    fill_rectangle: Option<Box<FillRectangle>>,
}

impl Stretch {
    #[must_use]
    pub fn get_fill_rectangle(&self) -> Option<&FillRectangle> {
        self.fill_rectangle.as_deref()
    }

    pub fn get_fill_rectangle_mut(&mut self) -> Option<&mut FillRectangle> {
        self.fill_rectangle.as_deref_mut()
    }

    pub fn set_fill_rectangle(&mut self, value: FillRectangle) {
        self.fill_rectangle = Some(Box::new(value));
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
                    let fill_rectangle = FillRectangle::default();
                    FillRectangle::set_attributes(reader, e);
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
            Some(_) => {
                write_start_tag(writer, "a:stretch", vec![], false);
                FillRectangle::write_to(writer);
                write_end_tag(writer, "a:stretch");
            }
            None => {
                write_start_tag(writer, "a:stretch", vec![], true);
            }
        }
    }
}
