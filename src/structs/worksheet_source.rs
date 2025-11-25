// worksheetSource
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use crate::{
    reader::driver::get_attribute,
    structs::Address,
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct WorksheetSource {
    address: Address,
}

impl WorksheetSource {
    #[must_use]
    pub fn address(&self) -> &Address {
        &self.address
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use address()")]
    pub fn get_address(&self) -> &Address {
        self.address()
    }
    
    pub fn address_mut(&mut self) -> &mut Address {
        &mut self.address
    }

    #[deprecated(since = "3.0.0", note = "Use address_mut()")]
    pub fn get_address_mut(&mut self) -> &mut Address {
        self.address_mut()
    }

    pub fn set_address(&mut self, value: Address) -> &mut Self {
        self.address = value;
        self
    }

    /// Create a new worksheet source with sheet name and range
    pub fn new_simple(sheet: impl Into<String>, reference: impl Into<String>) -> Self {
        let mut ws_source = Self::default();
        let mut address = Address::default();
        address.set_sheet_name(sheet);
        address.range_mut().set_range(reference);
        ws_source.set_address(address);
        ws_source
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        let mut address = Address::default();
        if let Some(v) = get_attribute(e, b"ref") {
            address.range_mut().set_range(v);
        }
        if let Some(v) = get_attribute(e, b"sheet") {
            address.set_sheet_name(v);
        }
        self.set_address(address);
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // worksheetSource
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let ref_str = self.address.range().range();
        attributes.push(("ref", &ref_str).into());
        if self.address.sheet_name() != "" {
            attributes.push(("sheet", self.address.sheet_name()).into());
        }
        write_start_tag(writer, "worksheetSource", attributes, true);
    }
}
