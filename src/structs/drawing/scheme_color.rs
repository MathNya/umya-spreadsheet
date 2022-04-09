// a:schemeClr
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct SchemeColor {
    val: String,
    lum_mod: Option<String>,
    lum_off: Option<String>,
    shade: Option<String>,
    sat_mod: Option<String>,
    alpha: Option<String>,
}
impl SchemeColor {
    pub fn set_val<S: Into<String>>(&mut self, value: S) {
        self.val = value.into();
    }

    pub fn get_val(&self) -> &str {
        &self.val
    }

    pub fn set_lum_mod<S: Into<String>>(&mut self, value: S) {
        self.lum_mod = Some(value.into());
    }

    pub fn get_lum_mod(&self) -> &Option<String> {
        &self.lum_mod
    }

    pub fn set_lum_off<S: Into<String>>(&mut self, value: S) {
        self.lum_off = Some(value.into());
    }

    pub fn get_lum_off(&self) -> &Option<String> {
        &self.lum_off
    }

    pub fn set_shade<S: Into<String>>(&mut self, value: S) {
        self.shade = Some(value.into());
    }

    pub fn get_shade(&self) -> &Option<String> {
        &self.shade
    }

    pub fn set_sat_mod<S: Into<String>>(&mut self, value: S) {
        self.sat_mod = Some(value.into());
    }

    pub fn get_sat_mod(&self) -> &Option<String> {
        &self.sat_mod
    }

    pub fn set_alpha<S: Into<String>>(&mut self, value: S) {
        self.alpha = Some(value.into());
    }

    pub fn get_alpha(&self) -> &Option<String> {
        &self.alpha
    }

    pub(crate) fn with_inner_params(&self) -> bool {
        self.lum_mod.is_some()
            || self.lum_off.is_some()
            || self.shade.is_some()
            || self.sat_mod.is_some()
            || self.alpha.is_some()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        self.set_val(get_attribute(e, b"val").unwrap());

        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name() {
                    b"a:lumMod" => {
                        self.set_lum_mod(get_attribute(e, b"val").unwrap());
                    }
                    b"a:lumOff" => {
                        self.set_lum_off(get_attribute(e, b"val").unwrap());
                    }
                    b"a:shade" => {
                        self.set_shade(get_attribute(e, b"val").unwrap());
                    }
                    b"a:satMod" => {
                        self.set_sat_mod(get_attribute(e, b"val").unwrap());
                    }
                    b"a:alpha" => {
                        self.set_alpha(get_attribute(e, b"val").unwrap());
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"a:schemeClr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:schemeClr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:schemeClr
        if self.with_inner_params() {
            write_start_tag(writer, "a:schemeClr", vec![("val", &self.val)], false);

            // a:lumMod
            match &self.lum_mod {
                Some(v) => {
                    write_start_tag(writer, "a:lumMod", vec![("val", v)], true);
                }
                None => {}
            }

            // a:lumOff
            match &self.lum_off {
                Some(v) => {
                    write_start_tag(writer, "a:lumOff", vec![("val", v)], true);
                }
                None => {}
            }

            // a:shade
            match &self.shade {
                Some(v) => {
                    write_start_tag(writer, "a:shade", vec![("val", v)], true);
                }
                None => {}
            }

            // a:satMod
            match &self.sat_mod {
                Some(v) => {
                    write_start_tag(writer, "a:satMod", vec![("val", v)], true);
                }
                None => {}
            }

            // a:alpha
            match &self.alpha {
                Some(v) => {
                    write_start_tag(writer, "a:alpha", vec![("val", v)], true);
                }
                None => {}
            }

            write_end_tag(writer, "a:schemeClr");
        } else {
            write_start_tag(writer, "a:schemeClr", vec![("val", &self.val)], true);
        }
    }
}
