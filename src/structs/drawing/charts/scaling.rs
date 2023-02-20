// c:scaling
use super::Orientation;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
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
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().0 {
                    b"c:orientation" => {
                        self.orientation.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().0 {
                    b"c:scaling" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:scaling"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:scaling
        write_start_tag(writer, "c:scaling", vec![], false);

        // c:orientation
        self.orientation.write_to(writer);

        write_end_tag(writer, "c:scaling");
    }
}
