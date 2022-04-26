// a:srgbClr
use super::super::super::Int32Value;
use super::super::super::StringValue;
use super::SaturationModulation;
use super::Shade;
use super::Tint;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct RgbColorModelHex {
    val: StringValue,
    red: Int32Value,
    green: Int32Value,
    blue: Int32Value,
    tint: Option<Tint>,
    shade: Option<Shade>,
    saturation_modulation: Option<SaturationModulation>,
}
impl RgbColorModelHex {
    pub fn get_val(&self) -> &str {
        self.val.get_value()
    }

    pub fn set_val<S: Into<String>>(&mut self, value: S) -> &mut RgbColorModelHex {
        self.val.set_value(value);
        self
    }

    pub fn get_red(&self) -> &i32 {
        self.red.get_value()
    }

    pub fn set_red(&mut self, value: i32) -> &mut RgbColorModelHex {
        self.red.set_value(value);
        self
    }

    pub fn get_green(&self) -> &i32 {
        self.green.get_value()
    }

    pub fn set_green(&mut self, value: i32) -> &mut RgbColorModelHex {
        self.green.set_value(value);
        self
    }

    pub fn get_blue(&self) -> &i32 {
        self.blue.get_value()
    }

    pub fn set_blue(&mut self, value: i32) -> &mut RgbColorModelHex {
        self.blue.set_value(value);
        self
    }

    pub fn get_tint(&self) -> &Option<Tint> {
        &self.tint
    }

    pub fn get_tint_mut(&mut self) -> &mut Option<Tint> {
        &mut self.tint
    }

    pub fn set_tint(&mut self, value: Tint) -> &mut RgbColorModelHex {
        self.tint = Some(value);
        self
    }

    pub fn get_shade(&self) -> &Option<Shade> {
        &self.shade
    }

    pub fn get_shade_mut(&mut self) -> &mut Option<Shade> {
        &mut self.shade
    }

    pub fn set_shade(&mut self, value: Shade) -> &mut RgbColorModelHex {
        self.shade = Some(value);
        self
    }

    pub fn get_saturation_modulation(&self) -> &Option<SaturationModulation> {
        &self.saturation_modulation
    }

    pub fn get_saturation_modulation_mut(&mut self) -> &mut Option<SaturationModulation> {
        &mut self.saturation_modulation
    }

    pub fn set_saturation_modulation(
        &mut self,
        value: SaturationModulation,
    ) -> &mut RgbColorModelHex {
        self.saturation_modulation = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        for a in e.attributes().with_checks(false) {
            match a {
                Ok(ref attr) if attr.key == b"r" => {
                    self.red
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key == b"g" => {
                    self.green
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key == b"b" => {
                    self.blue
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(ref attr) if attr.key == b"val" => {
                    self.val
                        .set_value_string(get_attribute_value(attr).unwrap());
                }
                Ok(_) => {}
                Err(_) => {}
            }
        }
        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name() {
                    b"a:tint" => {
                        let mut obj = Tint::default();
                        obj.set_attributes(reader, e);
                        self.set_tint(obj);
                    }
                    b"a:shade" => {
                        let mut obj = Shade::default();
                        obj.set_attributes(reader, e);
                        self.set_shade(obj);
                    }
                    b"a:satMod" => {
                        let mut obj = SaturationModulation::default();
                        obj.set_attributes(reader, e);
                        self.set_saturation_modulation(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name() {
                    b"a:srgbClr" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "a:srgbClr"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let empty_flag =
            self.tint.is_none() && self.shade.is_none() && self.saturation_modulation.is_none();

        // a:srgbClr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let r = self.red.get_value_string();
        if &self.red.has_value() == &true {
            attributes.push(("r", &r));
        }
        let g = self.green.get_value_string();
        if &self.green.has_value() == &true {
            attributes.push(("g", &g));
        }
        let b = self.blue.get_value_string();
        if &self.blue.has_value() == &true {
            attributes.push(("b", &b));
        }
        if &self.val.has_value() == &true {
            attributes.push(("val", self.val.get_value_string()));
        }
        write_start_tag(writer, "a:srgbClr", attributes, empty_flag);

        if !empty_flag {
            // a:tint
            match &self.tint {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            // a:shade
            match &self.shade {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            // a:satMod
            match &self.saturation_modulation {
                Some(v) => {
                    v.write_to(writer);
                }
                None => {}
            }

            write_end_tag(writer, "a:srgbClr");
        }
    }
}
