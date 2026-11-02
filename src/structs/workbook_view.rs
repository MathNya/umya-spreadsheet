use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::BytesStart,
};

use super::{
    BooleanValue,
    EnumValue,
    Int32Value,
    UInt32Value,
    VisibilityValues,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
    },
    writer::driver::write_start_tag,
};

#[derive(Clone, Debug)]
pub struct WorkbookView {
    active_tab: UInt32Value,
    x_window: Int32Value,
    y_window: Int32Value,
    window_width: Int32Value,
    window_height: Int32Value,
    tab_ratio: Int32Value,
    visibility: EnumValue<VisibilityValues>,
    minimized: BooleanValue,
    show_horizontal_scroll: BooleanValue,
    show_vertical_scroll: BooleanValue,
    show_sheet_tabs: BooleanValue,
    first_sheet: UInt32Value,
    auto_filter_date_grouping: BooleanValue,
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
            visibility: EnumValue::default(),
            minimized: BooleanValue::default(),
            show_horizontal_scroll: BooleanValue::default(),
            show_vertical_scroll: BooleanValue::default(),
            show_sheet_tabs: BooleanValue::default(),
            first_sheet: UInt32Value::default(),
            auto_filter_date_grouping: BooleanValue::default(),
        }
    }
}
impl WorkbookView {
    #[inline]
    #[must_use]
    pub fn active_tab(&self) -> u32 {
        self.active_tab.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use active_tab()")]
    pub fn get_active_tab(&self) -> u32 {
        self.active_tab()
    }

    #[inline]
    pub fn set_active_tab(&mut self, value: u32) -> &mut Self {
        self.active_tab.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn x_window(&self) -> i32 {
        self.x_window.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use x_window()")]
    pub fn get_x_window(&self) -> i32 {
        self.x_window()
    }

    #[inline]
    pub fn set_x_window(&mut self, value: i32) -> &mut Self {
        self.x_window.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn y_window(&self) -> i32 {
        self.y_window.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use y_window()")]
    pub fn get_y_window(&self) -> i32 {
        self.y_window()
    }

    #[inline]
    pub fn set_y_window(&mut self, value: i32) -> &mut Self {
        self.y_window.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn window_width(&self) -> i32 {
        self.window_width.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use window_width()")]
    pub fn get_window_width(&self) -> i32 {
        self.window_width()
    }

    #[inline]
    pub fn set_window_width(&mut self, value: i32) -> &mut Self {
        self.window_width.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn window_height(&self) -> i32 {
        self.window_height.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use window_height()")]
    pub fn get_window_height(&self) -> i32 {
        self.window_height()
    }

    #[inline]
    pub fn set_window_height(&mut self, value: i32) -> &mut Self {
        self.window_height.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn tab_ratio(&self) -> i32 {
        self.tab_ratio.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use tab_ratio()")]
    pub fn get_tab_ratio(&self) -> i32 {
        self.tab_ratio()
    }

    #[inline]
    pub fn set_tab_ratio(&mut self, value: i32) -> &mut Self {
        self.tab_ratio.set_value(value);
        self
    }

    /// Visibility of the workbook window (`visibility`). Defaults to
    /// `Visible`.
    #[inline]
    #[must_use]
    pub fn visibility(&self) -> &VisibilityValues {
        self.visibility.value()
    }

    #[inline]
    pub fn set_visibility(&mut self, value: VisibilityValues) -> &mut Self {
        self.visibility.set_value(value);
        self
    }

    /// Whether the workbook window is minimized (`minimized`).
    #[inline]
    #[must_use]
    pub fn minimized(&self) -> bool {
        self.minimized.value()
    }

    #[inline]
    pub fn set_minimized(&mut self, value: bool) -> &mut Self {
        self.minimized.set_value(value);
        self
    }

    /// Whether the horizontal scroll bar is shown (`showHorizontalScroll`).
    /// Defaults to `true`.
    #[inline]
    #[must_use]
    pub fn show_horizontal_scroll(&self) -> bool {
        !self.show_horizontal_scroll.has_value() || self.show_horizontal_scroll.value()
    }

    #[inline]
    pub fn set_show_horizontal_scroll(&mut self, value: bool) -> &mut Self {
        self.show_horizontal_scroll.set_value(value);
        self
    }

    /// Whether the vertical scroll bar is shown (`showVerticalScroll`).
    /// Defaults to `true`.
    #[inline]
    #[must_use]
    pub fn show_vertical_scroll(&self) -> bool {
        !self.show_vertical_scroll.has_value() || self.show_vertical_scroll.value()
    }

    #[inline]
    pub fn set_show_vertical_scroll(&mut self, value: bool) -> &mut Self {
        self.show_vertical_scroll.set_value(value);
        self
    }

    /// Whether the sheet tabs are shown (`showSheetTabs`). Defaults to
    /// `true`.
    #[inline]
    #[must_use]
    pub fn show_sheet_tabs(&self) -> bool {
        !self.show_sheet_tabs.has_value() || self.show_sheet_tabs.value()
    }

    #[inline]
    pub fn set_show_sheet_tabs(&mut self, value: bool) -> &mut Self {
        self.show_sheet_tabs.set_value(value);
        self
    }

    /// Index of the first sheet shown in the tab bar (`firstSheet`).
    #[inline]
    #[must_use]
    pub fn first_sheet(&self) -> u32 {
        self.first_sheet.value()
    }

    #[inline]
    pub fn set_first_sheet(&mut self, value: u32) -> &mut Self {
        self.first_sheet.set_value(value);
        self
    }

    /// Whether dates are grouped in the filter menus
    /// (`autoFilterDateGrouping`). Defaults to `true`.
    #[inline]
    #[must_use]
    pub fn auto_filter_date_grouping(&self) -> bool {
        !self.auto_filter_date_grouping.has_value() || self.auto_filter_date_grouping.value()
    }

    #[inline]
    pub fn set_auto_filter_date_grouping(&mut self, value: bool) -> &mut Self {
        self.auto_filter_date_grouping.set_value(value);
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
        set_string_from_xml!(self, e, visibility, "visibility");
        set_string_from_xml!(self, e, minimized, "minimized");
        set_string_from_xml!(self, e, show_horizontal_scroll, "showHorizontalScroll");
        set_string_from_xml!(self, e, show_vertical_scroll, "showVerticalScroll");
        set_string_from_xml!(self, e, show_sheet_tabs, "showSheetTabs");
        set_string_from_xml!(self, e, first_sheet, "firstSheet");
        set_string_from_xml!(self, e, auto_filter_date_grouping, "autoFilterDateGrouping");
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

        let visibility = self.visibility.value_string();
        if self.visibility.has_value() {
            attributes.push(("visibility", visibility).into());
        }

        let minimized = self.minimized.value_string();
        if self.minimized.has_value() {
            attributes.push(("minimized", minimized).into());
        }

        let show_horizontal_scroll = self.show_horizontal_scroll.value_string();
        if self.show_horizontal_scroll.has_value() {
            attributes.push(("showHorizontalScroll", show_horizontal_scroll).into());
        }

        let show_vertical_scroll = self.show_vertical_scroll.value_string();
        if self.show_vertical_scroll.has_value() {
            attributes.push(("showVerticalScroll", show_vertical_scroll).into());
        }

        let show_sheet_tabs = self.show_sheet_tabs.value_string();
        if self.show_sheet_tabs.has_value() {
            attributes.push(("showSheetTabs", show_sheet_tabs).into());
        }

        let first_sheet = self.first_sheet.value_string();
        if self.first_sheet.has_value() {
            attributes.push(("firstSheet", &first_sheet).into());
        }

        let auto_filter_date_grouping = self.auto_filter_date_grouping.value_string();
        if self.auto_filter_date_grouping.has_value() {
            attributes.push(("autoFilterDateGrouping", auto_filter_date_grouping).into());
        }

        // workbookView
        write_start_tag(writer, "workbookView", attributes, true);
    }
}
