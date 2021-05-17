// a:srgbClr
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct RgbColorModelHex {
    val: Option<String>,
    r: Option<String>,
    g: Option<String>,
    b: Option<String>,
}
impl RgbColorModelHex {
    pub fn set_val<S: Into<String>>(&mut self, value:S) {
        self.val = Some(value.into());
    }

    pub fn get_val(&self) -> &Option<String> {
        &self.val
    }

    pub fn set_r<S: Into<String>>(&mut self, value:S) {
        self.r = Some(value.into());
    }

    pub fn get_r(&self) -> &Option<String> {
        &self.r
    }

    pub fn set_g<S: Into<String>>(&mut self, value:S) {
        self.g = Some(value.into());
    }

    pub fn get_g(&self) -> &Option<String> {
        &self.g
    }

    pub fn set_b<S: Into<String>>(&mut self, value:S) {
        self.b = Some(value.into());
    }

    pub fn get_b(&self) -> &Option<String> {
        &self.b
    }
    
    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        for a in e.attributes().with_checks(false) {
            match a {
                Ok(ref attr) if attr.key == b"r" => {
                    &mut self.set_r(get_attribute_value(attr).unwrap());
                },
                Ok(ref attr) if attr.key == b"g" => {
                    &mut self.set_g(get_attribute_value(attr).unwrap());
                },
                Ok(ref attr) if attr.key == b"b" => {
                    &mut self.set_b(get_attribute_value(attr).unwrap());
                },
                Ok(ref attr) if attr.key == b"val" => {
                    &mut self.set_val(get_attribute_value(attr).unwrap());
                },
                Ok(_) => {},
                Err(_) => {},
            }
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:srgbClr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match &self.val {
            Some(v) => attributes.push(("val", v)),
            None => {}
        }
        match &self.r {
            Some(v) => attributes.push(("r", v)),
            None => {}
        }
        match &self.g {
            Some(v) => attributes.push(("g", v)),
            None => {}
        }
        match &self.b {
            Some(v) => attributes.push(("b", v)),
            None => {}
        }
        write_start_tag(writer, "a:srgbClr", attributes, true);
    }
}
