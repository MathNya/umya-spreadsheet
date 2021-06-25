// c:numFmt
use writer::driver::*;
use reader::driver::*;
use quick_xml::Reader;
use quick_xml::events::{BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct NumberingFormat {
    format_code: String,
    source_linked: String,
}
impl NumberingFormat {
    pub fn get_format_code(&self)-> &str {
        &self.format_code
    }

    pub fn set_format_code<S: Into<String>>(&mut self, value:S)-> &mut NumberingFormat {
        self.format_code = value.into();
        self
    }

    pub fn get_source_linked(&self)-> &str {
        &self.source_linked
    }

    pub fn set_source_linked<S: Into<String>>(&mut self, value:S)-> &mut NumberingFormat {
        self.source_linked = value.into();
        self
    }

    pub(crate) fn set_attributes(
        &mut self,
        _reader:&mut Reader<std::io::BufReader<std::fs::File>>,
        e:&BytesStart
    ) {
        &mut self.set_format_code(get_attribute(e, b"formatCode").unwrap());
        &mut self.set_source_linked(get_attribute(e, b"sourceLinked").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:numFmt
        write_start_tag(writer, "c:numFmt", vec![
            ("formatCode", &self.format_code),
            ("sourceLinked", &self.source_linked),
        ], true);
    }
}
