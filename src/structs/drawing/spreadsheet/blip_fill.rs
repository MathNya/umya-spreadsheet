// xdr:blipFill
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::super::{
    super::BooleanValue,
    Blip,
    SourceRectangle,
    Stretch,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    structs::raw::RawRelationships,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct BlipFill {
    rotate_with_shape: BooleanValue,
    blip:              Blip,
    source_rectangle:  Option<Box<SourceRectangle>>,
    stretch:           Stretch,
}

impl BlipFill {
    #[inline]
    #[must_use]
    pub fn get_rotate_with_shape(&self) -> bool {
        self.rotate_with_shape.value()
    }

    #[inline]
    pub fn set_rotate_with_shape(&mut self, value: bool) -> &mut BlipFill {
        self.rotate_with_shape.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_source_rectangle(&self) -> Option<&SourceRectangle> {
        self.source_rectangle.as_deref()
    }

    #[inline]
    pub fn get_source_rectangle_mut(&mut self) -> Option<&mut SourceRectangle> {
        self.source_rectangle.as_deref_mut()
    }

    #[inline]
    pub fn set_source_rectangle(&mut self, value: SourceRectangle) -> &mut BlipFill {
        self.source_rectangle = Some(Box::new(value));
        self
    }

    #[inline]
    #[must_use]
    pub fn get_blip(&self) -> &Blip {
        &self.blip
    }

    #[inline]
    pub fn get_blip_mut(&mut self) -> &mut Blip {
        &mut self.blip
    }

    #[inline]
    pub fn set_blip(&mut self, value: Blip) -> &mut BlipFill {
        self.blip = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn get_stretch(&self) -> &Stretch {
        &self.stretch
    }

    #[inline]
    pub fn get_stretch_mut(&mut self) -> &mut Stretch {
        &mut self.stretch
    }

    #[inline]
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
        set_string_from_xml!(self, e, rotate_with_shape, "rotWithShape");

        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"a:blip" => {
                        self.blip
                            .set_attributes(reader, e, drawing_relationships.unwrap(), false);
                        }
                    b"a:stretch" => {
                        self.stretch.set_attributes(reader, e);
                    }
                    _ => (),
                }
            },
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"a:blip" => {
                        self.blip
                            .set_attributes(reader, e, drawing_relationships.unwrap(), true);
                        }
                    b"a:srcRect" => {
                        let mut source_rectangle = SourceRectangle::default();
                        source_rectangle.set_attributes(reader, e);
                        self.set_source_rectangle(source_rectangle);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xdr:blipFill" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xdr:blipFill")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // xdr:blipFill
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.rotate_with_shape.has_value() {
            attributes.push(("rotWithShape", self.rotate_with_shape.value_string()).into());
        }
        write_start_tag(writer, "xdr:blipFill", attributes, false);

        // a:blip
        self.blip.write_to(writer, rel_list);

        // a:srcRect
        if let Some(v) = &self.source_rectangle {
            v.write_to(writer);
        }

        // a:stretch
        self.stretch.write_to(writer);

        write_end_tag(writer, "xdr:blipFill");
    }
}
