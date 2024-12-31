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
        TrueFalseValue,
        raw::RawRelationships,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct Fill {
    color:      StringValue,
    color_2:    StringValue,
    on:         TrueFalseValue,
    focus_size: StringValue,
    image:      Option<MediaObject>,
}

impl Fill {
    #[inline]
    #[must_use]
    pub fn get_color(&self) -> &str {
        self.color.get_value_str()
    }

    #[inline]
    pub fn set_color<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.color.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_color_2(&self) -> &str {
        self.color_2.get_value_str()
    }

    #[inline]
    pub fn set_color_2<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.color_2.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_on(&self) -> bool {
        self.on.get_value()
    }

    #[inline]
    pub fn set_on(&mut self, value: bool) -> &mut Self {
        self.on.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_focus_size(&self) -> &str {
        self.focus_size.get_value_str()
    }

    #[inline]
    pub fn set_focus_size<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.focus_size.set_value(value);
        self
    }

    #[inline]
    #[must_use]
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

    /// We allow the `unused_assignments` lint here, because the compiler is not
    /// smart enough to see, that defining `r_id_str` outside of the `if let
    /// Some()` is necessary to avoid lifetime errors.
    #[inline]
    #[allow(unused_assignments)]
    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        rel_list: &mut Vec<(String, String)>,
    ) {
        // v:fill
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.color.has_value() {
            attributes.push(("color", self.color.get_value_str()).into());
        }
        if self.color_2.has_value() {
            attributes.push(("color2", self.color_2.get_value_str()).into());
        }
        if self.on.has_value() {
            attributes.push(("on", self.on.get_value_string()).into());
        }
        if self.focus_size.has_value() {
            attributes.push(("focussize", self.focus_size.get_value_str()).into());
        }
        let mut r_id_str = String::new();
        if let Some(image) = &self.image {
            let r_id = image.get_rid(rel_list);
            r_id_str = format!("rId{r_id}");
            attributes.push(("o:title", image.get_image_title()).into());
            attributes.push(("o:relid", r_id_str.as_str()).into());
            attributes.push(("recolor", "t").into());
            attributes.push(("rotate", "t").into());
            attributes.push(("type", "frame").into());
        }
        write_start_tag(writer, "v:fill", attributes, true);
    }
}
