use super::Color;
use super::ConditionalFormatValueObject;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ColorScale {
    cfvo_collection: Vec<ConditionalFormatValueObject>,
    color_collection: Vec<Color>,
}

impl ColorScale {
    pub fn get_cfvo_collection(&self) -> &Vec<ConditionalFormatValueObject> {
        &self.cfvo_collection
    }

    pub fn set_cfvo_collection(&mut self, value: Vec<ConditionalFormatValueObject>) -> &mut Self {
        self.cfvo_collection = value;
        self
    }

    pub fn add_cfvo_collection(&mut self, value: ConditionalFormatValueObject) -> &mut Self {
        self.cfvo_collection.push(value);
        self
    }

    pub fn get_color_collection(&self) -> &Vec<Color> {
        &self.color_collection
    }

    pub fn set_color_collection(&mut self, value: Vec<Color>) -> &mut Self {
        self.color_collection = value;
        self
    }

    pub fn add_color_collection(&mut self, value: Color) -> &mut Self {
        self.color_collection.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"cfvo" => {
                        let mut obj = ConditionalFormatValueObject::default();
                        obj.set_attributes(reader, e, true);
                        self.cfvo_collection.push(obj);
                    }
                    b"color" => {
                        let mut obj = Color::default();
                        obj.set_attributes(reader, e, true);
                        self.color_collection.push(obj);
                    }
                    _ => (),
                }
            },
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"cfvo" => {
                        let mut obj = ConditionalFormatValueObject::default();
                        obj.set_attributes(reader, e, false);
                        self.cfvo_collection.push(obj);
                    }
                    b"color" => {
                        let mut obj = Color::default();
                        obj.set_attributes(reader, e, false);
                        self.color_collection.push(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"colorScale" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "colorScale")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // colorScale
        write_start_tag(writer, "colorScale", vec![], false);

        // cfvo
        for v in &self.cfvo_collection {
            v.write_to(writer);
        }

        // color
        for v in &self.color_collection {
            v.write_to_color(writer);
        }

        write_end_tag(writer, "colorScale");
    }
}
