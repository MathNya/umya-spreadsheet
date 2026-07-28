// formula
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    Address,
    StringValue,
};
use crate::{
    helper::address::is_address,
    writer::driver::{
        write_end_tag,
        write_start_tag,
        write_text_node,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Formula {
    address:      Address,
    string_value: StringValue,
}

impl Formula {
    #[inline]
    #[must_use]
    pub fn address(&self) -> &Address {
        &self.address
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use address()")]
    pub fn get_address(&self) -> &Address {
        self.address()
    }

    #[inline]
    pub fn address_mut(&mut self) -> &mut Address {
        &mut self.address
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use address_mut()")]
    pub fn get_address_mut(&mut self) -> &mut Address {
        self.address_mut()
    }

    #[inline]
    #[must_use]
    pub fn address_str(&self) -> String {
        if self.string_value.has_value() {
            return self.string_value.value_str().to_string();
        }
        self.address.address()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use address_str()")]
    pub fn get_address_str(&self) -> String {
        self.address_str()
    }

    #[inline]
    pub fn set_address(&mut self, value: Address) -> &mut Self {
        self.address = value;
        self.string_value.remove_value();
        self
    }

    #[inline]
    pub fn set_string_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.address = Address::default();
        self.string_value.set_value(value);
        self
    }

    #[inline]
    pub fn set_address_str<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let value = value.into();
        if is_address(&value) {
            self.address.set_address(value);
        } else {
            self.set_string_value(value);
        }
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        let text = reader
                    .read_text_into(e.name(), &mut buf).unwrap();
        self.set_address_str(crate::helper::utils::unescape_xml_text(&text));
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // formula
        write_start_tag(writer, "formula", vec![], false);
        write_text_node(writer, self.address_str());
        write_end_tag(writer, "formula");
    }
}
