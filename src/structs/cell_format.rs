// xf
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
    Alignment,
    BooleanValue,
    Protection,
    UInt32Value,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub(crate) struct CellFormat {
    number_format_id:    UInt32Value,
    font_id:             UInt32Value,
    fill_id:             UInt32Value,
    border_id:           UInt32Value,
    format_id:           UInt32Value,
    apply_number_format: BooleanValue,
    apply_fill:          BooleanValue,
    apply_border:        BooleanValue,
    apply_font:          BooleanValue,
    apply_alignment:     BooleanValue,
    apply_protection:    BooleanValue,
    alignment:           Option<Alignment>,
    protection:          Option<Protection>,
}

impl CellFormat {
    #[inline]
    pub(crate) fn number_format_id(&self) -> u32 {
        self.number_format_id.get_value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use number_format_id()")]
    pub(crate) fn get_number_format_id(&self) -> u32 {
        self.number_format_id()
    }

    #[inline]
    pub(crate) fn set_number_format_id(&mut self, value: u32) -> &mut Self {
        self.number_format_id.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn font_id(&self) -> u32 {
        self.font_id.get_value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use number_format_id()")]
    pub(crate) fn get_font_id(&self) -> u32 {
        self.font_id()
    }

    #[inline]
    pub(crate) fn set_font_id(&mut self, value: u32) -> &mut Self {
        self.font_id.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn fill_id(&self) -> u32 {
        self.fill_id.get_value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use fill_id()")]
    pub(crate) fn get_fill_id(&self) -> u32 {
        self.fill_id()
    }

    #[inline]
    pub(crate) fn set_fill_id(&mut self, value: u32) -> &mut Self {
        self.fill_id.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn border_id(&self) -> u32 {
        self.border_id.get_value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use border_id()")]
    pub(crate) fn get_border_id(&self) -> u32 {
        self.border_id()
    }

    #[inline]
    pub(crate) fn set_border_id(&mut self, value: u32) -> &mut Self {
        self.border_id.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn format_id(&self) -> u32 {
        self.format_id.get_value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use format_id()")]
    pub(crate) fn get_format_id(&self) -> u32 {
        self.format_id()
    }

    #[inline]
    pub(crate) fn set_format_id(&mut self, value: u32) -> &mut Self {
        self.format_id.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn apply_number_format(&self) -> bool {
        self.apply_number_format.get_value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use apply_number_format()")]
    pub(crate) fn get_apply_number_format(&self) -> bool {
        self.apply_number_format()
    }

    #[inline]
    pub(crate) fn set_apply_number_format(&mut self, value: bool) -> &mut Self {
        self.apply_number_format.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn has_apply_number_format(&self) -> bool {
        self.apply_number_format.has_value()
    }

    #[inline]
    pub(crate) fn apply_fill(&self) -> bool {
        self.apply_fill.get_value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use apply_fill()")]
    pub(crate) fn get_apply_fill(&self) -> bool {
        self.apply_fill()
    }

    #[inline]
    pub(crate) fn set_apply_fill(&mut self, value: bool) -> &mut Self {
        self.apply_fill.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn has_apply_fill(&self) -> bool {
        self.apply_fill.has_value()
    }

    #[inline]
    pub(crate) fn apply_border(&self) -> bool {
        self.apply_border.get_value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use apply_border()")]
    pub(crate) fn get_apply_border(&self) -> bool {
        self.apply_border()
    }

    #[inline]
    pub(crate) fn set_apply_border(&mut self, value: bool) -> &mut Self {
        self.apply_border.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn has_apply_border(&self) -> bool {
        self.apply_border.has_value()
    }

    #[inline]
    pub(crate) fn apply_font(&self) -> bool {
        self.apply_font.get_value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use apply_font()")]
    pub(crate) fn get_apply_font(&self) -> bool {
        self.apply_font()
    }

    #[inline]
    pub(crate) fn set_apply_font(&mut self, value: bool) -> &mut Self {
        self.apply_font.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn has_apply_font(&self) -> bool {
        self.apply_font.has_value()
    }

    #[inline]
    pub(crate) fn apply_alignment(&self) -> bool {
        self.apply_alignment.get_value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use apply_alignment()")]
    pub(crate) fn get_apply_alignment(&self) -> bool {
        self.apply_alignment()
    }

    #[inline]
    pub(crate) fn set_apply_alignment(&mut self, value: bool) -> &mut Self {
        self.apply_alignment.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn has_apply_alignment(&self) -> bool {
        self.apply_alignment.has_value()
    }

    #[inline]
    pub(crate) fn apply_protection(&self) -> bool {
        self.apply_protection.get_value()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use apply_protection()")]
    pub(crate) fn get_apply_protection(&self) -> bool {
        self.apply_protection()
    }

    #[inline]
    pub(crate) fn set_apply_protection(&mut self, value: bool) -> &mut Self {
        self.apply_protection.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn has_apply_protection(&self) -> bool {
        self.apply_protection.has_value()
    }

    #[inline]
    pub(crate) fn alignment(&self) -> Option<&Alignment> {
        self.alignment.as_ref()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use alignment()")]
    pub(crate) fn get_alignment(&self) -> Option<&Alignment> {
        self.alignment()
    }

    #[inline]
    pub(crate) fn alignment_mut(&mut self) -> Option<&mut Alignment> {
        self.alignment.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use alignment_mut()")]
    pub(crate) fn get_alignment_mut(&mut self) -> Option<&mut Alignment> {
        self.alignment_mut()
    }

    #[inline]
    pub(crate) fn set_alignment(&mut self, value: Alignment) -> &mut Self {
        self.alignment = Some(value);
        self
    }

    #[inline]
    pub(crate) fn protection(&self) -> Option<&Protection> {
        self.protection.as_ref()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use protection()")]
    pub(crate) fn get_protection(&self) -> Option<&Protection> {
        self.protection()
    }

    #[inline]
    pub(crate) fn protection_mut(&mut self) -> Option<&mut Protection> {
        self.protection.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use protection_mut()")]
    pub(crate) fn get_protection_mut(&mut self) -> Option<&mut Protection> {
        self.protection_mut()
    }

    #[inline]
    pub(crate) fn set_protection(&mut self, value: Protection) -> &mut Self {
        self.protection = Some(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, number_format_id, "numFmtId");
        set_string_from_xml!(self, e, font_id, "fontId");
        set_string_from_xml!(self, e, fill_id, "fillId");
        set_string_from_xml!(self, e, border_id, "borderId");
        set_string_from_xml!(self, e, apply_number_format, "applyNumberFormat");
        set_string_from_xml!(self, e, apply_border, "applyBorder");
        set_string_from_xml!(self, e, apply_font, "applyFont");
        set_string_from_xml!(self, e, apply_fill, "applyFill");
        set_string_from_xml!(self, e, apply_alignment, "applyAlignment");
        set_string_from_xml!(self, e, apply_protection, "applyProtection");

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner(){
                    b"alignment" =>{
                        let mut obj = Alignment::default();
                        obj.set_attributes(reader, e);
                        self.set_alignment(obj);
                    },
                    b"protection" =>{
                        let mut obj = Protection::default();
                        obj.set_attributes(reader, e);
                        self.set_protection(obj);
                    },
                    _ =>{}
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"xf" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "xf")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, is_cell_xfs: bool) {
        let empty_flag = self.alignment.is_none() && self.protection.is_none();

        // xf
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let number_format_id = self.number_format_id.get_value_string();
        attributes.push(("numFmtId", &number_format_id).into());
        let font_id = self.font_id.get_value_string();
        attributes.push(("fontId", &font_id).into());
        let fill_id = self.fill_id.get_value_string();
        attributes.push(("fillId", &fill_id).into());
        let border_id = self.border_id.get_value_string();
        attributes.push(("borderId", &border_id).into());

        let format_id = self.format_id.get_value_string();
        if is_cell_xfs {
            attributes.push(("xfId", &format_id).into());
        }
        if self.apply_font.has_value() {
            attributes.push(("applyFont", self.apply_font.get_value_string()).into());
        }
        if self.apply_number_format.has_value() {
            attributes.push(
                (
                    "applyNumberFormat",
                    self.apply_number_format.get_value_string(),
                )
                    .into(),
            );
        }
        if self.apply_fill.has_value() {
            attributes.push(("applyFill", self.apply_fill.get_value_string()).into());
        }
        if self.apply_border.has_value() {
            attributes.push(("applyBorder", self.apply_border.get_value_string()).into());
        }
        if self.apply_alignment.has_value() {
            attributes.push(("applyAlignment", self.apply_alignment.get_value_string()).into());
        }
        if self.apply_protection.has_value() {
            attributes.push(("applyProtection", self.apply_protection.get_value_string()).into());
        }
        write_start_tag(writer, "xf", attributes, empty_flag);

        if !empty_flag {
            // alignment
            if let Some(v) = &self.alignment {
                v.write_to(writer);
            }
            // protection
            if let Some(v) = &self.protection {
                v.write_to(writer);
            }
            write_end_tag(writer, "xf");
        }
    }
}
