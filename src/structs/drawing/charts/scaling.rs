// c:scaling
use super::Orientation;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Scaling {
    orientation: Orientation,
}

impl Scaling {
    pub fn get_orientation(&self) -> &Orientation {
        &self.orientation
    }

    pub fn get_orientation_mut(&mut self) -> &mut Orientation {
        &mut self.orientation
    }

    pub fn set_orientation(&mut self, value: Orientation) -> &mut Self {
        self.orientation = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().0 == b"c:orientation" {
                    self.orientation.set_attributes(reader, e);
                }
            },
            Event::End(ref e) => {
                if e.name().0 == b"c:scaling" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:scaling")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:scaling
        write_start_tag(writer, "c:scaling", vec![], false);

        // c:orientation
        self.orientation.write_to(writer);

        write_end_tag(writer, "c:scaling");
    }
}
