// xdr:grpSp
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
    GroupShapeProperties,
    NonVisualGroupShapeProperties,
    Picture,
    Shape,
};
use crate::{
    reader::driver::xml_read_loop,
    structs::raw::RawRelationships,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct GroupShape {
    non_visual_group_shape_properties: NonVisualGroupShapeProperties,
    group_shape_properties:            GroupShapeProperties,
    picture_collection:                Vec<Picture>,
    shape_collection:                  Vec<Shape>,
}

impl GroupShape {
    #[inline]
    #[must_use]
    pub fn non_visual_group_shape_properties(&self) -> &NonVisualGroupShapeProperties {
        &self.non_visual_group_shape_properties
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use non_visual_group_shape_properties()")]
    pub fn get_non_visual_group_shape_properties(&self) -> &NonVisualGroupShapeProperties {
        self.non_visual_group_shape_properties()
    }

    #[inline]
    pub fn non_visual_group_shape_properties_mut(&mut self) -> &mut NonVisualGroupShapeProperties {
        &mut self.non_visual_group_shape_properties
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use non_visual_group_shape_properties_mut()")]
    pub fn get_non_visual_group_shape_properties_mut(
        &mut self,
    ) -> &mut NonVisualGroupShapeProperties {
        self.non_visual_group_shape_properties_mut()
    }

    #[inline]
    pub fn set_non_visual_group_shape_properties(&mut self, value: NonVisualGroupShapeProperties) {
        self.non_visual_group_shape_properties = value;
    }

    #[inline]
    #[must_use]
    pub fn group_shape_properties(&self) -> &GroupShapeProperties {
        &self.group_shape_properties
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use group_shape_properties()")]
    pub fn get_group_shape_properties(&self) -> &GroupShapeProperties {
        self.group_shape_properties()
    }

    #[inline]
    pub fn group_shape_properties_mut(&mut self) -> &mut GroupShapeProperties {
        &mut self.group_shape_properties
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use group_shape_properties_mut()")]
    pub fn get_group_shape_properties_mut(&mut self) -> &mut GroupShapeProperties {
        self.group_shape_properties_mut()
    }

    #[inline]
    pub fn set_group_shape_properties(&mut self, value: GroupShapeProperties) {
        self.group_shape_properties = value;
    }

    #[inline]
    #[must_use]
    pub fn picture_collection(&self) -> &[Picture] {
        &self.picture_collection
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use picture_collection()")]
    pub fn get_picture_collection(&self) -> &[Picture] {
        self.picture_collection()
    }

    #[inline]
    pub fn picture_collection_mut(&mut self) -> &mut Vec<Picture> {
        &mut self.picture_collection
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use picture_collection_mut()")]
    pub fn get_picture_collection_mut(&mut self) -> &mut Vec<Picture> {
        self.picture_collection_mut()
    }

    #[inline]
    pub fn add_picture_collection(&mut self, value: Picture) {
        self.picture_collection.push(value);
    }

    #[inline]
    #[must_use]
    pub fn shape_collection(&self) -> &[Shape] {
        &self.shape_collection
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use shape_collection()")]
    pub fn get_shape_collection(&self) -> &[Shape] {
        self.shape_collection()
    }

    #[inline]
    pub fn shape_collection_mut(&mut self) -> &mut Vec<Shape> {
        &mut self.shape_collection
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use shape_collection_mut()")]
    pub fn get_shape_collection_mut(&mut self) -> &mut Vec<Shape> {
        self.shape_collection_mut()
    }

    #[inline]
    pub fn add_shape_collection(&mut self, value: Shape) {
        self.shape_collection.push(value);
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
                    b"xdr:nvGrpSpPr" => {
                        self.non_visual_group_shape_properties.set_attributes(reader, e);
                    }
                    b"xdr:grpSpPr" => {
                        self.group_shape_properties.set_attributes(reader, e);
                    }
                    b"xdr:pic" => {
                        let mut obj = Picture::default();
                        obj.set_attributes(reader, e, drawing_relationships);
                        self.add_picture_collection(obj);
                    }
                    b"xdr:sp" => {
                        let mut obj = Shape::default();
                        obj.set_attributes(reader, e, drawing_relationships);
                        self.add_shape_collection(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:grpSp" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:grpSp")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // xdr:grpSp
        write_start_tag(writer, "xdr:grpSp", vec![], false);

        // xdr:nvGrpSpPr
        self.non_visual_group_shape_properties.write_to(writer);

        // xdr:grpSpPr
        self.group_shape_properties.write_to(writer);

        // xdr:pic
        for obj in &self.picture_collection {
            obj.write_to(writer, rel_list);
        }

        // xdr:sp
        for obj in &self.shape_collection {
            obj.write_to(writer, rel_list, 0);
        }

        write_end_tag(writer, "xdr:grpSp");
    }
}
