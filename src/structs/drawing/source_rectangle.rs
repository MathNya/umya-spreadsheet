// a:srcRect
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct SourceRectangle {
    t: Option<String>,
    l: Option<String>,
    r: Option<String>,
    b: Option<String>,
}
impl SourceRectangle {
    pub fn set_t<S: Into<String>>(&mut self, value: S) {
        self.t = Some(value.into());
    }

    pub fn get_t(&self) -> Option<&String> {
        self.t.as_ref()
    }

    pub fn set_l<S: Into<String>>(&mut self, value: S) {
        self.l = Some(value.into());
    }

    pub fn get_l(&self) -> Option<&String> {
        self.l.as_ref()
    }

    pub fn set_r<S: Into<String>>(&mut self, value: S) {
        self.r = Some(value.into());
    }

    pub fn get_r(&self) -> Option<&String> {
        self.r.as_ref()
    }

    pub fn set_b<S: Into<String>>(&mut self, value: S) {
        self.b = Some(value.into());
    }

    pub fn get_b(&self) -> Option<&String> {
        self.b.as_ref()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        for a in e.attributes().with_checks(false) {
            if let Ok(attr) = a {
                match attr.key.0 {
                    b"t" => self.set_t(get_attribute_value(&attr).unwrap()),
                    b"l" => self.set_l(get_attribute_value(&attr).unwrap()),
                    b"r" => self.set_r(get_attribute_value(&attr).unwrap()),
                    b"b" => self.set_b(get_attribute_value(&attr).unwrap()),
                    _ => {}
                }
            }
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // a:srcRect
        let mut attributes: Vec<(&str, &str)> = Vec::new();

        if let Some(v) = &self.t {
            attributes.push(("t", v))
        }
        if let Some(v) = &self.l {
            attributes.push(("l", v))
        }
        if let Some(v) = &self.r {
            attributes.push(("r", v))
        }
        if let Some(v) = &self.b {
            attributes.push(("b", v))
        }
        write_start_tag(writer, "a:srcRect", attributes, true);
    }
}
