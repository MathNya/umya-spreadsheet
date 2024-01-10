use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use structs::raw::RawRelationships;
use structs::EnumValue;
use structs::OrientationValues;
use structs::UInt32Value;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct PageSetup {
    paper_size: UInt32Value,
    orientation: EnumValue<OrientationValues>,
    scale: UInt32Value,
    fit_to_height: UInt32Value,
    fit_to_width: UInt32Value,
    horizontal_dpi: UInt32Value,
    vertical_dpi: UInt32Value,
    object_data: Option<Vec<u8>>,
}

impl PageSetup {
    pub fn get_paper_size(&self) -> &u32 {
        self.paper_size.get_value()
    }

    pub fn set_paper_size(&mut self, value: u32) -> &mut Self {
        self.paper_size.set_value(value);
        self
    }

    pub fn get_orientation(&self) -> &OrientationValues {
        self.orientation.get_value()
    }

    pub fn set_orientation(&mut self, value: OrientationValues) -> &mut Self {
        self.orientation.set_value(value);
        self
    }

    pub fn get_scale(&self) -> &u32 {
        self.scale.get_value()
    }

    pub fn set_scale(&mut self, value: u32) -> &mut Self {
        self.scale.set_value(value);
        self
    }

    pub fn get_fit_to_height(&self) -> &u32 {
        self.fit_to_height.get_value()
    }

    pub fn set_fit_to_height(&mut self, value: u32) -> &mut Self {
        self.fit_to_height.set_value(value);
        self
    }

    pub fn get_fit_to_width(&self) -> &u32 {
        self.fit_to_width.get_value()
    }

    pub fn set_fit_to_width(&mut self, value: u32) -> &mut Self {
        self.fit_to_width.set_value(value);
        self
    }

    pub fn get_horizontal_dpi(&self) -> &u32 {
        self.horizontal_dpi.get_value()
    }

    pub fn set_horizontal_dpi(&mut self, value: u32) -> &mut Self {
        self.horizontal_dpi.set_value(value);
        self
    }

    pub fn get_vertical_dpi(&self) -> &u32 {
        self.vertical_dpi.get_value()
    }

    pub fn set_vertical_dpi(&mut self, value: u32) -> &mut Self {
        self.vertical_dpi.set_value(value);
        self
    }

    pub fn get_object_data(&self) -> Option<&Vec<u8>> {
        self.object_data.as_ref()
    }

    pub fn get_object_data_mut(&mut self) -> Option<&mut Vec<u8>> {
        self.object_data.as_mut()
    }

    pub fn set_object_data(&mut self, value: Vec<u8>) -> &mut Self {
        self.object_data = Some(value);
        self
    }

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
            self.set_object_data(attached_file.get_file_data().clone());
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, r_id: &mut usize) {
        if self.has_param() {
            // pageSetup
            let r_id_str = format!("rId{}", r_id);
            let mut attributes: Vec<(&str, &str)> = Vec::new();
            let paper_size = self.paper_size.get_value_string();
            if self.paper_size.has_value() {
                attributes.push(("paperSize", &paper_size));
            }
            let scale = self.scale.get_value_string();
            if self.scale.has_value() {
                attributes.push(("scale", &scale));
            }
            let orientation = self.orientation.get_value_string();
            if self.orientation.has_value() {
                attributes.push(("orientation", orientation));
            }
            let fit_to_height = self.fit_to_height.get_value_string();
            if self.fit_to_height.has_value() {
                attributes.push(("fitToHeight", &fit_to_height));
            }
            let fit_to_width = self.fit_to_width.get_value_string();
            if self.fit_to_width.has_value() {
                attributes.push(("fitToWidth", &fit_to_width));
            }
            let horizontal_dpi = self.horizontal_dpi.get_value_string();
            if self.horizontal_dpi.has_value() {
                attributes.push(("horizontalDpi", &horizontal_dpi));
            }
            let vertical_dpi = self.vertical_dpi.get_value_string();
            if self.vertical_dpi.has_value() {
                attributes.push(("verticalDpi", &vertical_dpi));
            }
            if self.object_data.is_some() {
                attributes.push(("r:id", r_id_str.as_str()));
                *r_id += 1;
            }
            write_start_tag(writer, "pageSetup", attributes, true);
        }
    }
}
