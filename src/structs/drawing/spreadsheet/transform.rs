// xdr:xfrm
use super::super::Offset;
use super::super::Extents;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Transform {
    offset: Offset,
    extents: Extents,
    rotation: Option<String>,
    flip_v: Option<String>,
    flip_h: Option<String>,
}
impl Transform {
    pub fn get_offset(&self) -> &Offset {
        &self.offset
    }

    pub fn get_offset_mut(&mut self) -> &mut Offset {
        &mut self.offset
    }

    pub fn set_offset(&mut self, value:Offset) -> &mut Transform {
        self.offset = value;
        self
    }

    pub fn get_extents(&self) -> &Extents {
        &self.extents
    }

    pub fn get_extents_mut(&mut self) -> &mut Extents {
        &mut self.extents
    }

    pub fn set_extents(&mut self, value:Extents) -> &mut Transform {
        self.extents = value;
        self
    }

    pub fn get_rotation(&self) -> &Option<String> {
        &self.rotation
    }
    
    pub fn set_rotation<S: Into<String>>(&mut self, value:S) {
        self.rotation = Some(value.into());
    }

    pub fn get_flip_v(&self) -> &Option<String> {
        &self.flip_v
    }
    
    pub fn set_flip_v<S: Into<String>>(&mut self, value:S) {
        self.flip_v = Some(value.into());
    }

    pub fn get_flip_h(&self) -> &Option<String> {
        &self.flip_h
    }
    
    pub fn set_flip_h<S: Into<String>>(&mut self, value:S) {
        self.flip_h = Some(value.into());
    }
    
    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        let mut buf = Vec::new();
    
        match get_attribute(e, b"rot") {
            Some(v) => {&mut self.set_rotation(v);},
            None => {}
        }
    
        match get_attribute(e, b"flipH") {
            Some(v) => {&mut self.set_flip_h(v);},
            None => {}
        }
    
        match get_attribute(e, b"flipV") {
            Some(v) => {&mut self.set_flip_v(v);},
            None => {}
        }
    
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"a:off" => {
                            self.offset.set_attributes(reader, e);
                        },
                        b"a:ext" => {
                            self.extents.set_attributes(reader, e);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"xdr:xfrm" => {
                            return;
                        },
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:xfrm"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:xfrm
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match &self.rotation {
            Some(v) => attributes.push(("rot", v)),
            None => {}
        }
        match &self.flip_h {
            Some(v) => attributes.push(("flipH", v)),
            None => {}
        }
        match &self.flip_v {
            Some(v) => attributes.push(("flipV", v)),
            None => {}
        }
        write_start_tag(writer, "xdr:xfrm", attributes, false);

        // a:off
        &self.offset.write_to(writer);

        // a:ext
        &self.extents.write_to(writer);

        write_end_tag(writer, "xdr:xfrm");
    }
}