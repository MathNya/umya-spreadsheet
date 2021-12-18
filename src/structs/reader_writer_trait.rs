use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

pub(crate) trait ReaderWriterTrait {
    fn set_attributes<R: std::io::BufRead>(&mut self, reader: &mut Reader<R>, e: &BytesStart, empty_flag: &bool);
    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>);
}
