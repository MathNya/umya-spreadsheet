// xdr:oneCellAnchor
use super::Extent;
use super::MarkerType;
use super::Picture;
use super::Shape;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::raw::RawRelationships;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct OneCellAnchor {
    from_marker: MarkerType,
    extent: Extent,
    shape: Option<Shape>,
    picture: Option<Picture>,
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

    pub fn get_shape(&self) -> Option<&Shape> {
        self.shape.as_ref()
    }

    pub fn get_shape_mut(&mut self) -> Option<&mut Shape> {
        self.shape.as_mut()
    }

    pub fn set_shape(&mut self, value: Shape) -> &mut OneCellAnchor {
        self.shape = Some(value);
        self
    }

    pub fn get_picture(&self) -> Option<&Picture> {
        self.picture.as_ref()
    }

    pub fn get_picture_mut(&mut self) -> Option<&mut Picture> {
        self.picture.as_mut()
    }

    pub fn set_picture(&mut self, value: Picture) -> &mut Self {
        self.picture = Some(value);
        self
    }

    pub(crate) fn _adjustment_insert_row(&mut self, num_rows: &u32) {
        self.from_marker._adjustment_insert_row(num_rows);
    }

    pub(crate) fn _adjustment_insert_column(&mut self, num_cols: &u32) {
        self.from_marker._adjustment_insert_column(num_cols);
    }

    pub(crate) fn _adjustment_remove_row(&mut self, num_rows: &u32) {
        self.from_marker._adjustment_remove_row(num_rows);
    }

    pub(crate) fn _adjustment_remove_column(&mut self, num_cols: &u32) {
        self.from_marker._adjustment_remove_column(num_cols);
    }

    pub(crate) fn is_image(&self) -> bool {
        self.picture.is_some()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"xdr:from" => {
                        self.from_marker.set_attributes(reader, e);
                    }
                    b"xdr:sp" => {
                        let mut obj = Shape::default();
                        obj.set_attributes(reader, e);
                        self.set_shape(obj);
                    }
                    b"xdr:pic" => {
                        let mut obj = Picture::default();
                        obj.set_attributes(reader, e, drawing_relationships);
                        self.set_picture(obj);
                    }
                    _ => (),
                }
            },
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"xdr:ext" {
                    self.extent.set_attributes(reader, e);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:oneCellAnchor" {
                    return
                }
            },
            Event::Eof => panic!("Error not find {} end element", "xdr:oneCellAnchor")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &mut i32) {
        // xdr:oneCellAnchor
        write_start_tag(writer, "xdr:oneCellAnchor", vec![], false);

        // xdr:from
        let _ = &self.from_marker.write_to_from(writer);

        // xdr:ext
        let _ = &self.extent.write_to(writer);

        // xdr:sp
        if let Some(v) = &self.shape {
            v.write_to(writer, &0)
        }

        // xdr:pic
        if let Some(v) = &self.picture {
            v.write_to(writer, r_id);
            *r_id += 1i32;
        }

        // xdr:clientData
        write_start_tag(writer, "xdr:clientData", vec![], true);

        write_end_tag(writer, "xdr:oneCellAnchor");
    }
}
