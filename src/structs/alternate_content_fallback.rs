// mc:Fallback
use super::drawing::charts::Style;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct AlternateContentFallback {
    style: Style,
}
impl AlternateContentFallback {

    pub fn get_style(&self)-> &Style {
        &self.style
    }

    pub fn get_style_mut(&mut self)-> &mut Style {
        &mut self.style
    }

    pub fn set_style(&mut self, value:Style)-> &mut AlternateContentFallback {
        self.style = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"c:style" => {
                            self.style.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"mc:Fallback" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "mc:Fallback"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // mc:Fallback
        write_start_tag(writer, "mc:Fallback", vec![], false);

        // c:style
        self.style.write_to(writer);

        write_end_tag(writer, "mc:Fallback");
    }
}
