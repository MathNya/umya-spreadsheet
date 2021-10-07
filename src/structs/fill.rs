use super::PatternFill;
use super::Color;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug, Clone)]
pub struct Fill {
    pattern_fill: PatternFill,
}
impl Fill {
    // Fill types
    pub const FILL_NONE: &'static str = "none";
    pub const FILL_SOLID: &'static str = "solid";
    pub const FILL_GRADIENT_LINEAR: &'static str = "linear";
    pub const FILL_GRADIENT_PATH: &'static str = "path";
    pub const FILL_PATTERN_DARKDOWN: &'static str = "darkDown";
    pub const FILL_PATTERN_DARKGRAY: &'static str = "darkGray";
    pub const FILL_PATTERN_DARKGRID: &'static str = "darkGrid";
    pub const FILL_PATTERN_DARKHORIZONTAL: &'static str = "darkHorizontal";
    pub const FILL_PATTERN_DARKTRELLIS: &'static str = "darkTrellis";
    pub const FILL_PATTERN_DARKUP: &'static str = "darkUp";
    pub const FILL_PATTERN_DARKVERTICAL: &'static str = "darkVertical";
    pub const FILL_PATTERN_GRAY0625: &'static str = "gray0625";
    pub const FILL_PATTERN_GRAY125: &'static str = "gray125";
    pub const FILL_PATTERN_LIGHTDOWN: &'static str = "lightDown";
    pub const FILL_PATTERN_LIGHTGRAY: &'static str = "lightGray";
    pub const FILL_PATTERN_LIGHTGRID: &'static str = "lightGrid";
    pub const FILL_PATTERN_LIGHTHORIZONTAL: &'static str = "lightHorizontal";
    pub const FILL_PATTERN_LIGHTTRELLIS: &'static str = "lightTrellis";
    pub const FILL_PATTERN_LIGHTUP: &'static str = "lightUp";
    pub const FILL_PATTERN_LIGHTVERTICAL: &'static str = "lightVertical";
    pub const FILL_PATTERN_MEDIUMGRAY: &'static str = "mediumGray";

    pub fn get_pattern_fill(&self)-> &PatternFill {
        &self.pattern_fill
    }

    pub fn get_pattern_fill_mut(&mut self)-> &mut PatternFill {
        &mut self.pattern_fill
    }

    pub fn set_pattern_fill(&mut self, value:PatternFill)-> &mut Self {
        self.pattern_fill = value;
        self
    }

    pub fn get_fill_type(&self)-> &str {
        &self.pattern_fill.pattern_type.get_value_string()
    }

    pub fn set_fill_type(&mut self, value:String)-> &mut Self {
        self.pattern_fill.pattern_type.set_value_string(value);
        self
    }

    pub fn get_start_color(&self)-> &Option<Color> {
        &self.pattern_fill.get_foreground_color()
    }

    pub fn get_start_color_mut(&mut self)-> &mut Color {
        match self.pattern_fill.get_foreground_color() {
            Some(_) => {},
            None => {self.set_start_color(Color::default());}
        }
        self.pattern_fill.get_foreground_color_mut().as_mut().unwrap()
    }

    pub fn set_start_color(&mut self, value:Color)-> &mut Self {
        self.pattern_fill.set_foreground_color(value);
        self
    }

    pub fn get_end_color(&self)-> &Option<Color> {
        &self.pattern_fill.get_background_color()
    }

    pub fn get_end_color_mut(&mut self)-> &mut Color {
        match self.pattern_fill.get_background_color() {
            Some(_) => {},
            None => {self.set_end_color(Color::default());}
        }
        self.pattern_fill.get_background_color_mut().as_mut().unwrap()
    }

    pub fn set_end_color(&mut self, value:Color)-> &mut Self {
        self.pattern_fill.set_background_color(value);
        self
    }

    pub(crate) fn get_defalut_value()-> Self {
        let mut def = Self::default();
        def.set_fill_type(String::from(Self::FILL_NONE));
        def
    }

    pub(crate) fn get_defalut_value_2()-> Self {
        let mut def = Self::default();
        def.set_fill_type(String::from(Self::FILL_PATTERN_GRAY125));
        def
    }

    pub(crate) fn get_hash_code(&self)-> String {
        format!("{:x}", md5::compute(format!("{}",
            &self.pattern_fill.get_hash_code(),
        )))
    }

    pub(crate) fn set_attributes(
        &mut self,
        reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        _e:&BytesStart
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"patternFill" => {
                            &mut self.pattern_fill.set_attributes(reader, e, true);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        b"patternFill" => {
                            &mut self.pattern_fill.set_attributes(reader, e, false);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"fill" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "fill"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // fill
        write_start_tag(writer, "fill", vec![], false);

        // patternFill
        &self.pattern_fill.write_to(writer);

        write_end_tag(writer, "fill");
    }
}
