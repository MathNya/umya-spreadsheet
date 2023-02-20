// si
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub(crate) struct PhoneticRun {}
impl PhoneticRun {
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"rPh" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "rPh"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn _write_to(&self, _writer: &mut Writer<Cursor<Vec<u8>>>) {}
}
