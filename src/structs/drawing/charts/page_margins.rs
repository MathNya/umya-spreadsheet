// c:pageMargins
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct PageMargins {
    bottom: String,
    left: String,
    right: String,
    top: String,
    header: String,
    footer: String,
}
impl PageMargins {
    pub fn get_bottom(&self)-> &str {
        &self.bottom
    }

    pub fn set_bottom<S: Into<String>>(&mut self, value:S)-> &mut PageMargins {
        self.bottom = value.into();
        self
    }

    pub fn get_left(&self)-> &str {
        &self.left
    }

    pub fn set_left<S: Into<String>>(&mut self, value:S)-> &mut PageMargins {
        self.left = value.into();
        self
    }

    pub fn get_right(&self)-> &str {
        &self.right
    }

    pub fn set_right<S: Into<String>>(&mut self, value:S)-> &mut PageMargins {
        self.right = value.into();
        self
    }

    pub fn get_top(&self)-> &str {
        &self.top
    }

    pub fn set_top<S: Into<String>>(&mut self, value:S)-> &mut PageMargins {
        self.top = value.into();
        self
    }

    pub fn get_header(&self)-> &str {
        &self.header
    }

    pub fn set_header<S: Into<String>>(&mut self, value:S)-> &mut PageMargins {
        self.header = value.into();
        self
    }

    pub fn get_footer(&self)-> &str {
        &self.footer
    }

    pub fn set_footer<S: Into<String>>(&mut self, value:S)-> &mut PageMargins {
        self.footer = value.into();
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        self.bottom = get_attribute(e, b"b").unwrap();
        self.left = get_attribute(e, b"l").unwrap();
        self.right = get_attribute(e, b"r").unwrap();
        self.top = get_attribute(e, b"t").unwrap();
        self.header = get_attribute(e, b"header").unwrap();
        self.footer = get_attribute(e, b"footer").unwrap();
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:pageMargins
        write_start_tag(writer, "c:pageMargins", vec![
            ("b", &self.bottom),
            ("l", &self.left),
            ("r", &self.right),
            ("t", &self.top),
            ("header", &self.header),
            ("footer", &self.footer),
        ], true);
    }
}
