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
    BooleanValue,
    ObjectAnchor,
    StringValue,
    UInt32Value,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    structs::{
        MediaObject,
        raw::RawRelationships,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct EmbeddedObjectProperties {
    prog_id: StringValue,
    shape_id: UInt32Value,
    image: MediaObject,
    default_size: BooleanValue,
    auto_pict: BooleanValue,
    object_anchor: ObjectAnchor,
}

impl EmbeddedObjectProperties {
    #[inline]
    #[must_use]
    pub fn get_prog_id(&self) -> &str {
        self.prog_id.get_value_str()
    }

    #[inline]
    pub fn set_prog_id<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.prog_id.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_shape_id(&self) -> u32 {
        self.shape_id.get_value()
    }

    #[inline]
    pub fn set_shape_id(&mut self, value: u32) -> &mut Self {
        self.shape_id.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_image(&self) -> &MediaObject {
        &self.image
    }

    #[inline]
    pub fn get_image_mut(&mut self) -> &mut MediaObject {
        &mut self.image
    }

    #[inline]
    pub fn set_image(&mut self, value: MediaObject) {
        self.image = value;
    }

    #[inline]
    #[must_use]
    pub fn get_default_size(&self) -> bool {
        self.default_size.get_value()
    }

    #[inline]
    pub fn set_default_size(&mut self, value: bool) -> &mut Self {
        self.default_size.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_auto_pict(&self) -> bool {
        self.auto_pict.get_value()
    }

    #[inline]
    pub fn set_auto_pict(&mut self, value: bool) -> &mut Self {
        self.auto_pict.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_object_anchor(&self) -> &ObjectAnchor {
        &self.object_anchor
    }

    #[inline]
    pub fn get_object_anchor_mut(&mut self) -> &mut ObjectAnchor {
        &mut self.object_anchor
    }

    #[inline]
    pub fn set_object_anchor(&mut self, value: ObjectAnchor) -> &mut Self {
        self.object_anchor = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        relationships: &RawRelationships,
    ) {
        let r_id = get_attribute(e, b"r:id").unwrap();
        let attached_file = relationships.get_relationship_by_rid(&r_id).get_raw_file();

        self.get_image_mut()
            .set_image_name(attached_file.get_file_name());
        self.get_image_mut()
            .set_image_data(attached_file.get_file_data());

        set_string_from_xml!(self, e, default_size, "defaultSize");
        set_string_from_xml!(self, e, auto_pict, "autoPict");

        xml_read_loop!(
            reader,
                Event::Start(ref e) => {
                    if e.name().into_inner() == b"anchor" {
                        self.object_anchor.set_attributes(reader, e);
                    }
                },
                Event::End(ref e) => {
                    if e.name().into_inner() == b"objectPr" {
                        return
                    }
                },
                Event::Eof => panic!("Error: Could not find {} end element", "objectPr")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: usize) {
        // objectPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.default_size.has_value() {
            attributes.push(("defaultSize", self.default_size.get_value_string()));
        }
        if self.auto_pict.has_value() {
            attributes.push(("autoPict", self.auto_pict.get_value_string()));
        }
        let r_id_str = format!("rId{r_id}");
        attributes.push(("r:id", r_id_str.as_str()));
        write_start_tag(writer, "objectPr", attributes, false);

        // anchor
        self.object_anchor.write_to(writer);

        write_end_tag(writer, "objectPr");
    }
}
