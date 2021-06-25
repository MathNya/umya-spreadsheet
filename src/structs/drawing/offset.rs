// a:off
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Offset {
    x: usize,
    y: usize,
}
impl Offset {
    pub fn get_x(&self) -> &usize {
        &self.x
    }

    pub fn set_x(&mut self, value:usize) {
        self.x = value;
    }

    pub fn get_y(&self) -> &usize {
        &self.y
    }

    pub fn set_y(&mut self, value:usize) {
        self.y = value;
    }
    
    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        &mut self.set_x(get_attribute(e, b"x").unwrap().parse::<usize>().unwrap());
        &mut self.set_y(get_attribute(e, b"y").unwrap().parse::<usize>().unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:off
        write_start_tag(writer, "a:off", vec![
            ("x", &self.x.to_string()),
            ("y", &self.y.to_string()),
        ], true);
    }
}
