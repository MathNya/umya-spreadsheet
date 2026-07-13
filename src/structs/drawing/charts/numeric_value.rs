// c:v
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::writer::driver::{
    write_end_tag,
    write_start_tag,
    write_text_node,
};

#[derive(Clone, Default, Debug)]
pub struct NumericValue {
    text: Box<str>,
}

impl NumericValue {
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use text()")]
    pub fn get_text(&self) -> &str {
        self.text()
    }

    pub fn set_text<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.text = value.into().into_boxed_str();
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        let text = reader.read_text_into(e.name(), &mut buf).unwrap();
        self.set_text(crate::helper::utils::unescape_xml_text(&text));
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // c:v
        write_start_tag(writer, "c:v", vec![], false);
        write_text_node(writer, &*self.text);
        write_end_tag(writer, "c:v");
    }
}
