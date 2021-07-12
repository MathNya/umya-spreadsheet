// c:sideWall
use super::Thickness;
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct SideWall {
    thickness: Option<Thickness>,
}
impl SideWall {
    pub fn get_thickness(&self)-> &Option<Thickness> {
        &self.thickness
    }

    pub fn get_thickness_mut(&mut self)-> &mut Option<Thickness> {
        &mut self.thickness
    }

    pub fn set_thickness(&mut self, value:Thickness)-> &mut SideWall {
        self.thickness = Some(value);
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"c:thickness" => {
                            let mut obj = Thickness::default();
                            obj.set_attributes(reader, e);
                            self.set_thickness(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"c:sideWall" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "c:sideWall"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:sideWall
        write_start_tag(writer, "c:sideWall", vec![], false);

        // c:thickness
        match &self.thickness {
            Some(v) => {v.write_to(writer);},
            None => {}
        }

        write_end_tag(writer, "c:sideWall");
    }
}
