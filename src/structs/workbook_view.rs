use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::UInt32Value;
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Default, Debug)]
pub struct WorkbookView {
    active_tab: UInt32Value,
}

impl WorkbookView {
    #[must_use]
    pub fn get_active_tab(&self) -> u32 {
        self.active_tab.get_value()
    }

    pub fn set_active_tab(&mut self, value: u32) -> &mut Self {
        self.active_tab.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, active_tab, "activeTab");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // selection
        let mut attributes: crate::structs::AttrCollection = vec![
            ("xWindow", "240").into(),
            ("yWindow", "105").into(),
            ("windowWidth", "14805").into(),
            ("windowHeight", "8010").into(),
        ];
        let active_tab = self.active_tab.get_value_string();
        if self.active_tab.has_value() {
            attributes.push(("activeTab", &active_tab).into());
        }

        // workbookView
        write_start_tag(writer, "workbookView", attributes, true);
    }
}
