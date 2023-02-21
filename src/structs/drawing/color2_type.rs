use super::RgbColorModelHex;
use super::SystemColor;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
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

    pub fn get_rgb_color_model_hex(&self) -> &Option<RgbColorModelHex> {
        &self.rgb_color_model_hex
    }

    pub fn get_rgb_color_model_hex_mut(&mut self) -> &mut Option<RgbColorModelHex> {
        &mut self.rgb_color_model_hex
    }

    pub fn set_system_color(&mut self, value: SystemColor) {
        self.system_color = Some(value);
    }

    pub fn get_system_color(&self) -> &Option<SystemColor> {
        &self.system_color
    }

    pub fn get_system_color_mut(&mut self) -> &mut Option<SystemColor> {
        &mut self.system_color
    }

    pub fn get_val(&self) -> String {
        match &self.rgb_color_model_hex {
            Some(v) => return v.get_val().to_string(),
            _ => {}
        }
        match &self.system_color {
            Some(v) => return v.get_last_color().to_string(),
            _ => {}
        }
        String::from("")
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"a:srgbClr" => {
                        let mut obj = RgbColorModelHex::default();
                        obj.set_attributes(reader, e, false);
                        self.rgb_color_model_hex = Some(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
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
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "Color2Type"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
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
        match &self.rgb_color_model_hex {
            Some(v) => {
                v.write_to(writer);
            }
            _ => {}
        }

        // a:sysClr
        match &self.system_color {
            Some(v) => {
                v.write_to(writer);
            }
            _ => {}
        }

        write_end_tag(writer, tag_name);
    }
}
