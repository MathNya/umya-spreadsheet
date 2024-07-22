// sheetView
use super::BooleanValue;
use super::EnumValue;
use super::Pane;
use super::Selection;
use super::SheetViewValues;
use super::StringValue;
use super::UInt32Value;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct SheetView {
    tab_selected: BooleanValue,
    workbook_view_id: UInt32Value,
    pane: Option<Pane>,
    view: EnumValue<SheetViewValues>,
    zoom_scale: UInt32Value,
    zoom_scale_normal: UInt32Value,
    zoom_scale_page_layout_view: UInt32Value,
    zoom_scale_sheet_layout_view: UInt32Value,
    top_left_cell: StringValue,
    selection: Vec<Selection>,
}

impl SheetView {
    pub fn get_tab_selected(&self) -> &bool {
        self.tab_selected.get_value()
    }

    pub fn set_tab_selected(&mut self, value: bool) -> &mut Self {
        self.tab_selected.set_value(value);
        self
    }

    pub fn get_workbook_view_id(&self) -> &u32 {
        self.workbook_view_id.get_value()
    }

    pub fn set_workbook_view_id(&mut self, value: u32) -> &mut Self {
        self.workbook_view_id.set_value(value);
        self
    }

    pub fn get_pane(&self) -> Option<&Pane> {
        self.pane.as_ref()
    }

    pub fn get_pane_mut(&mut self) -> Option<&mut Pane> {
        self.pane.as_mut()
    }

    pub fn set_pane(&mut self, value: Pane) -> &mut Self {
        self.pane = Some(value);
        self
    }

    pub fn get_view(&self) -> &SheetViewValues {
        self.view.get_value()
    }

    pub fn set_view(&mut self, value: SheetViewValues) -> &mut Self {
        self.view.set_value(value);
        self
    }

    pub fn get_zoom_scale(&self) -> &u32 {
        self.zoom_scale.get_value()
    }

    pub fn set_zoom_scale(&mut self, value: u32) -> &mut Self {
        self.zoom_scale.set_value(value);
        self
    }

    pub fn get_zoom_scale_normal(&self) -> &u32 {
        self.zoom_scale_normal.get_value()
    }

    pub fn set_zoom_scale_normal(&mut self, value: u32) -> &mut Self {
        self.zoom_scale_normal.set_value(value);
        self
    }

    pub fn get_zoom_scale_page_layout_view(&self) -> &u32 {
        self.zoom_scale_page_layout_view.get_value()
    }

    pub fn set_zoom_scale_page_layout_view(&mut self, value: u32) -> &mut Self {
        self.zoom_scale_page_layout_view.set_value(value);
        self
    }

    pub fn get_zoom_scale_sheet_layout_view(&self) -> &u32 {
        self.zoom_scale_sheet_layout_view.get_value()
    }

    pub fn set_zoom_scale_sheet_layout_view(&mut self, value: u32) -> &mut Self {
        self.zoom_scale_sheet_layout_view.set_value(value);
        self
    }

    pub fn get_top_left_cell(&self) -> &str {
        self.top_left_cell.get_value_str()
    }

    pub fn set_top_left_cell<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.top_left_cell.set_value(value);
        self
    }

    pub fn get_selection(&self) -> &Vec<Selection> {
        &self.selection
    }

    pub fn get_selection_mut(&mut self) -> &mut Vec<Selection> {
        &mut self.selection
    }

    pub fn set_selection(&mut self, value: Selection) -> &mut Self {
        self.selection.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flag: bool,
    ) {
        set_string_from_xml!(self, e, tab_selected, "tabSelected");
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
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if *self.tab_selected.get_value() {
            attributes.push(("tabSelected", self.tab_selected.get_value_string()));
        }
        if self.view.has_value() {
            attributes.push(("view", self.view.get_value_string()));
        }
        let zoom_scale = self.zoom_scale.get_value_string();
        if self.zoom_scale.has_value() {
            attributes.push(("zoomScale", &zoom_scale));
        }
        let zoom_scale_normal = self.zoom_scale_normal.get_value_string();
        if self.zoom_scale_normal.has_value() {
            attributes.push(("zoomScaleNormal", &zoom_scale_normal));
        }
        let zoom_scale_page_layout_view = self.zoom_scale_page_layout_view.get_value_string();
        if self.zoom_scale_page_layout_view.has_value() {
            attributes.push(("zoomScalePageLayoutView", &zoom_scale_page_layout_view));
        }
        let zoom_scale_sheet_layout_view = self.zoom_scale_sheet_layout_view.get_value_string();
        if self.zoom_scale_sheet_layout_view.has_value() {
            attributes.push(("zoomScaleSheetLayoutView", &zoom_scale_sheet_layout_view));
        }
        let top_left_cell = self.top_left_cell.get_value_str();
        if self.top_left_cell.has_value() {
            attributes.push(("topLeftCell", &top_left_cell));
        }
        let workbook_view_id = self.workbook_view_id.get_value_string();
        attributes.push(("workbookViewId", &workbook_view_id));

        write_start_tag(writer, "sheetView", attributes, empty_flag);

        if empty_flag {
            return;
        }
        // pane
        if let Some(v) = &self.pane {
            v.write_to(writer)
        }
        // selection
        for obj in &self.selection {
            obj.write_to(writer);
        }
        write_end_tag(writer, "sheetView");
    }
}
