use super::RgbColorModelHex;
use super::SystemColor;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Color2Type {
    rgb_color_model_hex: Option<RgbColorModelHex>,
    system_color: Option<SystemColor>,
}

impl Color2Type {
    pub fn set_rgb_color_model_hex(&mut self, value: RgbColorModelHex) {
        self.rgb_color_model_hex = Some(value);
    }

    pub fn get_rgb_color_model_hex(&self) -> Option<&RgbColorModelHex> {
        self.rgb_color_model_hex.as_ref()
    }

    pub fn get_rgb_color_model_hex_mut(&mut self) -> Option<&mut RgbColorModelHex> {
        self.rgb_color_model_hex.as_mut()
    }

    pub fn set_system_color(&mut self, value: SystemColor) {
        self.system_color = Some(value);
    }

    pub fn get_system_color(&self) -> Option<&SystemColor> {
        self.system_color.as_ref()
    }

    pub fn get_system_color_mut(&mut self) -> Option<&mut SystemColor> {
        self.system_color.as_mut()
    }

    pub fn get_val(&self) -> String {
        if let Some(v) = &self.rgb_color_model_hex {
            return v.get_val().to_string();
        }
        if let Some(v) = &self.system_color {
            return v.get_last_color().to_string();
        }
        String::from("")
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                b"a:srgbClr" => {
                    let mut obj = RgbColorModelHex::default();
                    obj.set_attributes(reader, e, true);
                    self.rgb_color_model_hex = Some(obj);
                }
                b"a:sysClr" => {
                    let mut obj = SystemColor::default();
                    obj.set_attributes(reader, e);
                    self.system_color = Some(obj);
                }
                _ => (),
                }
            },
            Event::Start(ref e) => {
                match e.name().into_inner() {
                b"a:srgbClr" => {
                    let mut obj = RgbColorModelHex::default();
                    obj.set_attributes(reader, e, false);
                    self.rgb_color_model_hex = Some(obj);
                }
                b"a:sysClr" => {
                    let mut obj = SystemColor::default();
                    obj.set_attributes(reader, e);
                    self.system_color = Some(obj);
                }
                _ => (),
                }
            },
            Event::End(ref e) => {
                match e.name().into_inner() {
                b"a:accent1" => return,
                b"a:accent2" => return,
                b"a:accent3" => return,
                b"a:accent4" => return,
                b"a:accent5" => return,
                b"a:accent6" => return,
                b"a:dk1" => return,
                b"a:dk2" => return,
                b"a:folHlink" => return,
                b"a:hlink" => return,
                b"a:lt1" => return,
                b"a:lt2" => return,
                _ => (),
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "Color2Type")
        );
    }

    pub(crate) fn write_to_accent1(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:accent1
        self.write_to(writer, "a:accent1");
    }

    pub(crate) fn write_to_accent2(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:accent2
        self.write_to(writer, "a:accent2");
    }

    pub(crate) fn write_to_accent3(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:accent3
        self.write_to(writer, "a:accent3");
    }

    pub(crate) fn write_to_accent4(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:accent4
        self.write_to(writer, "a:accent4");
    }

    pub(crate) fn write_to_accent5(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:accent5
        self.write_to(writer, "a:accent5");
    }

    pub(crate) fn write_to_accent6(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:accent6
        self.write_to(writer, "a:accent6");
    }

    pub(crate) fn write_to_dk1(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:dk1
        self.write_to(writer, "a:dk1");
    }

    pub(crate) fn write_to_dk2(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:dk2
        self.write_to(writer, "a:dk2");
    }

    pub(crate) fn write_to_fol_hlink(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:folHlink
        self.write_to(writer, "a:folHlink");
    }

    pub(crate) fn write_to_hlink(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:hlink
        self.write_to(writer, "a:hlink");
    }

    pub(crate) fn write_to_lt1(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lt1
        self.write_to(writer, "a:lt1");
    }

    pub(crate) fn write_to_lt2(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:lt2
        self.write_to(writer, "a:lt2");
    }

    fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, tag_name: &str) {
        // a:clrScheme
        write_start_tag(writer, tag_name, vec![], false);

        // a:srgbClr
        if let Some(v) = &self.rgb_color_model_hex {
            v.write_to(writer);
        }

        // a:sysClr
        if let Some(v) = &self.system_color {
            v.write_to(writer);
        }

        write_end_tag(writer, tag_name);
    }
}
