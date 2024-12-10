// worksheetSource
use crate::helper::const_str::*;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use crate::reader::driver::*;
use std::io::Cursor;
use crate::structs::Address;
use crate::writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct WorksheetSource {
    address: Address,
}

impl WorksheetSource {
    pub fn get_address(&self) -> &Address {
        &self.address
    }

    pub fn get_address_mut(&mut self) -> &mut Address {
        &mut self.address
    }

    pub fn set_address(&mut self, value: Address) -> &mut Self {
        self.address = value;
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut address = Address::default();
        if let Some(v) = get_attribute(e, b"ref") {
            address.get_range_mut().set_range(v);
        }
        if let Some(v) = get_attribute(e, b"sheet") {
            address.set_sheet_name(v);
        }
        self.set_address(address);
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // worksheetSource
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let ref_str = self.address.get_range().get_range();
        attributes.push(("ref", ref_str.as_str()));
        if self.address.get_sheet_name() != "" {
            attributes.push(("sheet", self.address.get_sheet_name()));
        }
        write_start_tag(writer, "worksheetSource", attributes, true);
    }
}
