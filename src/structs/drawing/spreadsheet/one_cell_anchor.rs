// xdr:oneCellAnchor
use super::Extent;
use super::MarkerType;
use super::Shape;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct OneCellAnchor {
    from_marker: MarkerType,
    extent: Extent,
    shape: Option<Shape>,
}
impl OneCellAnchor {
    pub fn get_from_marker(&self) -> &MarkerType {
        &self.from_marker
    }

    pub fn get_from_marker_mut(&mut self) -> &mut MarkerType {
        &mut self.from_marker
    }

    pub fn set_from_marker(&mut self, value: MarkerType) -> &mut OneCellAnchor {
        self.from_marker = value;
        self
    }

    pub fn get_extent(&self) -> &Extent {
        &self.extent
    }

    pub fn get_extent_mut(&mut self) -> &mut Extent {
        &mut self.extent
    }

    pub fn set_extent(&mut self, value: Extent) -> &mut OneCellAnchor {
        self.extent = value;
        self
    }

    pub fn get_shape(&self) -> &Option<Shape> {
        &self.shape
    }

    pub fn get_shape_mut(&mut self) -> &mut Option<Shape> {
        &mut self.shape
    }

    pub fn set_shape(&mut self, value: Shape) -> &mut OneCellAnchor {
        self.shape = Some(value);
        self
    }

    pub(crate) fn _adjustment_insert_row(&mut self, num_rows: &usize) {
        &mut self.from_marker._adjustment_insert_row(num_rows);
    }

    pub(crate) fn _adjustment_insert_colmun(&mut self, num_cols: &usize) {
        &mut self.from_marker._adjustment_insert_colmun(num_cols);
    }

    pub(crate) fn _adjustment_remove_row(&mut self, num_rows: &usize) {
        &mut self.from_marker._adjustment_remove_row(num_rows);
    }

    pub(crate) fn _adjustment_remove_colmun(&mut self, num_cols: &usize) {
        &mut self.from_marker._adjustment_remove_colmun(num_cols);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name() {
                    b"xdr:from" => {
                        self.from_marker.set_attributes(reader, e);
                    }
                    b"xdr:sp" => {
                        let mut obj = Shape::default();
                        obj.set_attributes(reader, e);
                        self.set_shape(obj);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name() {
                    b"xdr:ext" => {
                        self.extent.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"xdr:oneCellAnchor" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:oneCellAnchor"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // xdr:oneCellAnchor
        write_start_tag(writer, "xdr:oneCellAnchor", vec![], false);

        // xdr:from
        &self.from_marker.write_to_from(writer);

        // xdr:ext
        &self.extent.write_to(writer);

        // xdr:sp
        match &self.shape {
            Some(v) => v.write_to(writer, &0),
            None => {}
        }

        // xdr:clientData
        write_start_tag(writer, "xdr:clientData", vec![], true);

        write_end_tag(writer, "xdr:oneCellAnchor");
    }
}
