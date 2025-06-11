// a:solidFill
use super::rgb_color_model_hex::RgbColorModelHex;
use super::scheme_color::SchemeColor;
use super::SystemColor;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct SolidFill {
    scheme_color: Option<Box<SchemeColor>>,
    rgb_color_model_hex: Option<Box<RgbColorModelHex>>,
    system_color: Option<Box<SystemColor>>,
}

impl SolidFill {
    #[inline]
    pub fn get_scheme_color(&self) -> Option<&SchemeColor> {
        self.scheme_color.as_deref()
    }

    #[inline]
    pub fn get_scheme_color_mut(&mut self) -> Option<&mut SchemeColor> {
        self.scheme_color.as_deref_mut()
    }

    #[inline]
    pub fn set_scheme_color(&mut self, value: SchemeColor) {
        self.scheme_color = Some(Box::new(value));
    }

    #[inline]
    pub fn get_rgb_color_model_hex(&self) -> Option<&RgbColorModelHex> {
        self.rgb_color_model_hex.as_deref()
    }

    #[inline]
    pub fn get_rgb_color_model_hex_mut(&mut self) -> Option<&mut RgbColorModelHex> {
        self.rgb_color_model_hex.as_deref_mut()
    }

    #[inline]
    pub fn set_rgb_color_model_hex(&mut self, value: RgbColorModelHex) {
        self.rgb_color_model_hex = Some(Box::new(value));
    }

    #[inline]
    pub fn get_system_color(&self) -> Option<&SystemColor> {
        self.system_color.as_deref()
    }

    #[inline]
    pub fn get_system_color_mut(&mut self) -> Option<&mut SystemColor> {
        self.system_color.as_deref_mut()
    }

    #[inline]
    pub fn set_system_color(&mut self, value: SystemColor) {
        self.system_color = Some(Box::new(value));
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
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
                }
            },
            Event::Empty(ref e) => {
                match e.name().into_inner() {
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
                b"a:sysClr" => {
                    let mut obj = SystemColor::default();
                    obj.set_attributes(reader, e);
                    self.set_system_color(obj);
                }
                _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"a:solidFill" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "a:solidFill")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:solidFill
        write_start_tag(writer, "a:solidFill", vec![], false);
        if let Some(color) = &self.scheme_color {
            color.write_to(writer);
        }

        // a:srgbClr
        if let Some(hex) = &self.rgb_color_model_hex {
            hex.write_to(writer);
        }

        // a:sysClr
        if let Some(v) = &self.system_color {
            v.write_to(writer);
        }

        write_end_tag(writer, "a:solidFill");
    }
}
