// a:lvl1pPr
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
    super::{
        BooleanValue,
        EnumValue,
    },
    RunProperties,
    SpaceAfter,
    SpaceBefore,
    TextAlignmentTypeValues,
    TextFontAlignmentValues,
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
pub struct TextParagraphPropertiesType {
    right_to_left:          BooleanValue,
    alignment:              EnumValue<TextAlignmentTypeValues>,
    font_alignment:         EnumValue<TextFontAlignmentValues>,
    space_before:           Option<SpaceBefore>,
    space_after:            Option<SpaceAfter>,
    default_run_properties: Option<Box<RunProperties>>,
}
impl TextParagraphPropertiesType {
    #[inline]
    #[must_use]
    pub fn get_right_to_left(&self) -> bool {
        self.right_to_left.value()
    }

    #[inline]
    pub fn set_right_to_left(&mut self, value: bool) -> &mut Self {
        self.right_to_left.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_alignment(&self) -> &TextAlignmentTypeValues {
        self.alignment.value()
    }

    #[inline]
    pub fn set_alignment(&mut self, value: TextAlignmentTypeValues) -> &mut Self {
        self.alignment.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_font_alignment(&self) -> &TextFontAlignmentValues {
        self.font_alignment.value()
    }

    #[inline]
    pub fn set_font_alignment(&mut self, value: TextFontAlignmentValues) -> &mut Self {
        self.font_alignment.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_space_before(&self) -> Option<&SpaceBefore> {
        self.space_before.as_ref()
    }

    #[inline]
    pub fn get_space_before_mut(&mut self) -> Option<&mut SpaceBefore> {
        self.space_before.as_mut()
    }

    #[inline]
    pub fn set_space_before(&mut self, value: SpaceBefore) -> &mut Self {
        self.space_before = Some(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_space_after(&self) -> Option<&SpaceAfter> {
        self.space_after.as_ref()
    }

    #[inline]
    pub fn get_space_after_mut(&mut self) -> Option<&mut SpaceAfter> {
        self.space_after.as_mut()
    }

    #[inline]
    pub fn set_space_after(&mut self, value: SpaceAfter) -> &mut Self {
        self.space_after = Some(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn get_default_run_properties(&self) -> Option<&RunProperties> {
        self.default_run_properties.as_deref()
    }

    #[inline]
    pub fn get_default_run_properties_mut(&mut self) -> Option<&mut RunProperties> {
        self.default_run_properties.as_deref_mut()
    }

    #[inline]
    pub fn set_default_run_properties(&mut self, value: RunProperties) -> &mut Self {
        self.default_run_properties = Some(Box::new(value));
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, right_to_left, "rtl");
        set_string_from_xml!(self, e, alignment, "algn");
        set_string_from_xml!(self, e, font_alignment, "fontAlgn");

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"a:defRPr" {
                    let mut obj = RunProperties::default();
                    obj.set_attributes(reader, e, true);
                    self.set_default_run_properties(obj);
                }
            },
            Event::Start(ref e) => {
                match e.name().into_inner() {
                b"a:spcBef" => {
                    let mut obj = SpaceBefore::default();
                    obj.set_attributes(reader, e);
                    self.set_space_before(obj);
                }
                b"a:spcAft" => {
                    let mut obj = SpaceAfter::default();
                    obj.set_attributes(reader, e);
                    self.set_space_after(obj);
                }
                b"a:defRPr" => {
                    let mut obj = RunProperties::default();
                    obj.set_attributes(reader, e, false);
                    self.set_default_run_properties(obj);
                }
                _ => (),
                }
            },
            Event::End(ref e) => {
                match e.name().into_inner() {
                    b"a:defPPr"  |
                    b"a:lvl1pPr" |
                    b"a:lvl2pPr" |
                    b"a:lvl3pPr" |
                    b"a:lvl4pPr" |
                    b"a:lvl5pPr" |
                    b"a:lvl6pPr" |
                    b"a:lvl7pPr" |
                    b"a:lvl8pPr" |
                    b"a:lvl9pPr" => return,
                    _ =>()
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:lvl1pPr-lvl9pPr")
        );
    }

    #[inline]
    pub(crate) fn write_to_default(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:defPPr");
    }

    #[inline]
    pub(crate) fn write_to_lvl1(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl1pPr");
    }

    #[inline]
    pub(crate) fn write_to_lvl2(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl2pPr");
    }

    #[inline]
    pub(crate) fn write_to_lvl3(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl3pPr");
    }

    #[inline]
    pub(crate) fn write_to_lvl4(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl4pPr");
    }

    #[inline]
    pub(crate) fn write_to_lvl5(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl5pPr");
    }

    #[inline]
    pub(crate) fn write_to_lvl6(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl6pPr");
    }

    #[inline]
    pub(crate) fn write_to_lvl7(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl7pPr");
    }

    #[inline]
    pub(crate) fn write_to_lvl8(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl8pPr");
    }

    #[inline]
    pub(crate) fn write_to_lvl9(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl9pPr");
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        // a:lvl1pPr
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.right_to_left.has_value() {
            attributes.push(("rtl", self.right_to_left.value_string()).into());
        }
        if self.alignment.has_value() {
            attributes.push(("algn", self.alignment.value_string()).into());
        }
        if self.font_alignment.has_value() {
            attributes.push(("fontAlgn", self.font_alignment.value_string()).into());
        }
        write_start_tag(writer, tag_name, attributes, false);

        // a:spcBef
        if let Some(v) = &self.space_before {
            v.write_to(writer);
        }

        // a:spcAft
        if let Some(v) = &self.space_after {
            v.write_to(writer);
        }

        // a:defRPr
        if let Some(v) = &self.default_run_properties {
            v.write_to_def_rpr(writer);
        }

        write_end_tag(writer, tag_name);
    }
}
