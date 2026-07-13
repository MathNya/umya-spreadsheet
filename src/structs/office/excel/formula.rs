// xm:f
use std::{
    io::Cursor,
    vec,
};

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    structs::Address,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node,
    },
};

#[derive(Default, Debug, Clone)]
pub struct Formula {
    value: Address,
}
impl Formula {
    #[inline]
    #[must_use]
    pub fn value(&self) -> &Address {
        &self.value
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use val()")]
    pub fn get_value(&self) -> &Address {
        self.value()
    }

    #[inline]
    pub fn value_mut(&mut self) -> &mut Address {
        &mut self.value
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use val()")]
    pub fn get_value_mut(&mut self) -> &mut Address {
        self.value_mut()
    }

    #[inline]
    pub fn set_value(&mut self, value: Address) -> &mut Self {
        self.value = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        let text = reader.read_text_into(e.name(), &mut buf).unwrap();
        let mut obj = Address::default();
        obj.set_address(crate::helper::utils::unescape_xml_text(&text));
        self.value = obj;
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        write_start_tag(writer, "xm:f", vec![], false);
        write_text_node(writer, self.value.address());
        write_end_tag(writer, "xm:f");
    }
}
