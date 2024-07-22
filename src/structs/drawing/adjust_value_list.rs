// a:avLst
use super::shape_guide::ShapeGuide;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct AdjustValueList {
    shape_guide_collection: Vec<ShapeGuide>,
}

impl AdjustValueList {
    pub fn get_shape_guide_collection(&self) -> &Vec<ShapeGuide> {
        &self.shape_guide_collection
    }

    pub fn get_shape_guide_collection_mut(&mut self) -> &mut Vec<ShapeGuide> {
        &mut self.shape_guide_collection
    }

    pub fn set_shape_guide_collection(&mut self, value: Vec<ShapeGuide>) {
        self.shape_guide_collection = value;
    }

    pub fn add_shape_guide_collection(&mut self, value: ShapeGuide) {
        self.shape_guide_collection.push(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:gd" {
                    let mut shape_guide = ShapeGuide::default();
                    shape_guide.set_name(get_attribute(e, b"name").unwrap());
                    shape_guide.set_fmla(get_attribute(e, b"fmla").unwrap());
                    self.add_shape_guide_collection(shape_guide);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:avLst" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:avLst")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:avLst
        if !self.shape_guide_collection.is_empty() {
            write_start_tag(writer, "a:avLst", vec![], false);
            for gd in &self.shape_guide_collection {
                gd.write_to(writer);
            }
            write_end_tag(writer, "a:avLst");
        } else {
            write_start_tag(writer, "a:avLst", vec![], true);
        }
    }
}
