// a:bodyPr
use writer::driver::*;
use reader::driver::*;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use quick_xml::Reader;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct BodyProperties {
    vert_overflow: Option<String>,
    horz_overflow: Option<String>,
    rtl_col: Option<String>,
    anchor: Option<String>,
}
impl BodyProperties {
    pub fn get_vert_overflow(&self)-> &Option<String> {
        &self.vert_overflow
    }
    pub fn set_vert_overflow<S: Into<String>>(&mut self, value:S) {
        self.vert_overflow = Some(value.into());
    }
    pub fn get_horz_overflow(&self)-> &Option<String> {
        &self.horz_overflow
    }
    pub fn set_horz_overflow<S: Into<String>>(&mut self, value:S) {
        self.horz_overflow = Some(value.into());
    }
    pub fn get_rtl_col(&self)-> &Option<String> {
        &self.rtl_col
    }
    pub fn set_rtl_col<S: Into<String>>(&mut self, value:S) {
        self.rtl_col = Some(value.into());
    }
    pub fn get_anchor(&self)-> &Option<String> {
        &self.anchor
    }
    pub fn set_anchor<S: Into<String>>(&mut self, value:S) {
        self.anchor = Some(value.into());
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        for a in e.attributes().with_checks(false) {
            match a {
                Ok(ref attr) if attr.key == b"vertOverflow" => {
                    &mut self.set_vert_overflow(get_attribute_value(attr).unwrap());
                },
                Ok(ref attr) if attr.key == b"horzOverflow" => {
                    &mut self.set_horz_overflow(get_attribute_value(attr).unwrap());
                },
                Ok(ref attr) if attr.key == b"rtlCol" => {
                    &mut self.set_rtl_col(get_attribute_value(attr).unwrap());
                },
                Ok(ref attr) if attr.key == b"anchor" => {
                    &mut self.set_anchor(get_attribute_value(attr).unwrap());
                },
                Ok(_) => {},
                Err(_) => {},
            }
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:bodyPr
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        match &self.vert_overflow {
            Some(v) => attributes.push(("vertOverflow", v)),
            None => {}
        }
        match &self.horz_overflow {
            Some(v) => attributes.push(("horzOverflow", v)),
            None => {}
        }
        match &self.rtl_col {
            Some(v) => attributes.push(("rtlCol", v)),
            None => {}
        }
        match &self.anchor {
            Some(v) => attributes.push(("anchor", v)),
            None => {}
        }
        write_start_tag(writer, "a:bodyPr", attributes, true);
    }
}