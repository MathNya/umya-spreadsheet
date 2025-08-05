use crate::reader::driver::*;
use crate::structs::raw::RawRelationships;
use crate::structs::MediaObject;
use crate::structs::StringValue;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct ImageData {
    image: Option<MediaObject>,
    title: StringValue,
}

impl ImageData {
    #[inline]
    pub fn get_image(&self) -> Option<&MediaObject> {
        self.image.as_ref()
    }

    #[inline]
    pub fn get_image_mut(&mut self) -> Option<&mut MediaObject> {
        self.image.as_mut()
    }

    #[inline]
    pub fn set_image(&mut self, value: MediaObject) -> &mut Self {
        self.image = Some(value);
        self
    }

    pub fn get_title(&self) -> &str {
        self.title.get_value_str()
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
                let relationship = rel.get_relationship_by_rid(&relid);
                let mut obj = MediaObject::default();
                obj.set_image_name(relationship.get_raw_file().get_file_name());
                obj.set_image_data(relationship.get_raw_file().get_file_data());
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
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let mut r_id_str = String::from("");
        if let Some(image) = &self.image {
            r_id_str = format!("rId{}", image.get_rid(rel_list));
            attributes.push(("o:relid", &r_id_str));
        }
        if self.title.has_value() {
            attributes.push(("o:title", self.title.get_value_str()));
        }

        write_start_tag(writer, "v:imagedata", attributes, true);
    }
}
