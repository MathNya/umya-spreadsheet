use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    structs::{
        MediaObject,
        StringValue,
        raw::RawRelationships,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct ImageData {
    image: Option<MediaObject>,
    title: StringValue,
}

impl ImageData {
    #[inline]
    #[must_use]
    pub fn image(&self) -> Option<&MediaObject> {
        self.image.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use image()")]
    pub fn get_image(&self) -> Option<&MediaObject> {
        self.image()
    }

    #[inline]
    pub fn image_mut(&mut self) -> Option<&mut MediaObject> {
        self.image.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use image_mut()")]
    pub fn get_image_mut(&mut self) -> Option<&mut MediaObject> {
        self.image_mut()
    }

    #[inline]
    pub fn set_image(&mut self, value: MediaObject) -> &mut Self {
        self.image = Some(value);
        self
    }

    #[must_use]
    pub fn title(&self) -> &str {
        self.title.value_str()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use title()")]
    pub fn get_title(&self) -> &str {
        self.title()
    }

    pub fn set_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.title.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        if let Some(relid) = get_attribute(e, b"o:relid") {
            if let Some(rel) = drawing_relationships {
                let relationship = rel.relationship_by_rid(&relid);
                let mut obj = MediaObject::default();
                obj.set_image_name(relationship.raw_file().file_name());
                obj.set_image_data(relationship.raw_file().file_data());
                self.set_image(obj);
            }
        }

        set_string_from_xml!(self, e, title, "o:title");
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // v:imagedata
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if let Some(image) = &self.image {
            let r_id_str = format!("rId{}", image.rid(rel_list));
            attributes.push(("o:relid", r_id_str).into());
        }
        if self.title.has_value() {
            attributes.push(("o:title", self.title.value_str()).into());
        }

        write_start_tag(writer, "v:imagedata", attributes, true);
    }
}
