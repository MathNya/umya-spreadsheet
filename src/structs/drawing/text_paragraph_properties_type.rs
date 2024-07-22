// a:lvl1pPr
use super::super::BooleanValue;
use super::super::EnumValue;
use super::RunProperties;
use super::SpaceAfter;
use super::SpaceBefore;
use super::TextAlignmentTypeValues;
use super::TextFontAlignmentValues;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct TextParagraphPropertiesType {
    right_to_left: BooleanValue,
    alignment: EnumValue<TextAlignmentTypeValues>,
    font_alignment: EnumValue<TextFontAlignmentValues>,
    space_before: Option<SpaceBefore>,
    space_after: Option<SpaceAfter>,
    default_run_properties: Option<RunProperties>,
}
impl TextParagraphPropertiesType {
    pub fn get_right_to_left(&self) -> &bool {
        self.right_to_left.get_value()
    }

    pub fn set_right_to_left(&mut self, value: bool) -> &mut Self {
        self.right_to_left.set_value(value);
        self
    }

    pub fn get_alignment(&self) -> &TextAlignmentTypeValues {
        self.alignment.get_value()
    }

    pub fn set_alignment(&mut self, value: TextAlignmentTypeValues) -> &mut Self {
        self.alignment.set_value(value);
        self
    }

    pub fn get_font_alignment(&self) -> &TextFontAlignmentValues {
        self.font_alignment.get_value()
    }

    pub fn set_font_alignment(&mut self, value: TextFontAlignmentValues) -> &mut Self {
        self.font_alignment.set_value(value);
        self
    }

    pub fn get_space_before(&self) -> Option<&SpaceBefore> {
        self.space_before.as_ref()
    }

    pub fn get_space_before_mut(&mut self) -> Option<&mut SpaceBefore> {
        self.space_before.as_mut()
    }

    pub fn set_space_before(&mut self, value: SpaceBefore) -> &mut Self {
        self.space_before = Some(value);
        self
    }

    pub fn get_space_after(&self) -> Option<&SpaceAfter> {
        self.space_after.as_ref()
    }

    pub fn get_space_after_mut(&mut self) -> Option<&mut SpaceAfter> {
        self.space_after.as_mut()
    }

    pub fn set_space_after(&mut self, value: SpaceAfter) -> &mut Self {
        self.space_after = Some(value);
        self
    }

    pub fn get_default_run_properties(&self) -> Option<&RunProperties> {
        self.default_run_properties.as_ref()
    }

    pub fn get_default_run_properties_mut(&mut self) -> Option<&mut RunProperties> {
        self.default_run_properties.as_mut()
    }

    pub fn set_default_run_properties(&mut self, value: RunProperties) -> &mut Self {
        self.default_run_properties = Some(value);
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
                match e.name().into_inner() {
                    b"a:defRPr" => {
                        let mut obj = RunProperties::default();
                        obj.set_attributes(reader, e, true);
                        self.set_default_run_properties(obj);
                    }
                    _ => (),
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
                    b"a:defPPr"  => return,
                    b"a:lvl1pPr" => return,
                    b"a:lvl2pPr" => return,
                    b"a:lvl3pPr" => return,
                    b"a:lvl4pPr" => return,
                    b"a:lvl5pPr" => return,
                    b"a:lvl6pPr" => return,
                    b"a:lvl7pPr" => return,
                    b"a:lvl8pPr" => return,
                    b"a:lvl9pPr" => return,
                    _ =>()
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:lvl1pPr-lvl9pPr")
        );
    }

    pub(crate) fn write_to_default(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:defPPr")
    }

    pub(crate) fn write_to_lvl1(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl1pPr")
    }

    pub(crate) fn write_to_lvl2(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl2pPr")
    }

    pub(crate) fn write_to_lvl3(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl3pPr")
    }

    pub(crate) fn write_to_lvl4(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl4pPr")
    }

    pub(crate) fn write_to_lvl5(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl5pPr")
    }

    pub(crate) fn write_to_lvl6(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl6pPr")
    }

    pub(crate) fn write_to_lvl7(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl7pPr")
    }

    pub(crate) fn write_to_lvl8(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl8pPr")
    }

    pub(crate) fn write_to_lvl9(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        self.write_to(writer, "a:lvl9pPr")
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        // a:lvl1pPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.right_to_left.has_value() {
            attributes.push(("rtl", self.right_to_left.get_value_string()));
        }
        if self.alignment.has_value() {
            attributes.push(("algn", self.alignment.get_value_string()));
        }
        if self.font_alignment.has_value() {
            attributes.push(("fontAlgn", self.font_alignment.get_value_string()));
        }
        write_start_tag(writer, tag_name, attributes, false);

        // a:spcBef
        match &self.space_before {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:spcAft
        match &self.space_after {
            Some(v) => {
                v.write_to(writer);
            }
            None => {}
        }

        // a:defRPr
        match &self.default_run_properties {
            Some(v) => {
                v.write_to_def_rpr(writer);
            }
            None => {}
        }

        write_end_tag(writer, tag_name);
    }
}
