// sheetView
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::{
    BooleanValue,
    EnumValue,
    Pane,
    Selection,
    SheetViewValues,
    StringValue,
    UInt32Value,
};
use crate::{
    reader::driver::{
        get_attribute,
        set_string_from_xml,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct SheetView {
    window_protection:            BooleanValue,
    show_formulas:                BooleanValue,
    show_grid_lines:              BooleanValue,
    show_row_col_headers:         BooleanValue,
    show_zeros:                   BooleanValue,
    right_to_left:                BooleanValue,
    tab_selected:                 BooleanValue,
    show_ruler:                   BooleanValue,
    show_outline_symbols:         BooleanValue,
    default_grid_color:           BooleanValue,
    show_white_space:             BooleanValue,
    color_id:                     UInt32Value,
    workbook_view_id:             UInt32Value,
    pane:                         Option<Box<Pane>>,
    view:                         EnumValue<SheetViewValues>,
    zoom_scale:                   UInt32Value,
    zoom_scale_normal:            UInt32Value,
    zoom_scale_page_layout_view:  UInt32Value,
    zoom_scale_sheet_layout_view: UInt32Value,
    top_left_cell:                StringValue,
    selection:                    Vec<Selection>,
}

impl SheetView {
    #[inline]
    #[must_use]
    pub fn show_grid_lines(&self) -> bool {
        self.show_grid_lines.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use show_grid_lines()")]
    pub fn get_show_grid_lines(&self) -> bool {
        self.show_grid_lines()
    }

    #[inline]
    pub fn set_show_grid_lines(&mut self, value: bool) -> &mut Self {
        self.show_grid_lines.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn tab_selected(&self) -> bool {
        self.tab_selected.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use tab_selected()")]
    pub fn get_tab_selected(&self) -> bool {
        self.tab_selected()
    }

    #[inline]
    pub fn set_tab_selected(&mut self, value: bool) -> &mut Self {
        self.tab_selected.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn workbook_view_id(&self) -> u32 {
        self.workbook_view_id.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use workbook_view_id()")]
    pub fn get_workbook_view_id(&self) -> u32 {
        self.workbook_view_id()
    }

    #[inline]
    pub fn set_workbook_view_id(&mut self, value: u32) -> &mut Self {
        self.workbook_view_id.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn pane(&self) -> Option<&Pane> {
        self.pane.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use pane()")]
    pub fn get_pane(&self) -> Option<&Pane> {
        self.pane()
    }

    #[inline]
    pub fn pane_mut(&mut self) -> Option<&mut Pane> {
        self.pane.as_deref_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use pane_mut()")]
    pub fn get_pane_mut(&mut self) -> Option<&mut Pane> {
        self.pane_mut()
    }

    #[inline]
    pub fn set_pane(&mut self, value: Pane) -> &mut Self {
        self.pane = Some(Box::new(value));
        self
    }

    #[inline]
    #[must_use]
    pub fn view(&self) -> &SheetViewValues {
        self.view.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use view()")]
    pub fn get_view(&self) -> &SheetViewValues {
        self.view()
    }

    #[inline]
    pub fn set_view(&mut self, value: SheetViewValues) -> &mut Self {
        self.view.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn zoom_scale(&self) -> u32 {
        self.zoom_scale.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use zoom_scale()")]
    pub fn get_zoom_scale(&self) -> u32 {
        self.zoom_scale()
    }

    #[inline]
    pub fn set_zoom_scale(&mut self, value: u32) -> &mut Self {
        self.zoom_scale.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn zoom_scale_normal(&self) -> u32 {
        self.zoom_scale_normal.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use zoom_scale_normal()")]
    pub fn get_zoom_scale_normal(&self) -> u32 {
        self.zoom_scale_normal()
    }

    #[inline]
    pub fn set_zoom_scale_normal(&mut self, value: u32) -> &mut Self {
        self.zoom_scale_normal.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn zoom_scale_page_layout_view(&self) -> u32 {
        self.zoom_scale_page_layout_view.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use zoom_scale_page_layout_view()")]
    pub fn get_zoom_scale_page_layout_view(&self) -> u32 {
        self.zoom_scale_page_layout_view()
    }

    #[inline]
    pub fn set_zoom_scale_page_layout_view(&mut self, value: u32) -> &mut Self {
        self.zoom_scale_page_layout_view.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn zoom_scale_sheet_layout_view(&self) -> u32 {
        self.zoom_scale_sheet_layout_view.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use zoom_scale_sheet_layout_view()")]
    pub fn get_zoom_scale_sheet_layout_view(&self) -> u32 {
        self.zoom_scale_sheet_layout_view()
    }

    #[inline]
    pub fn set_zoom_scale_sheet_layout_view(&mut self, value: u32) -> &mut Self {
        self.zoom_scale_sheet_layout_view.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn top_left_cell(&self) -> &str {
        self.top_left_cell.value_str()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use top_left_cell()")]
    pub fn get_top_left_cell(&self) -> &str {
        self.top_left_cell()
    }

    #[inline]
    pub fn set_top_left_cell<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.top_left_cell.set_value(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn selection(&self) -> &[Selection] {
        &self.selection
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use selection()")]
    pub fn get_selection(&self) -> &[Selection] {
        self.selection()
    }

    #[inline]
    pub fn selection_mut(&mut self) -> &mut Vec<Selection> {
        &mut self.selection
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use selection_mut()")]
    pub fn get_selection_mut(&mut self) -> &mut Vec<Selection> {
        self.selection_mut()
    }

    #[inline]
    pub fn set_selection(&mut self, value: Selection) -> &mut Self {
        self.selection.push(value);
        self
    }

    /// Whether the panes of the window are locked (`windowProtection`).
    #[inline]
    #[must_use]
    pub fn window_protection(&self) -> bool {
        self.window_protection.value()
    }

    #[inline]
    pub fn set_window_protection(&mut self, value: bool) -> &mut Self {
        self.window_protection.set_value(value);
        self
    }

    /// Whether formulas are shown instead of their results (`showFormulas`).
    #[inline]
    #[must_use]
    pub fn show_formulas(&self) -> bool {
        self.show_formulas.value()
    }

    #[inline]
    pub fn set_show_formulas(&mut self, value: bool) -> &mut Self {
        self.show_formulas.set_value(value);
        self
    }

    /// Whether the row and column headers are shown (`showRowColHeaders`).
    /// Defaults to `true`.
    #[inline]
    #[must_use]
    pub fn show_row_col_headers(&self) -> bool {
        !self.show_row_col_headers.has_value() || self.show_row_col_headers.value()
    }

    #[inline]
    pub fn set_show_row_col_headers(&mut self, value: bool) -> &mut Self {
        self.show_row_col_headers.set_value(value);
        self
    }

    /// Whether zero values are shown (`showZeros`). Defaults to `true`.
    #[inline]
    #[must_use]
    pub fn show_zeros(&self) -> bool {
        !self.show_zeros.has_value() || self.show_zeros.value()
    }

    #[inline]
    pub fn set_show_zeros(&mut self, value: bool) -> &mut Self {
        self.show_zeros.set_value(value);
        self
    }

    /// Whether the sheet is shown from right to left (`rightToLeft`).
    #[inline]
    #[must_use]
    pub fn right_to_left(&self) -> bool {
        self.right_to_left.value()
    }

    #[inline]
    pub fn set_right_to_left(&mut self, value: bool) -> &mut Self {
        self.right_to_left.set_value(value);
        self
    }

    /// Whether the ruler is shown in page layout view (`showRuler`). Defaults
    /// to `true`.
    #[inline]
    #[must_use]
    pub fn show_ruler(&self) -> bool {
        !self.show_ruler.has_value() || self.show_ruler.value()
    }

    #[inline]
    pub fn set_show_ruler(&mut self, value: bool) -> &mut Self {
        self.show_ruler.set_value(value);
        self
    }

    /// Whether the outline symbols are shown (`showOutlineSymbols`). Defaults
    /// to `true`.
    #[inline]
    #[must_use]
    pub fn show_outline_symbols(&self) -> bool {
        !self.show_outline_symbols.has_value() || self.show_outline_symbols.value()
    }

    #[inline]
    pub fn set_show_outline_symbols(&mut self, value: bool) -> &mut Self {
        self.show_outline_symbols.set_value(value);
        self
    }

    /// Whether the default grid line color is used (`defaultGridColor`).
    /// Defaults to `true`.
    #[inline]
    #[must_use]
    pub fn default_grid_color(&self) -> bool {
        !self.default_grid_color.has_value() || self.default_grid_color.value()
    }

    #[inline]
    pub fn set_default_grid_color(&mut self, value: bool) -> &mut Self {
        self.default_grid_color.set_value(value);
        self
    }

    /// Whether the margins are shown in page layout view (`showWhiteSpace`).
    /// Defaults to `true`.
    #[inline]
    #[must_use]
    pub fn show_white_space(&self) -> bool {
        !self.show_white_space.has_value() || self.show_white_space.value()
    }

    #[inline]
    pub fn set_show_white_space(&mut self, value: bool) -> &mut Self {
        self.show_white_space.set_value(value);
        self
    }

    /// Index of the grid line color (`colorId`). Only used when
    /// `default_grid_color()` is `false`. Defaults to `64`.
    #[inline]
    #[must_use]
    pub fn color_id(&self) -> u32 {
        if self.color_id.has_value() {
            self.color_id.value()
        } else {
            64
        }
    }

    #[inline]
    pub fn set_color_id(&mut self, value: u32) -> &mut Self {
        self.color_id.set_value(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, window_protection, "windowProtection");
        set_string_from_xml!(self, e, show_formulas, "showFormulas");
        set_string_from_xml!(self, e, show_grid_lines, "showGridLines");
        set_string_from_xml!(self, e, show_row_col_headers, "showRowColHeaders");
        set_string_from_xml!(self, e, show_zeros, "showZeros");
        set_string_from_xml!(self, e, right_to_left, "rightToLeft");
        set_string_from_xml!(self, e, tab_selected, "tabSelected");
        set_string_from_xml!(self, e, show_ruler, "showRuler");
        set_string_from_xml!(self, e, show_outline_symbols, "showOutlineSymbols");
        set_string_from_xml!(self, e, default_grid_color, "defaultGridColor");
        set_string_from_xml!(self, e, show_white_space, "showWhiteSpace");
        set_string_from_xml!(self, e, workbook_view_id, "workbookViewId");
        set_string_from_xml!(self, e, view, "view");
        set_string_from_xml!(self, e, zoom_scale, "zoomScale");
        set_string_from_xml!(self, e, zoom_scale_normal, "zoomScaleNormal");
        set_string_from_xml!(
            self,
            e,
            zoom_scale_page_layout_view,
            "zoomScalePageLayoutView"
        );
        set_string_from_xml!(
            self,
            e,
            zoom_scale_sheet_layout_view,
            "zoomScaleSheetLayoutView"
        );
        set_string_from_xml!(self, e, top_left_cell, "topLeftCell");
        set_string_from_xml!(self, e, color_id, "colorId");

        if empty_flag {
            return;
        }

        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                match e.name().into_inner() {
                    b"pane" => {
                        let mut obj = Pane::default();
                        obj.set_attributes(reader, e);
                        self.set_pane(obj);
                    }
                    b"selection" => {
                        let mut obj = Selection::default();
                        obj.set_attributes(reader, e);
                        self.set_selection(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"sheetView" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "sheetView")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let empty_flag = self.pane.is_none() && self.selection.is_empty();

        // sheetView
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        if self.window_protection.has_value() {
            attributes.push(("windowProtection", self.window_protection.value_string()).into());
        }
        if self.show_formulas.has_value() {
            attributes.push(("showFormulas", self.show_formulas.value_string()).into());
        }
        if self.show_grid_lines.has_value() {
            attributes.push(("showGridLines", self.show_grid_lines.value_string()).into());
        }
        let show_row_col_headers = self.show_row_col_headers.value_string();
        if self.show_row_col_headers.has_value() {
            attributes.push(("showRowColHeaders", show_row_col_headers).into());
        }
        if self.show_zeros.has_value() {
            attributes.push(("showZeros", self.show_zeros.value_string()).into());
        }
        if self.right_to_left.has_value() {
            attributes.push(("rightToLeft", self.right_to_left.value_string()).into());
        }
        if self.tab_selected.value() {
            attributes.push(("tabSelected", self.tab_selected.value_string()).into());
        }
        if self.show_ruler.has_value() {
            attributes.push(("showRuler", self.show_ruler.value_string()).into());
        }
        let show_outline_symbols = self.show_outline_symbols.value_string();
        if self.show_outline_symbols.has_value() {
            attributes.push(("showOutlineSymbols", show_outline_symbols).into());
        }
        if self.default_grid_color.has_value() {
            attributes.push(("defaultGridColor", self.default_grid_color.value_string()).into());
        }
        if self.show_white_space.has_value() {
            attributes.push(("showWhiteSpace", self.show_white_space.value_string()).into());
        }
        if self.view.has_value() {
            attributes.push(("view", self.view.value_string()).into());
        }
        let zoom_scale = self.zoom_scale.value_string();
        if self.zoom_scale.has_value() {
            attributes.push(("zoomScale", &zoom_scale).into());
        }
        let zoom_scale_normal = self.zoom_scale_normal.value_string();
        if self.zoom_scale_normal.has_value() {
            attributes.push(("zoomScaleNormal", &zoom_scale_normal).into());
        }
        let zoom_scale_page_layout_view = self.zoom_scale_page_layout_view.value_string();
        if self.zoom_scale_page_layout_view.has_value() {
            attributes.push(("zoomScalePageLayoutView", &zoom_scale_page_layout_view).into());
        }
        let zoom_scale_sheet_layout_view = self.zoom_scale_sheet_layout_view.value_string();
        if self.zoom_scale_sheet_layout_view.has_value() {
            attributes.push(("zoomScaleSheetLayoutView", &zoom_scale_sheet_layout_view).into());
        }
        let top_left_cell = self.top_left_cell.value_str();
        if self.top_left_cell.has_value() {
            attributes.push(("topLeftCell", top_left_cell).into());
        }
        let color_id = self.color_id.value_string();
        if self.color_id.has_value() {
            attributes.push(("colorId", &color_id).into());
        }
        let workbook_view_id = self.workbook_view_id.value_string();
        attributes.push(("workbookViewId", &workbook_view_id).into());

        write_start_tag(writer, "sheetView", attributes, empty_flag);

        if empty_flag {
            return;
        }
        // pane
        if let Some(v) = &self.pane {
            v.write_to(writer);
        }
        // selection
        for obj in &self.selection {
            obj.write_to(writer);
        }
        write_end_tag(writer, "sheetView");
    }
}
