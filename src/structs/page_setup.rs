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
        EnumValue,
        OrientationValues,
        UInt32Value,
        raw::RawRelationships,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct PageSetup {
    paper_size:     UInt32Value,
    orientation:    EnumValue<OrientationValues>,
    scale:          UInt32Value,
    fit_to_height:  UInt32Value,
    fit_to_width:   UInt32Value,
    horizontal_dpi: UInt32Value,
    vertical_dpi:   UInt32Value,
    object_data:    Option<Vec<u8>>,
}

impl PageSetup {
    #[inline]
    #[must_use]
    pub fn get_paper_size(&self) -> u32 {
        self.paper_size.value()
    }

    #[inline]
    pub fn set_paper_size(&mut self, value: u32) -> &mut Self {
        self.paper_size.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_orientation(&self) -> &OrientationValues {
        self.orientation.get_value()
    }

    #[inline]
    pub fn set_orientation(&mut self, value: OrientationValues) -> &mut Self {
        self.orientation.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_scale(&self) -> u32 {
        self.scale.value()
    }

    #[inline]
    pub fn set_scale(&mut self, value: u32) -> &mut Self {
        self.scale.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_fit_to_height(&self) -> u32 {
        self.fit_to_height.value()
    }

    #[inline]
    pub fn set_fit_to_height(&mut self, value: u32) -> &mut Self {
        self.fit_to_height.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_fit_to_width(&self) -> u32 {
        self.fit_to_width.value()
    }

    #[inline]
    pub fn set_fit_to_width(&mut self, value: u32) -> &mut Self {
        self.fit_to_width.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_horizontal_dpi(&self) -> u32 {
        self.horizontal_dpi.value()
    }

    #[inline]
    pub fn set_horizontal_dpi(&mut self, value: u32) -> &mut Self {
        self.horizontal_dpi.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_vertical_dpi(&self) -> u32 {
        self.vertical_dpi.value()
    }

    #[inline]
    pub fn set_vertical_dpi(&mut self, value: u32) -> &mut Self {
        self.vertical_dpi.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_object_data(&self) -> Option<&[u8]> {
        self.object_data.as_deref()
    }

    #[inline]
    pub fn get_object_data_mut(&mut self) -> Option<&mut Vec<u8>> {
        self.object_data.as_mut()
    }

    #[inline]
    pub fn set_object_data(&mut self, value: impl Into<Vec<u8>>) -> &mut Self {
        self.object_data = Some(value.into());
        self
    }

    #[inline]
    pub(crate) fn has_param(&self) -> bool {
        self.paper_size.has_value()
            || self.orientation.has_value()
            || self.scale.has_value()
            || self.fit_to_height.has_value()
            || self.fit_to_width.has_value()
            || self.horizontal_dpi.has_value()
            || self.vertical_dpi.has_value()
            || self.object_data.is_some()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
        relationships: Option<&RawRelationships>,
    ) {
        set_string_from_xml!(self, e, paper_size, "paperSize");
        set_string_from_xml!(self, e, orientation, "orientation");
        set_string_from_xml!(self, e, scale, "scale");
        set_string_from_xml!(self, e, fit_to_height, "fitToHeight");
        set_string_from_xml!(self, e, fit_to_width, "fitToWidth");
        set_string_from_xml!(self, e, horizontal_dpi, "horizontalDpi");
        set_string_from_xml!(self, e, vertical_dpi, "verticalDpi");

        if let Some(r_id) = get_attribute(e, b"r:id") {
            let attached_file = relationships
                .unwrap()
                .get_relationship_by_rid(&r_id)
                .get_raw_file();
            self.set_object_data(attached_file.get_file_data());
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &mut usize) {
        if self.has_param() {
            // pageSetup
            let r_id_str = format!("rId{r_id}");
            let mut attributes: crate::structs::AttrCollection = Vec::new();
            let paper_size = self.paper_size.value_string();
            if self.paper_size.has_value() {
                attributes.push(("paperSize", &paper_size).into());
            }
            let scale = self.scale.value_string();
            if self.scale.has_value() {
                attributes.push(("scale", &scale).into());
            }
            let orientation = self.orientation.get_value_string();
            if self.orientation.has_value() {
                attributes.push(("orientation", orientation).into());
            }
            let fit_to_height = self.fit_to_height.value_string();
            if self.fit_to_height.has_value() {
                attributes.push(("fitToHeight", &fit_to_height).into());
            }
            let fit_to_width = self.fit_to_width.value_string();
            if self.fit_to_width.has_value() {
                attributes.push(("fitToWidth", &fit_to_width).into());
            }
            let horizontal_dpi = self.horizontal_dpi.value_string();
            if self.horizontal_dpi.has_value() {
                attributes.push(("horizontalDpi", &horizontal_dpi).into());
            }
            let vertical_dpi = self.vertical_dpi.value_string();
            if self.vertical_dpi.has_value() {
                attributes.push(("verticalDpi", &vertical_dpi).into());
            }
            if self.object_data.is_some() {
                attributes.push(("r:id", r_id_str.as_str()).into());
                *r_id += 1;
            }
            write_start_tag(writer, "pageSetup", attributes, true);
        }
    }
}
