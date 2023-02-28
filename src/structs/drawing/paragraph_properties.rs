// a:pPr
use super::super::EnumValue;
use super::LineSpacing;
use super::RunProperties;
use super::TextAlignmentTypeValues;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ParagraphProperties {
    right_to_left: Option<String>,
    alignment: EnumValue<TextAlignmentTypeValues>,
    default_run_properties: Option<RunProperties>,
    line_spacing: Option<LineSpacing>,
}
impl ParagraphProperties {
    pub fn get_right_to_left(&self) -> &Option<String> {
        &self.right_to_left
    }

    pub fn set_right_to_left<S: Into<String>>(&mut self, value: S) -> &mut ParagraphProperties {
        self.right_to_left = Some(value.into());
        self
    }

    pub fn get_alignment(&self) -> &TextAlignmentTypeValues {
        self.alignment.get_value()
    }

    pub fn set_alignment(&mut self, value: TextAlignmentTypeValues) -> &mut ParagraphProperties {
        self.alignment.set_value(value);
        self
    }

    pub fn get_default_run_properties(&self) -> &Option<RunProperties> {
        &self.default_run_properties
    }

    pub fn get_default_run_properties_mut(&mut self) -> &mut Option<RunProperties> {
        &mut self.default_run_properties
    }

    pub fn set_default_run_properties(&mut self, value: RunProperties) -> &mut ParagraphProperties {
        self.default_run_properties = Some(value);
        self
    }

    pub fn get_line_spacing(&self) -> &Option<LineSpacing> {
        &self.line_spacing
    }

    pub fn get_line_spacing_mut(&mut self) -> &mut Option<LineSpacing> {
        &mut self.line_spacing
    }

    pub fn set_line_spacing(&mut self, value: LineSpacing) -> &mut Self {
        self.line_spacing = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        match get_attribute(e, b"rtl") {
            Some(v) => {
                self.set_right_to_left(v);
            }
            None => {}
        }
        match get_attribute(e, b"algn") {
            Some(v) => {
                self.alignment.set_value_string(v);
            }
            None => {}
        }

        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:defRPr" => {
                        let mut obj = RunProperties::default();
                        obj.set_attributes(reader, e, false);
                        self.set_default_run_properties(obj);
                    }
                    b"a:lnSpc" => {
                        let mut obj = LineSpacing::default();
                        obj.set_attributes(reader, e);
                        self.set_line_spacing(obj);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:defRPr" => {
                        let mut obj = RunProperties::default();
                        obj.set_attributes(reader, e, true);
                        self.set_default_run_properties(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:pPr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:pPr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let empty_flag = self.default_run_properties.is_none() && self.line_spacing.is_none();

        // a:pPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match &self.right_to_left {
            Some(v) => {
                attributes.push(("rtl", v));
            }
            None => {}
        }
        if self.alignment.has_value() {
            attributes.push(("algn", self.alignment.get_value_string()));
        }
        write_start_tag(writer, "a:pPr", attributes, empty_flag);

        if !empty_flag {
            // a:defRPr
            match &self.default_run_properties {
                Some(v) => v.write_to_def_rpr(writer),
                None => {}
            }

            // a:lnSpc
            match &self.line_spacing {
                Some(v) => v.write_to(writer),
                None => {}
            }

            write_end_tag(writer, "a:pPr");
        }
    }
}
