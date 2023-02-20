// c:tx
use super::RichText;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ChartText {
    rich_text: RichText,
}
impl ChartText {
    pub fn get_rich_text(&self) -> &RichText {
        &self.rich_text
    }

    pub fn get_rich_text_mut(&mut self) -> &mut RichText {
        &mut self.rich_text
    }

    pub fn set_rich_text(&mut self, value: RichText) -> &mut ChartText {
        self.rich_text = value;
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
                    b"c:rich" => {
                        self.rich_text.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"c:tx" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:tx"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:tx
        write_start_tag(writer, "c:tx", vec![], false);

        // c:rich
        self.rich_text.write_to(writer);

        write_end_tag(writer, "c:tx");
    }
}
