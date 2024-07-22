// xdr:xfrm
use super::super::{Extents, Offset};
use crate::{BooleanValue, Int32Value};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Transform {
    offset: Offset,
    extents: Extents,
    rotation: Int32Value,
    vertical_flip: BooleanValue,
    horizontal_flip: BooleanValue,
}

impl Transform {
    pub fn get_offset(&self) -> &Offset {
        &self.offset
    }

    pub fn get_offset_mut(&mut self) -> &mut Offset {
        &mut self.offset
    }

    pub fn set_offset(&mut self, value: Offset) -> &mut Transform {
        self.offset = value;
        self
    }

    pub fn get_extents(&self) -> &Extents {
        &self.extents
    }

    pub fn get_extents_mut(&mut self) -> &mut Extents {
        &mut self.extents
    }

    pub fn set_extents(&mut self, value: Extents) -> &mut Transform {
        self.extents = value;
        self
    }

    pub fn get_rotation(&self) -> &i32 {
        self.rotation.get_value()
    }

    pub fn set_rotation(&mut self, value: i32) {
        self.rotation.set_value(value);
    }

    pub fn get_vertical_flip(&self) -> &bool {
        self.vertical_flip.get_value()
    }

    pub fn set_vertical_flip(&mut self, value: bool) {
        self.vertical_flip.set_value(value);
    }

    pub fn get_horizontal_flip(&self) -> &bool {
        self.horizontal_flip.get_value()
    }

    pub fn set_horizontal_flip(&mut self, value: bool) {
        self.horizontal_flip.set_value(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, rotation, "rot");
        set_string_from_xml!(self, e, horizontal_flip, "flipH");
        set_string_from_xml!(self, e, vertical_flip, "flipV");

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"a:off" => {
                        self.offset.set_attributes(reader, e);
                    }
                    b"a:ext" => {
                        self.extents.set_attributes(reader, e);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if  e.name().into_inner() == b"xdr:xfrm" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:xfrm")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:xfrm
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let rot = self.rotation.get_value_string();
        if self.rotation.has_value() {
            attributes.push(("rot", &rot));
        }
        if self.horizontal_flip.has_value() {
            attributes.push(("flipH", self.horizontal_flip.get_value_string()));
        }
        if self.vertical_flip.has_value() {
            attributes.push(("flipV", self.vertical_flip.get_value_string()));
        }
        write_start_tag(writer, "xdr:xfrm", attributes, false);

        // a:off
        self.offset.write_to(writer);

        // a:ext
        self.extents.write_to(writer);

        write_end_tag(writer, "xdr:xfrm");
    }
}
