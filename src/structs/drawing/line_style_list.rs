use super::Outline;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct LineStyleList {
    outline_collection: Vec<Outline>,
}

impl LineStyleList {
    pub fn get_outline_collection(&self) -> &Vec<Outline> {
        &self.outline_collection
    }

    pub fn get_outline_collection_mut(&mut self) -> &mut Vec<Outline> {
        &mut self.outline_collection
    }

    pub fn set_outline_collection(&mut self, value: Vec<Outline>) -> &mut Self {
        self.outline_collection = value;
        self
    }

    pub fn add_outline_collection(&mut self, value: Outline) -> &mut Self {
        self.outline_collection.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"a:ln" {
                    let mut obj = Outline::default();
                    obj.set_attributes(reader, e);
                    self.outline_collection.push(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:lnStyleLst" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "lnStyleLst")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lnStyleLst
        write_start_tag(writer, "a:lnStyleLst", vec![], false);

        // a:ln
        for v in &self.outline_collection {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:lnStyleLst");
    }
}
