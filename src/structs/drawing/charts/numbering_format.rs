// c:numFmt
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::super::super::{
    BooleanValue,
    StringValue,
};
use crate::{
    reader::driver::get_attribute,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct NumberingFormat {
    format_code:   StringValue,
    source_linked: BooleanValue,
}
impl NumberingFormat {
    #[must_use]
    pub fn get_format_code(&self) -> &str {
        self.format_code.get_value_str()
    }

    pub fn set_format_code<S: Into<String>>(&mut self, value: S) -> &mut NumberingFormat {
        self.format_code.set_value(value);
        self
    }

    #[must_use]
    pub fn get_source_linked(&self) -> bool {
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
                ("formatCode", self.format_code.get_value_str()),
                ("sourceLinked", self.source_linked.get_value_string()),
            ],
            true,
        );
    }
}
