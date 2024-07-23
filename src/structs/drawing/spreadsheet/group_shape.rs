// xdr:grpSp
use super::GroupShapeProperties;
use super::NonVisualGroupShapeProperties;
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
pub struct GroupShape {
    non_visual_group_shape_properties: NonVisualGroupShapeProperties,
    group_shape_properties: GroupShapeProperties,
    picture_collection: Vec<Picture>,
    shape_collection: Vec<Shape>,
}

impl GroupShape {
    pub fn get_non_visual_group_shape_properties(&self) -> &NonVisualGroupShapeProperties {
        &self.non_visual_group_shape_properties
    }

    pub fn get_non_visual_group_shape_properties_mut(
        &mut self,
    ) -> &mut NonVisualGroupShapeProperties {
        &mut self.non_visual_group_shape_properties
    }

    pub fn set_non_visual_group_shape_properties(&mut self, value: NonVisualGroupShapeProperties) {
        self.non_visual_group_shape_properties = value;
    }

    pub fn get_group_shape_properties(&self) -> &GroupShapeProperties {
        &self.group_shape_properties
    }

    pub fn get_group_shape_properties_mut(&mut self) -> &mut GroupShapeProperties {
        &mut self.group_shape_properties
    }

    pub fn set_group_shape_properties(&mut self, value: GroupShapeProperties) {
        self.group_shape_properties = value;
    }

    pub fn get_picture_collection(&self) -> &Vec<Picture> {
        &self.picture_collection
    }

    pub fn get_picture_collection_mut(&mut self) -> &mut Vec<Picture> {
        &mut self.picture_collection
    }

    pub fn add_picture_collection(&mut self, value: Picture) {
        self.picture_collection.push(value);
    }

    pub fn get_shape_collection(&self) -> &Vec<Shape> {
        &self.shape_collection
    }

    pub fn get_shape_collection_mut(&mut self) -> &mut Vec<Shape> {
        &mut self.shape_collection
    }

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
        &self.non_visual_group_shape_properties.write_to(writer);

        // xdr:grpSpPr
        &self.group_shape_properties.write_to(writer);

        // xdr:pic
        for obj in &self.picture_collection {
            obj.write_to(writer, rel_list);
        }

        // xdr:sp
        for obj in &self.shape_collection {
            obj.write_to(writer, rel_list, &0);
        }

        write_end_tag(writer, "xdr:grpSp");
    }
}
