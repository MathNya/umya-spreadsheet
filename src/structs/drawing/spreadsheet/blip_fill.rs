// xdr:blipFill
use super::super::super::BooleanValue;
use super::super::Blip;
use super::super::SourceRectangle;
use super::super::Stretch;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::raw::RawRelationships;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct BlipFill {
    rotate_with_shape: BooleanValue,
    blip: Blip,
    source_rectangle: Option<SourceRectangle>,
    stretch: Stretch,
}
impl BlipFill {
    pub fn get_rotate_with_shape(&self) -> &bool {
        self.rotate_with_shape.get_value()
    }

    pub fn set_rotate_with_shape(&mut self, value: bool) -> &mut BlipFill {
        self.rotate_with_shape.set_value(value);
        self
    }

    pub fn get_source_rectangle(&self) -> &Option<SourceRectangle> {
        &self.source_rectangle
    }

    pub fn get_source_rectangle_mut(&mut self) -> &mut Option<SourceRectangle> {
        &mut self.source_rectangle
    }

    pub fn set_source_rectangle(&mut self, value: SourceRectangle) -> &mut BlipFill {
        self.source_rectangle = Some(value);
        self
    }

    pub fn get_blip(&self) -> &Blip {
        &self.blip
    }

    pub fn get_blip_mut(&mut self) -> &mut Blip {
        &mut self.blip
    }

    pub fn set_blip(&mut self, value: Blip) -> &mut BlipFill {
        self.blip = value;
        self
    }

    pub fn get_stretch(&self) -> &Stretch {
        &self.stretch
    }

    pub fn get_stretch_mut(&mut self) -> &mut Stretch {
        &mut self.stretch
    }

    pub fn set_stretch(&mut self, value: Stretch) -> &mut BlipFill {
        self.stretch = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        let mut buf = Vec::new();

        match get_attribute(e, b"rotWithShape") {
            Some(v) => {
                self.rotate_with_shape.set_value_string(v);
            }
            None => {}
        }

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:blip" => {
                        self.blip
                            .set_attributes(reader, e, drawing_relationships.unwrap());
                    }
                    b"a:stretch" => {
                        self.stretch.set_attributes(reader, e);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:blip" => {
                        self.blip
                            .set_attributes(reader, e, drawing_relationships.unwrap());
                    }
                    b"a:srcRect" => {
                        let mut source_rectangle = SourceRectangle::default();
                        source_rectangle.set_attributes(reader, e);
                        self.set_source_rectangle(source_rectangle);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xdr:blipFill" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xdr:blipFill"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &i32) {
        // xdr:blipFill
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if &self.rotate_with_shape.has_value() == &true {
            attributes.push(("rotWithShape", self.rotate_with_shape.get_value_string()))
        }
        write_start_tag(writer, "xdr:blipFill", attributes, false);

        // a:blip
        let _ = &self.blip.write_to(writer, r_id);

        // a:srcRect
        match &self.source_rectangle {
            Some(v) => v.write_to(writer),
            None => {}
        }

        // a:stretch
        let _ = &self.stretch.write_to(writer);

        write_end_tag(writer, "xdr:blipFill");
    }
}
