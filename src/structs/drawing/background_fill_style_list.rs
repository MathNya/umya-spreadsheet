use super::GradientFill;
use super::SolidFill;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct BackgroundFillStyleList {
    solid_fill: Vec<SolidFill>,
    gradient_fill_collection: Vec<GradientFill>,
}
impl BackgroundFillStyleList {
    pub fn get_solid_fill(&self) -> &Vec<SolidFill> {
        &self.solid_fill
    }

    pub fn get_solid_fill_mut(&mut self) -> &mut Vec<SolidFill> {
        &mut self.solid_fill
    }

    pub fn set_solid_fill(&mut self, value: Vec<SolidFill>) -> &mut Self {
        self.solid_fill = value;
        self
    }

    pub fn add_solid_fill(&mut self, value: SolidFill) -> &mut Self {
        self.solid_fill.push(value);
        self
    }

    pub fn get_gradient_fill_collection(&self) -> &Vec<GradientFill> {
        &self.gradient_fill_collection
    }

    pub fn get_gradient_fill_collectionl_mut(&mut self) -> &mut Vec<GradientFill> {
        &mut self.gradient_fill_collection
    }

    pub fn set_gradient_fill_collection(&mut self, value: Vec<GradientFill>) -> &mut Self {
        self.gradient_fill_collection = value;
        self
    }

    pub fn add_gradient_fill_collection(&mut self, value: GradientFill) -> &mut Self {
        self.gradient_fill_collection.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:bgFillStyleLst" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "bgFillStyleLst"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:bgFillStyleLst
        write_start_tag(writer, "a:bgFillStyleLst", vec![], false);

        // a:solidFill
        for v in &self.solid_fill {
            v.write_to(writer);
        }

        // a:gradFill
        for v in &self.gradient_fill_collection {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:bgFillStyleLst");
    }
}
