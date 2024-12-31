// a:xfrm
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use crate::{
    StringValue,
    reader::driver::{
        get_attribute,
        xml_read_loop,
    },
    structs::drawing::{
        Point2DType,
        PositiveSize2DType,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Transform2D {
    offset:        Point2DType,
    extents:       PositiveSize2DType,
    child_offset:  Option<Box<Point2DType>>,
    child_extents: Option<Box<PositiveSize2DType>>,
    rot:           StringValue,
    flip_v:        StringValue,
    flip_h:        StringValue,
}

impl Transform2D {
    #[inline]
    #[must_use]
    pub fn get_offset(&self) -> &Point2DType {
        &self.offset
    }

    #[inline]
    pub fn get_offset_mut(&mut self) -> &mut Point2DType {
        &mut self.offset
    }

    #[inline]
    pub fn set_offset(&mut self, value: Point2DType) {
        self.offset = value;
    }

    #[inline]
    #[must_use]
    pub fn get_extents(&self) -> &PositiveSize2DType {
        &self.extents
    }

    #[inline]
    pub fn get_extents_mut(&mut self) -> &mut PositiveSize2DType {
        &mut self.extents
    }

    #[inline]
    pub fn set_extents(&mut self, value: PositiveSize2DType) {
        self.extents = value;
    }

    #[inline]
    #[must_use]
    pub fn get_child_offset(&self) -> Option<&Point2DType> {
        self.child_offset.as_deref()
    }

    #[inline]
    pub fn get_child_offset_mut(&mut self) -> Option<&mut Point2DType> {
        self.child_offset.as_deref_mut()
    }

    #[inline]
    pub fn set_child_offset(&mut self, value: Point2DType) {
        self.child_offset = Some(Box::new(value));
    }

    #[inline]
    #[must_use]
    pub fn get_child_extents(&self) -> Option<&PositiveSize2DType> {
        self.child_extents.as_deref()
    }

    #[inline]
    pub fn get_child_extents_mut(&mut self) -> Option<&mut PositiveSize2DType> {
        self.child_extents.as_deref_mut()
    }

    #[inline]
    pub fn set_child_extents(&mut self, value: PositiveSize2DType) {
        self.child_extents = Some(Box::new(value));
    }

    #[inline]
    #[must_use]
    pub fn get_rot(&self) -> Option<&str> {
        self.rot.get_value()
    }

    #[inline]
    pub fn set_rot<S: Into<String>>(&mut self, value: S) {
        self.rot.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn get_flip_v(&self) -> Option<&str> {
        self.flip_v.get_value()
    }

    #[inline]
    pub fn set_flip_v<S: Into<String>>(&mut self, value: S) {
        self.flip_v.set_value(value);
    }

    #[inline]
    #[must_use]
    pub fn get_flip_h(&self) -> Option<&str> {
        self.flip_h.get_value()
    }

    #[inline]
    pub fn set_flip_h<S: Into<String>>(&mut self, value: S) {
        self.flip_h.set_value(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        if let Some(v) = get_attribute(e, b"rot") {
            self.set_rot(v);
        }

        if let Some(v) = get_attribute(e, b"flipH") {
            self.set_flip_h(v);
        }

        if let Some(v) = get_attribute(e, b"flipV") {
            self.set_flip_v(v);
        }

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
                    b"a:chOff" => {
                        let mut obj = Point2DType::default();
                        obj.set_attributes(reader, e);
                        self.set_child_offset(obj);
                    }
                    b"a:chExt" => {
                        let mut obj = PositiveSize2DType::default();
                        obj.set_attributes(reader, e);
                        self.set_child_extents(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:xfrm" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:xfrm")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:xfrm
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if let Some(v) = self.rot.get_value() {
            attributes.push(("rot", v).into());
        }
        if let Some(v) = self.flip_h.get_value() {
            attributes.push(("flipH", v).into());
        }
        if let Some(v) = self.flip_v.get_value() {
            attributes.push(("flipV", v).into());
        }
        write_start_tag(writer, "a:xfrm", attributes, false);

        // a:off
        self.offset.write_to_off(writer);

        // a:ext
        self.extents.write_to_ext(writer);

        // a:chOff
        if let Some(v) = &self.child_offset {
            v.write_to_ch_off(writer);
        }

        // a:chExt
        if let Some(v) = &self.child_extents {
            v.write_to_ch_ext(writer);
        }

        write_end_tag(writer, "a:xfrm");
    }
}
