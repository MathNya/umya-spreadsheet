// patternFill
use super::Color;
use super::EnumValue;
use super::PatternValues;
use md5::Digest;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct PatternFill {
    pub(crate) pattern_type: EnumValue<PatternValues>,
    foreground_color: Option<Color>,
    background_color: Option<Color>,
}

impl PatternFill {
    pub fn get_pattern_type(&self) -> &PatternValues {
        self.pattern_type.get_value()
    }

    pub fn set_pattern_type(&mut self, value: PatternValues) -> &mut Self {
        self.pattern_type.set_value(value);
        self
    }

    fn auto_set_pattern_type(&mut self) -> &mut Self {
        if self.get_pattern_type() == &PatternValues::None {
            if self.get_foreground_color().is_some() {
                self.set_pattern_type(PatternValues::Solid);
            }
        } else if self.get_foreground_color().is_none() {
            self.set_pattern_type(PatternValues::None);
        }
        self
    }

    pub fn get_foreground_color(&self) -> Option<&Color> {
        self.foreground_color.as_ref()
    }

    pub fn get_foreground_color_mut(&mut self) -> &mut Color {
        self.foreground_color.get_or_insert(Color::default())
    }

    pub fn set_foreground_color(&mut self, value: Color) -> &mut Self {
        self.foreground_color = Some(value);
        self.auto_set_pattern_type();
        self
    }

    pub fn remove_foreground_color(&mut self) -> &mut Self {
        self.foreground_color = None;
        self
    }

    pub fn get_background_color(&self) -> Option<&Color> {
        self.background_color.as_ref()
    }

    pub fn get_background_color_mut(&mut self) -> &mut Color {
        self.background_color.get_or_insert(Color::default())
    }

    pub fn set_background_color(&mut self, value: Color) -> &mut Self {
        self.background_color = Some(value);
        self
    }

    pub fn remove_background_color(&mut self) -> &mut Self {
        self.background_color = None;
        self
    }

    pub(crate) fn get_hash_code(&self) -> String {
        let pattern_type = self.pattern_type.get_value_string();
        let foreground_color = self
            .foreground_color
            .as_ref()
            .map_or("None".into(), |v| v.get_hash_code());
        let background_color = self
            .background_color
            .as_ref()
            .map_or("None".into(), |v| v.get_hash_code());
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}{}",
                pattern_type, foreground_color, background_color
            ))
        )
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, pattern_type, "patternType");

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"fgColor" => {
                        let mut obj = Color::default();
                        obj.set_attributes(reader, e, true);
                        self.set_foreground_color(obj);
                    }
                    b"bgColor" => {
                        let mut obj = Color::default();
                        obj.set_attributes(reader, e, true);
                        self.set_background_color(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"patternFill" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "patternFill")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let empty_flag = self.foreground_color.is_none() && self.background_color.is_none();

        // patternFill
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.pattern_type.has_value() {
            attributes.push(("patternType", self.pattern_type.get_value_string()));
        }
        write_start_tag(writer, "patternFill", attributes, empty_flag);

        if !empty_flag {
            // fgColor
            if let Some(v) = &self.foreground_color {
                v.write_to_fg_color(writer);
            }

            // bgColor
            if let Some(v) = &self.background_color {
                v.write_to_bg_color(writer);
            }

            write_end_tag(writer, "patternFill");
        }
    }
}
