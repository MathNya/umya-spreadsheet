use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::structs::raw::RawRelationships;
use crate::structs::MediaObject;
use crate::structs::StringValue;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ImageData {
    image: MediaObject,
    title: StringValue,
}

impl ImageData {
    #[inline]
    pub fn get_image(&self) -> &MediaObject {
        &self.image
    }

    #[inline]
    pub fn get_image_mut(&mut self) -> &mut MediaObject {
        &mut self.image
    }

    #[inline]
    pub fn set_image(&mut self, value: MediaObject) -> &mut Self {
        self.image = value;
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
                self.get_image_mut()
                    .set_image_name(relationship.get_raw_file().get_file_name());
                self.get_image_mut()
                    .set_image_data(relationship.get_raw_file().get_file_data().clone());
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
        let r_id = &self.image.get_rid(rel_list);
        r_id_str = format!("rId{}", r_id);
        attributes.push(("o:relid", r_id_str.as_str()));
        if self.title.has_value() {
            attributes.push(("o:title", self.title.get_value_str()));
        }

        write_start_tag(writer, "v:imagedata", attributes, true);
    }
}
