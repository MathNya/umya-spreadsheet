// xdr:xfrm
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct Extents {
    cx: usize,
    cy: usize,
}
impl Extents {
   
    pub fn get_cx(&self) -> &usize {
        &self.cx
    }

    pub fn set_cx(&mut self, value:usize) {
        self.cx = value;
    }

    pub fn get_cy(&self) -> &usize {
        &self.cy
    }

    pub fn set_cy(&mut self, value:usize) {
        self.cy = value;
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        &mut self.set_cx(get_attribute(e, b"cx").unwrap().parse::<usize>().unwrap());
        &mut self.set_cy(get_attribute(e, b"cy").unwrap().parse::<usize>().unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:ext
        write_start_tag(writer, "a:ext", vec![
            ("cx", &self.cx.to_string()),
            ("cy", &self.cy.to_string()),
        ], true);
    }
}