// si
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::{BytesStart, Event};

use crate::reader::driver::xml_read_loop;

#[derive(Default, Debug)]
pub(crate) struct PhoneticRun {}

impl PhoneticRun {
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"rPh" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "rPh")
        );
    }

    pub(crate) fn write_to(&self, _writer: &mut Writer<Cursor<Vec<u8>>>) {}
}
