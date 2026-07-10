// t
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node,
    },
};

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Text {
    value: Box<str>,
}

impl Text {
    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use value()")]
    pub fn get_value(&self) -> &str {
        self.value()
    }

    #[inline]
    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value = value.into().into_boxed_str();
        self
    }

    #[inline]
    pub(crate) fn hash_code(&self) -> String {
        crate::helper::utils::md5_hash(&*self.value)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use hash_code()")]
    pub(crate) fn get_hash_code(&self) -> String {
        self.hash_code()
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        let text = reader
                    .read_text_into(e.name(), &mut buf).unwrap();
        self.set_value(crate::helper::utils::unescape_xml_text(&text));
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // t
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.value.starts_with(|c: char| c.is_whitespace())
            || self.value.ends_with(|c: char| c.is_whitespace())
        {
            attributes.push(("xml:space", "preserve").into());
        }
        write_start_tag(writer, "t", attributes, false);
        write_text_node(writer, &*self.value);
        write_end_tag(writer, "t");
    }
}
