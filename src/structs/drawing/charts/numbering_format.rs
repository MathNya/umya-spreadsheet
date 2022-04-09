// c:numFmt
use super::super::super::BooleanValue;
use super::super::super::StringValue;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct NumberingFormat {
    format_code: StringValue,
    source_linked: BooleanValue,
}
impl NumberingFormat {
    pub fn get_format_code(&self) -> &str {
        self.format_code.get_value()
    }

    pub fn set_format_code<S: Into<String>>(&mut self, value: S) -> &mut NumberingFormat {
        self.format_code.set_value(value);
        self
    }

    pub fn get_source_linked(&self) -> &bool {
        self.source_linked.get_value()
    }

    pub fn set_source_linked(&mut self, value: bool) -> &mut NumberingFormat {
        self.source_linked.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        self.format_code
            .set_value_string(get_attribute(e, b"formatCode").unwrap());
        self.source_linked
            .set_value_string(get_attribute(e, b"sourceLinked").unwrap());
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:numFmt
        write_start_tag(
            writer,
            "c:numFmt",
            vec![
                ("formatCode", self.format_code.get_value_string()),
                ("sourceLinked", self.source_linked.get_value_string()),
            ],
            true,
        );
    }
}
