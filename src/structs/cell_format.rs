// xf
use super::Alignment;
use super::BooleanValue;
use super::UInt32Value;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct CellFormat {
    number_format_id: UInt32Value,
    font_id: UInt32Value,
    fill_id: UInt32Value,
    border_id: UInt32Value,
    format_id: UInt32Value,
    apply_number_format: BooleanValue,
    apply_fill: BooleanValue,
    apply_border: BooleanValue,
    apply_font: BooleanValue,
    apply_alignment: BooleanValue,
    apply_protection: BooleanValue,
    alignment: Option<Alignment>,
}
impl CellFormat {
    pub(crate) fn get_number_format_id(&self) -> &u32 {
        self.number_format_id.get_value()
    }

    pub(crate) fn set_number_format_id(&mut self, value: u32) -> &mut Self {
        self.number_format_id.set_value(value);
        self
    }

    pub(crate) fn get_font_id(&self) -> &u32 {
        self.font_id.get_value()
    }

    pub(crate) fn set_font_id(&mut self, value: u32) -> &mut Self {
        self.font_id.set_value(value);
        self
    }

    pub(crate) fn get_fill_id(&self) -> &u32 {
        self.fill_id.get_value()
    }

    pub(crate) fn set_fill_id(&mut self, value: u32) -> &mut Self {
        self.fill_id.set_value(value);
        self
    }

    pub(crate) fn get_border_id(&self) -> &u32 {
        self.border_id.get_value()
    }

    pub(crate) fn set_border_id(&mut self, value: u32) -> &mut Self {
        self.border_id.set_value(value);
        self
    }

    pub(crate) fn get_format_id(&self) -> &u32 {
        self.format_id.get_value()
    }

    pub(crate) fn set_format_id(&mut self, value: u32) -> &mut Self {
        self.format_id.set_value(value);
        self
    }

    pub(crate) fn get_apply_number_format(&self) -> &bool {
        self.apply_number_format.get_value()
    }

    pub(crate) fn set_apply_number_format(&mut self, value: bool) -> &mut Self {
        self.apply_number_format.set_value(value);
        self
    }

    pub(crate) fn has_apply_number_format(&self) -> bool {
        self.apply_number_format.has_value()
    }

    pub(crate) fn get_apply_fill(&self) -> &bool {
        self.apply_fill.get_value()
    }

    pub(crate) fn set_apply_fill(&mut self, value: bool) -> &mut Self {
        self.apply_fill.set_value(value);
        self
    }

    pub(crate) fn has_apply_fill(&self) -> bool {
        self.apply_fill.has_value()
    }

    pub(crate) fn get_apply_border(&self) -> &bool {
        self.apply_border.get_value()
    }

    pub(crate) fn set_apply_border(&mut self, value: bool) -> &mut Self {
        self.apply_border.set_value(value);
        self
    }

    pub(crate) fn has_apply_border(&self) -> bool {
        self.apply_border.has_value()
    }

    pub(crate) fn get_apply_font(&self) -> &bool {
        self.apply_font.get_value()
    }

    pub(crate) fn set_apply_font(&mut self, value: bool) -> &mut Self {
        self.apply_font.set_value(value);
        self
    }

    pub(crate) fn has_apply_font(&self) -> bool {
        self.apply_font.has_value()
    }

    pub(crate) fn get_apply_alignment(&self) -> &bool {
        self.apply_alignment.get_value()
    }

    pub(crate) fn set_apply_alignment(&mut self, value: bool) -> &mut Self {
        self.apply_alignment.set_value(value);
        self
    }

    pub(crate) fn has_apply_alignment(&self) -> bool {
        self.apply_alignment.has_value()
    }

    pub(crate) fn _get_apply_protection(&self) -> &bool {
        self.apply_protection.get_value()
    }

    pub(crate) fn _set_apply_protection(&mut self, value: bool) -> &mut Self {
        self.apply_protection.set_value(value);
        self
    }

    pub(crate) fn get_alignment(&self) -> &Option<Alignment> {
        &self.alignment
    }

    pub(crate) fn _get_alignment_mut(&mut self) -> &mut Option<Alignment> {
        &mut self.alignment
    }

    pub(crate) fn set_alignment(&mut self, value: Alignment) -> &mut Self {
        self.alignment = Some(value);
        self
    }

    pub(crate) fn _get_hash_code(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}{}{}{}{}{}{}{}{}{}{}",
                &self.get_number_format_id(),
                &self.get_font_id(),
                &self.get_fill_id(),
                &self.get_border_id(),
                &self.get_format_id(),
                &self.get_apply_number_format(),
                &self.get_apply_fill(),
                &self.get_apply_border(),
                &self.get_apply_font(),
                &self.get_apply_alignment(),
                &self._get_apply_protection(),
                match &self.get_alignment() {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        String::from("NONE")
                    }
                },
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        match get_attribute(e, b"numFmtId") {
            Some(v) => {
                self.number_format_id.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"fontId") {
            Some(v) => {
                self.font_id.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"fillId") {
            Some(v) => {
                self.fill_id.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"borderId") {
            Some(v) => {
                self.border_id.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"xfId") {
            Some(v) => {
                self.format_id.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"applyNumberFormat") {
            Some(v) => {
                self.apply_number_format.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"applyBorder") {
            Some(v) => {
                self.apply_border.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"applyFont") {
            Some(v) => {
                self.apply_font.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"applyFill") {
            Some(v) => {
                self.apply_fill.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"applyAlignment") {
            Some(v) => {
                self.apply_alignment.set_value_string(v);
            }
            None => {}
        }
        match get_attribute(e, b"applyProtection") {
            Some(v) => {
                self.apply_protection.set_value_string(v);
            }
            None => {}
        }

        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"alignment" => {
                        let mut obj = Alignment::default();
                        obj.set_attributes(reader, e);
                        self.set_alignment(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"xf" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "xf"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, is_cell_xfs: bool) {
        let empty_flag = self.alignment.is_none();

        // xf
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let number_format_id = self.number_format_id.get_value_string();
        attributes.push(("numFmtId", &number_format_id));
        let font_id = self.font_id.get_value_string();
        attributes.push(("fontId", &font_id));
        let fill_id = self.fill_id.get_value_string();
        attributes.push(("fillId", &fill_id));
        let border_id = self.border_id.get_value_string();
        attributes.push(("borderId", &border_id));

        let format_id = self.format_id.get_value_string();
        if is_cell_xfs {
            attributes.push(("xfId", &format_id));
        }
        if self.apply_font.has_value() {
            attributes.push(("applyFont", self.apply_font.get_value_string()));
        }
        if self.apply_number_format.has_value() {
            attributes.push((
                "applyNumberFormat",
                self.apply_number_format.get_value_string(),
            ));
        }
        if self.apply_fill.has_value() {
            attributes.push(("applyFill", self.apply_fill.get_value_string()));
        }
        if self.apply_border.has_value() {
            attributes.push(("applyBorder", self.apply_border.get_value_string()));
        }
        if self.apply_alignment.has_value() {
            attributes.push(("applyAlignment", self.apply_alignment.get_value_string()));
        }
        if self.apply_protection.has_value() {
            attributes.push(("applyProtection", self.apply_protection.get_value_string()));
        }
        write_start_tag(writer, "xf", attributes, empty_flag);

        if !empty_flag {
            // alignment
            match &self.alignment {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }
            write_end_tag(writer, "xf");
        }
    }
}
