use super::UInt32Value;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct WorkbookView {
    active_tab: UInt32Value,
}
impl WorkbookView {
    pub fn get_active_tab(&self) -> &u32 {
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
        match get_attribute(e, b"activeTab") {
            Some(v) => {
                self.active_tab.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // selection
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        attributes.push(("xWindow", "240"));
        attributes.push(("yWindow", "105"));
        attributes.push(("windowWidth", "14805"));
        attributes.push(("windowHeight", "8010"));
        let active_tab = self.active_tab.get_value_string();
        if self.active_tab.has_value() {
            attributes.push(("activeTab", &active_tab));
        }

        // workbookView
        write_start_tag(writer, "workbookView", attributes, true);
    }
}
