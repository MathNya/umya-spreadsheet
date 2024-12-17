use crate::reader::driver::*;
use crate::structs::raw::RawRelationships;
use crate::structs::MediaObject;
use crate::structs::StringValue;
use crate::structs::TrueFalseValue;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Fill {
    color: StringValue,
    color_2: StringValue,
    on: TrueFalseValue,
    focus_size: StringValue,
    image: Option<MediaObject>,
}

impl Fill {
    #[inline]
    pub fn get_color(&self) -> &str {
        self.color.get_value_str()
    }

    #[inline]
    pub fn set_color<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.color.set_value(value);
        self
    }

    #[inline]
    pub fn get_color_2(&self) -> &str {
        self.color_2.get_value_str()
    }

    #[inline]
    pub fn set_color_2<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.color_2.set_value(value);
        self
    }

    #[inline]
    pub fn get_on(&self) -> bool {
        self.on.get_value()
    }

    #[inline]
    pub fn set_on(&mut self, value: bool) -> &mut Self {
        self.on.set_value(value);
        self
    }

    #[inline]
    pub fn get_focus_size(&self) -> &str {
        self.focus_size.get_value_str()
    }

    #[inline]
    pub fn set_focus_size<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.focus_size.set_value(value);
        self
    }

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

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
        drawing_relationships: Option<&RawRelationships>,
    ) {
        set_string_from_xml!(self, e, color, "color");
        set_string_from_xml!(self, e, color_2, "color2");
        set_string_from_xml!(self, e, on, "on");
        set_string_from_xml!(self, e, focus_size, "focussize");

        if let Some(relid) = get_attribute(e, b"o:relid") {
            if let Some(rel) = drawing_relationships {
                let relationship = rel.get_relationship_by_rid(&relid);
                let mut obj = MediaObject::default();
                obj.set_image_title(get_attribute(e, b"o:title").unwrap());
                obj.set_image_name(relationship.get_raw_file().get_file_name());
                obj.set_image_data(relationship.get_raw_file().get_file_data());
                self.set_image(obj);
            }
        }
    }

    #[inline]
    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // v:fill
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.color.has_value() {
            attributes.push(("color", self.color.get_value_str()));
        }
        if self.color_2.has_value() {
            attributes.push(("color2", self.color_2.get_value_str()));
        }
        if self.on.has_value() {
            attributes.push(("on", self.on.get_value_string()));
        }
        if self.focus_size.has_value() {
            attributes.push(("focussize", self.focus_size.get_value_str()));
        }
        let mut _r_id_str = String::from("");
        if let Some(image) = &self.image {
            let r_id = image.get_rid(rel_list);
            _r_id_str = format!("rId{}", r_id);
            attributes.push(("o:title", image.get_image_title()));
            attributes.push(("o:relid", _r_id_str.as_str()));
            attributes.push(("recolor", "t"));
            attributes.push(("rotate", "t"));
            attributes.push(("type", "frame"));
        }
        write_start_tag(writer, "v:fill", attributes, true);
    }
}
