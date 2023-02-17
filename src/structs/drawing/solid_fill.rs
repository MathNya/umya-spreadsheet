// a:solidFill
use super::rgb_color_model_hex::RgbColorModelHex;
use super::scheme_color::SchemeColor;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct SolidFill {
    scheme_color: Option<SchemeColor>,
    rgb_color_model_hex: Option<RgbColorModelHex>,
}
impl SolidFill {
    pub fn get_scheme_color(&self) -> &Option<SchemeColor> {
        &self.scheme_color
    }

    pub fn get_scheme_color_mut(&mut self) -> &mut Option<SchemeColor> {
        &mut self.scheme_color
    }

    pub fn set_scheme_color(&mut self, value: SchemeColor) {
        self.scheme_color = Some(value);
    }

    pub fn get_rgb_color_model_hex(&self) -> &Option<RgbColorModelHex> {
        &self.rgb_color_model_hex
    }

    pub fn get_rgb_color_model_hex_mut(&mut self) -> &mut Option<RgbColorModelHex> {
        &mut self.rgb_color_model_hex
    }

    pub fn set_rgb_color_model_hex(&mut self, value: RgbColorModelHex) {
        self.rgb_color_model_hex = Some(value);
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:schemeClr" => {
                        let mut scheme_color = SchemeColor::default();
                        scheme_color.set_attributes(reader, e, false);
                        self.set_scheme_color(scheme_color);
                    }
                    b"a:srgbClr" => {
                        let mut rgb_color_model_hex = RgbColorModelHex::default();
                        rgb_color_model_hex.set_attributes(reader, e, false);
                        self.set_rgb_color_model_hex(rgb_color_model_hex);
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"a:schemeClr" => {
                        let mut scheme_color = SchemeColor::default();
                        scheme_color.set_attributes(reader, e, true);
                        self.set_scheme_color(scheme_color);
                    }
                    b"a:srgbClr" => {
                        let mut rgb_color_model_hex = RgbColorModelHex::default();
                        rgb_color_model_hex.set_attributes(reader, e, true);
                        self.set_rgb_color_model_hex(rgb_color_model_hex);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"a:solidFill" => {
                        return;
                    }
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:solidFill"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:solidFill
        write_start_tag(writer, "a:solidFill", vec![], false);
        match &self.scheme_color {
            Some(color) => {
                color.write_to(writer);
            }
            None => {}
        }
        match &self.rgb_color_model_hex {
            Some(hex) => {
                hex.write_to(writer);
            }
            None => {}
        }
        write_end_tag(writer, "a:solidFill");
    }
}
