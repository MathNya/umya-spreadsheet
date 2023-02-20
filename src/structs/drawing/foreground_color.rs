// a:fgClr
use super::SchemeColor;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ForegroundColor {
    scheme_color: SchemeColor,
}
impl ForegroundColor {
    pub fn get_scheme_color(&self) -> &SchemeColor {
        &self.scheme_color
    }

    pub fn get_scheme_color_mut(&mut self) -> &mut SchemeColor {
        &mut self.scheme_color
    }

    pub fn set_scheme_color(&mut self, value: SchemeColor) -> &mut ForegroundColor {
        self.scheme_color = value;
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
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:schemeClr" => {
                        self.scheme_color.set_attributes(reader, e, false);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:schemeClr" => {
                        self.scheme_color.set_attributes(reader, e, true);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:fgClr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:fgClr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:fgClr
        write_start_tag(writer, "a:fgClr", vec![], false);

        // a:schemeClr
        let _ = &self.scheme_color.write_to(writer);

        write_end_tag(writer, "a:fgClr");
    }
}
