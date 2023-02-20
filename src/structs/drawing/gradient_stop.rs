// a:gs
use super::RgbColorModelHex;
use super::SchemeColor;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct GradientStop {
    position: i32,
    scheme_color: Option<SchemeColor>,
    rgb_color_model_hex: Option<RgbColorModelHex>,
}
impl GradientStop {
    pub fn get_position(&self) -> &i32 {
        &self.position
    }

    pub fn set_position(&mut self, value: i32) -> &mut GradientStop {
        self.position = value;
        self
    }

    pub fn get_scheme_color(&self) -> &Option<SchemeColor> {
        &self.scheme_color
    }

    pub fn get_scheme_color_mut(&mut self) -> &mut Option<SchemeColor> {
        &mut self.scheme_color
    }

    pub fn set_scheme_color(&mut self, value: SchemeColor) -> &mut GradientStop {
        self.scheme_color = Some(value);
        self
    }

    pub fn get_rgb_color_model_hex(&self) -> &Option<RgbColorModelHex> {
        &self.rgb_color_model_hex
    }

    pub fn get_rgb_color_model_hex_mut(&mut self) -> &mut Option<RgbColorModelHex> {
        &mut self.rgb_color_model_hex
    }

    pub fn set_rgb_color_model_hex(&mut self, value: RgbColorModelHex) -> &mut GradientStop {
        self.rgb_color_model_hex = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"pos") {
            Some(v) => {
                self.set_position(v.parse::<i32>().unwrap());
            }
            None => {}
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:schemeClr" => {
                        let mut obj = SchemeColor::default();
                        obj.set_attributes(reader, e, false);
                        self.set_scheme_color(obj);
                    }
                    b"a:srgbClr" => {
                        let mut obj = RgbColorModelHex::default();
                        obj.set_attributes(reader, e, false);
                        self.set_rgb_color_model_hex(obj);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:schemeClr" => {
                        let mut obj = SchemeColor::default();
                        obj.set_attributes(reader, e, true);
                        self.set_scheme_color(obj);
                    }
                    b"a:srgbClr" => {
                        let mut obj = RgbColorModelHex::default();
                        obj.set_attributes(reader, e, true);
                        self.set_rgb_color_model_hex(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:gs" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:gs"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:gs
        write_start_tag(
            writer,
            "a:gs",
            vec![("pos", &self.position.to_string())],
            false,
        );

        // a:schemeClr
        for v in &self.scheme_color {
            v.write_to(writer);
        }

        // a:srgbClr
        for v in &self.rgb_color_model_hex {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:gs");
    }
}
