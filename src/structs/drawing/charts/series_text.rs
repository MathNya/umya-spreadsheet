// c:tx
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    structs::StringValue,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node,
    },
};

#[derive(Clone, Default, Debug)]
pub struct SeriesText {
    value: StringValue,
}

impl SeriesText {
    #[must_use]
    pub fn value(&self) -> &str {
        self.value.value_str()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use value()")]
    pub fn get_value(&self) -> &str {
        self.value()
    }

    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        let text = reader.read_text_into(e.name(), &mut buf).unwrap();
        self.set_value(crate::helper::utils::unescape_xml_text(&text));
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:tx
        write_start_tag(writer, "c:tx", vec![], false);

        // c:v
        write_start_tag(writer, "c:v", vec![], false);
        write_text_node(writer, self.value.value_str());
        write_end_tag(writer, "c:v");

        write_end_tag(writer, "c:tx");
    }
}
