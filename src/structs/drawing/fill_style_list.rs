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
    GradientFill,
    SolidFill,
};
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct FillStyleList {
    solid_fill: Vec<SolidFill>,
    gradient_fill_collection: Vec<GradientFill>,
}

impl FillStyleList {
    #[inline]
    #[must_use]
    pub fn get_solid_fill(&self) -> &[SolidFill] {
        &self.solid_fill
    }

    #[inline]
    pub fn get_solid_fill_mut(&mut self) -> &mut Vec<SolidFill> {
        &mut self.solid_fill
    }

    #[inline]
    pub fn set_solid_fill(&mut self, value: impl Into<Vec<SolidFill>>) -> &mut Self {
        self.solid_fill = value.into();
        self
    }

    #[inline]
    pub fn add_solid_fill(&mut self, value: SolidFill) -> &mut Self {
        self.solid_fill.push(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_gradient_fill_collection(&self) -> &[GradientFill] {
        &self.gradient_fill_collection
    }

    #[inline]
    pub fn get_gradient_fill_collectionl_mut(&mut self) -> &mut Vec<GradientFill> {
        &mut self.gradient_fill_collection
    }

    #[inline]
    pub fn set_gradient_fill_collection(
        &mut self,
        value: impl Into<Vec<GradientFill>>,
    ) -> &mut Self {
        self.gradient_fill_collection = value.into();
        self
    }

    #[inline]
    pub fn add_gradient_fill_collection(&mut self, value: GradientFill) -> &mut Self {
        self.gradient_fill_collection.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"a:solidFill" => {
                        let mut obj = SolidFill::default();
                        obj.set_attributes(reader, e);
                        self.solid_fill.push(obj);
                    }
                    b"a:gradFill" => {
                        let mut obj = GradientFill::default();
                        obj.set_attributes(reader, e);
                        self.gradient_fill_collection.push(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:fillStyleLst" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "fillStyleLst")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:fillStyleLst
        write_start_tag(writer, "a:fillStyleLst", vec![], false);

        // a:solidFill
        for v in &self.solid_fill {
            v.write_to(writer);
        }

        // a:gradFill
        for v in &self.gradient_fill_collection {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:fillStyleLst");
    }
}
