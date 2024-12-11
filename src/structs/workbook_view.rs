use super::UInt32Value;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct WorkbookView {
    active_tab: UInt32Value,
}

impl WorkbookView {
    #[inline]
    pub fn get_active_tab(&self) -> &u32 {
        self.active_tab.get_value()
    }

    #[inline]
    pub fn set_active_tab(&mut self, value: u32) -> &mut Self {
        self.active_tab.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        set_string_from_xml!(self, e, active_tab, "activeTab")
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // selection
        let mut attributes = vec![
            ("xWindow", "240"),
            ("yWindow", "105"),
            ("windowWidth", "14805"),
            ("windowHeight", "8010"),
        ];
        let active_tab = self.active_tab.get_value_string();
        if self.active_tab.has_value() {
            attributes.push(("activeTab", &active_tab));
        }

        // workbookView
        write_start_tag(writer, "workbookView", attributes, true);
    }
}
