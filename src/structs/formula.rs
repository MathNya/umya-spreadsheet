// formula
use super::Address;
use super::StringValue;
use crate::helper::address::*;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct Formula {
    address: Address,
    string_value: StringValue,
}

impl Formula {
    #[inline]
    pub fn get_address(&self) -> &Address {
        &self.address
    }

    #[inline]
    pub fn get_address_mut(&mut self) -> &mut Address {
        &mut self.address
    }

    #[inline]
    pub fn get_address_str(&self) -> String {
        if self.string_value.has_value() {
            return self.string_value.get_value_str().to_string();
        }
        self.address.get_address()
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
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Text(e) => {
                self.set_address_str(e.unescape().unwrap());
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"formula" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "formula")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // formula
        write_start_tag(writer, "formula", vec![], false);
        write_text_node(writer, self.get_address_str());
        write_end_tag(writer, "formula");
    }
}
