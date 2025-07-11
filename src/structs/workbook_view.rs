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
    x_window: UInt32Value,
    y_window: UInt32Value,
    window_width: UInt32Value,
    window_height: UInt32Value,
    tab_ratio: UInt32Value,
}

impl WorkbookView {
    #[inline]
    #[must_use]
    pub fn get_active_tab(&self) -> u32 {
        self.active_tab.value()
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
        let mut attributes: crate::structs::AttrCollection = Vec::new();

        let active_tab = self.active_tab.value_string();
        if self.active_tab.has_value() {
            attributes.push(("activeTab", &active_tab).into());
        }

        let x_window = self.x_window.value_string();
        if self.x_window.has_value() {
            attributes.push(("xWindow", &x_window).into());
        }

        let y_window = self.y_window.value_string();
        if self.y_window.has_value() {
            attributes.push(("yWindow", &y_window).into());
        }

        let window_width = self.window_width.value_string();
        if self.window_width.has_value() {
            attributes.push(("windowWidth", &window_width).into());
        }

        let window_height = self.window_height.value_string();
        if self.window_height.has_value() {
            attributes.push(("windowHeight", &window_height).into());
        }

        let tab_ratio = self.tab_ratio.value_string();
        if self.tab_ratio.has_value() {
            attributes.push(("tabRatio", &tab_ratio).into());
        }

        // workbookView
        write_start_tag(writer, "workbookView", attributes, true);
    }
}
