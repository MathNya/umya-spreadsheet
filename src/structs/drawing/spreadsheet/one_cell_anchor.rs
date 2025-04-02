// xdr:oneCellAnchor
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    Extent,
    GroupShape,
    MarkerType,
    Picture,
    Shape,
};
use crate::{
    reader::driver::xml_read_loop,
    structs::raw::RawRelationships,
    traits::AdjustmentCoordinate,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct OneCellAnchor {
    from_marker: MarkerType,
    extent:      Extent,
    group_shape: Option<Box<GroupShape>>,
    shape:       Option<Box<Shape>>,
    picture:     Option<Box<Picture>>,
}

impl OneCellAnchor {
    #[inline]
    #[must_use]
    pub fn from_marker(&self) -> &MarkerType {
        &self.from_marker
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use from_marker()")]
    pub fn get_from_marker(&self) -> &MarkerType {
        self.from_marker()
    }

    #[inline]
    pub fn from_marker_mut(&mut self) -> &mut MarkerType {
        &mut self.from_marker
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use from_marker_mut()")]
    pub fn get_from_marker_mut(&mut self) -> &mut MarkerType {
        self.from_marker_mut()
    }

    #[inline]
    pub fn set_from_marker(&mut self, value: MarkerType) -> &mut OneCellAnchor {
        self.from_marker = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn extent(&self) -> &Extent {
        &self.extent
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use extent()")]
    pub fn get_extent(&self) -> &Extent {
        self.extent()
    }

    #[inline]
    pub fn extent_mut(&mut self) -> &mut Extent {
        &mut self.extent
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use extent_mut()")]
    pub fn get_extent_mut(&mut self) -> &mut Extent {
        self.extent_mut()
    }

    #[inline]
    pub fn set_extent(&mut self, value: Extent) -> &mut OneCellAnchor {
        self.extent = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn group_shape(&self) -> Option<&GroupShape> {
        self.group_shape.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use group_shape()")]
    pub fn get_group_shape(&self) -> Option<&GroupShape> {
        self.group_shape()
    }

    #[inline]
    pub fn group_shape_mut(&mut self) -> Option<&mut GroupShape> {
        self.group_shape.as_deref_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use group_shape_mut()")]
    pub fn get_group_shape_mut(&mut self) -> Option<&mut GroupShape> {
        self.group_shape_mut()
    }

    #[inline]
    pub fn set_group_shape(&mut self, value: GroupShape) -> &mut Self {
        self.group_shape = Some(Box::new(value));
        self
    }

    #[inline]
    #[must_use]
    pub fn shape(&self) -> Option<&Shape> {
        self.shape.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use shape()")]
    pub fn get_shape(&self) -> Option<&Shape> {
        self.shape()
    }

    #[inline]
    pub fn shape_mut(&mut self) -> Option<&mut Shape> {
        self.shape.as_deref_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use shape_mut()")]
    pub fn get_shape_mut(&mut self) -> Option<&mut Shape> {
        self.shape_mut()
    }

    #[inline]
    pub fn set_shape(&mut self, value: Shape) -> &mut OneCellAnchor {
        self.shape = Some(Box::new(value));
        self
    }

    #[inline]
    #[must_use]
    pub fn picture(&self) -> Option<&Picture> {
        self.picture.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use picture()")]
    pub fn get_picture(&self) -> Option<&Picture> {
        self.picture()
    }

    #[inline]
    pub fn picture_mut(&mut self) -> Option<&mut Picture> {
        self.picture.as_deref_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use picture_mut()")]
    pub fn get_picture_mut(&mut self) -> Option<&mut Picture> {
        self.picture_mut()
    }

    #[inline]
    pub fn set_picture(&mut self, value: Picture) -> &mut Self {
        self.picture = Some(Box::new(value));
        self
    }

    #[inline]
    pub(crate) fn is_image(&self) -> bool {
        self.picture.is_some() || self.group_shape.is_some()
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
                    b"xdr:grpSp" => {
                        let mut obj = GroupShape::default();
                        obj.set_attributes(reader, e, drawing_relationships);
                        self.set_group_shape(obj);
                    }
                    b"xdr:sp" => {
                        let mut obj = Shape::default();
                        obj.set_attributes(reader, e, drawing_relationships);
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
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:oneCellAnchor")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // xdr:oneCellAnchor
        write_start_tag(writer, "xdr:oneCellAnchor", vec![], false);

        // xdr:from
        self.from_marker.write_to_from(writer);

        // xdr:ext
        self.extent.write_to(writer);

        // xdr:grpSp
        if let Some(v) = &self.group_shape {
            v.write_to(writer, rel_list);
        }

        // xdr:sp
        if let Some(v) = &self.shape {
            v.write_to(writer, rel_list, 0);
        }

        // xdr:pic
        if let Some(v) = &self.picture {
            v.write_to(writer, rel_list);
        }

        // xdr:clientData
        write_start_tag(writer, "xdr:clientData", vec![], true);

        write_end_tag(writer, "xdr:oneCellAnchor");
    }
}
impl AdjustmentCoordinate for OneCellAnchor {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.from_marker.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.from_marker.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    #[inline]
    fn is_remove_coordinate(
        &self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) -> bool {
        self.from_marker.is_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        )
    }
}
