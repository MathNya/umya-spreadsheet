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
    x_window: UInt32Value,
    y_window: UInt32Value,
    window_width: UInt32Value,
    window_height: UInt32Value,
    tab_ratio: UInt32Value,
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
        set_string_from_xml!(self, e, active_tab, "activeTab");
        set_string_from_xml!(self, e, x_window, "xWindow");
        set_string_from_xml!(self, e, y_window, "yWindow");
        set_string_from_xml!(self, e, window_width, "windowWidth");
        set_string_from_xml!(self, e, window_height, "windowHeight");
        set_string_from_xml!(self, e, tab_ratio, "tabRatio");
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // selection
        let mut attributes = vec![
            //("xWindow", "240"),
            //("yWindow", "105"),
            //("windowWidth", "14805"),
            //("windowHeight", "8010"),
        ];
        let active_tab = self.active_tab.get_value_string();
        if self.active_tab.has_value() {
            attributes.push(("activeTab", active_tab.as_str()));
        }

        let x_window = self.x_window.get_value_string();
        if self.x_window.has_value() {
            attributes.push(("xWindow", x_window.as_str()));
        }


        let y_window = self.y_window.get_value_string();
        if self.y_window.has_value() {
            attributes.push(("yWindow", y_window.as_str()));
        }

        let window_width = self.window_width.get_value_string();
        if self.window_width.has_value() {
            attributes.push(("windowWidth", window_width.as_str()));
        }

        let window_height = self.window_height.get_value_string();
        if self.window_height.has_value() {
            attributes.push(("windowHeight", window_height.as_str()));
        }

        let tab_ratio = self.tab_ratio.get_value_string();
        if self.tab_ratio.has_value() {
            attributes.push(("tabRatio", tab_ratio.as_str()));
        }

        // workbookView
        write_start_tag(writer, "workbookView", attributes, true);
    }
}
