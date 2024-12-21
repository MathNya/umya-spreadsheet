// oleObjects
use super::OleObject;
use crate::reader::driver::xml_read_loop;
use crate::structs::raw::RawRelationships;
use crate::writer::driver::{write_end_tag, write_start_tag};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct OleObjects {
    ole_object: Vec<OleObject>,
}

impl OleObjects {
    #[inline]
    #[must_use]
    pub fn get_ole_object(&self) -> &[OleObject] {
        &self.ole_object
    }

    #[inline]
    pub fn get_ole_object_mut(&mut self) -> &mut Vec<OleObject> {
        &mut self.ole_object
    }

    #[inline]
    pub fn set_ole_object(&mut self, value: OleObject) -> &mut Self {
        self.ole_object.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        relationships: &RawRelationships,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                if e.name().into_inner() == b"mc:AlternateContent" {
                    let mut obj = OleObject::default();
                    obj.set_attributes(reader, e, relationships);
                    self.set_ole_object(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"oleObjects" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "oleObjects")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        r_id: usize,
        ole_id: usize,
    ) {
        if !self.ole_object.is_empty() {
            // oleObjects
            write_start_tag(writer, "oleObjects", vec![], false);

            // mc:AlternateContent
            let mut r = r_id;
            let mut o = ole_id;
            for obj in &self.ole_object {
                obj.write_to(writer, r, o);
                r += 2;
                o += 1;
            }

            write_end_tag(writer, "oleObjects");
        }
    }
}
