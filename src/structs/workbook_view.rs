use super::Int32Value;
use super::UInt32Value;
use crate::reader::driver::*;
use crate::writer::driver::*;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Debug)]
pub struct WorkbookView {
    active_tab: UInt32Value,
    x_window: Int32Value,
    y_window: Int32Value,
    window_width: Int32Value,
    window_height: Int32Value,
    tab_ratio: Int32Value,
}
impl Default for WorkbookView {
    #[inline]
    fn default() -> Self {
        let mut x_window = Int32Value::default();
        x_window.set_value(240);
        let mut y_window = Int32Value::default();
        y_window.set_value(105);
        let mut window_width = Int32Value::default();
        window_width.set_value(14805);
        let mut window_height = Int32Value::default();
        window_height.set_value(8010);
        Self {
            active_tab: UInt32Value::default(),
            x_window,
            y_window,
            window_width,
            window_height,
            tab_ratio: Int32Value::default(),
        }
    }
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
    pub fn get_x_window(&self) -> &i32 {
        self.x_window.get_value()
    }

    #[inline]
    pub fn set_x_window(&mut self, value: i32) -> &mut Self {
        self.x_window.set_value(value);
        self
    }

    #[inline]
    pub fn get_y_window(&self) -> &i32 {
        self.y_window.get_value()
    }

    #[inline]
    pub fn set_y_window(&mut self, value: i32) -> &mut Self {
        self.y_window.set_value(value);
        self
    }

    #[inline]
    pub fn get_window_width(&self) -> &i32 {
        self.window_width.get_value()
    }

    #[inline]
    pub fn set_window_width(&mut self, value: i32) -> &mut Self {
        self.window_width.set_value(value);
        self
    }

    #[inline]
    pub fn get_window_height(&self) -> &i32 {
        self.window_height.get_value()
    }

    #[inline]
    pub fn set_window_height(&mut self, value: i32) -> &mut Self {
        self.window_height.set_value(value);
        self
    }

    #[inline]
    pub fn get_tab_ratio(&self) -> &i32 {
        self.tab_ratio.get_value()
    }

    #[inline]
    pub fn set_tab_ratio(&mut self, value: i32) -> &mut Self {
        self.tab_ratio.set_value(value);
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
        let mut attributes: Vec<(&str, &str)> = Vec::new();
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
