// sheetView
use super::BooleanValue;
use super::UInt32Value;
use super::Pane;
use super::Selection;
use reader::driver::*;
use writer::driver::*;
use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Default, Debug)]
pub struct SheetView {
    tab_selected: BooleanValue,
    workbook_view_id: UInt32Value,
    pane: Option<Pane>,
    selection: Vec<Selection>,
}
impl SheetView {
    pub fn get_tab_selected(&self) -> &bool {
        &self.tab_selected.get_value()
    }

    pub fn set_tab_selected(&mut self, value:bool) -> &mut Self {
        self.tab_selected.set_value(value);
        self
    }

    pub fn get_workbook_view_id(&self) -> &u32 {
        &self.workbook_view_id.get_value()
    }

    pub fn set_workbook_view_id(&mut self, value:u32) -> &mut Self {
        self.workbook_view_id.set_value(value);
        self
    }

    pub fn get_pane(&self)-> &Option<Pane> {
        &self.pane
    }

    pub fn get_pane_mut(&mut self)-> &mut Option<Pane> {
        &mut self.pane
    }

    pub fn set_pane(&mut self, value:Pane)-> &mut Self {
        self.pane = Some(value);
        self
    }

    pub fn get_selection(&self)-> &Vec<Selection> {
        &self.selection
    }

    pub fn get_selection_mut(&mut self)-> &mut Vec<Selection> {
        &mut self.selection
    }

    pub fn set_selection(&mut self, value:Selection)-> &mut Self {
        self.selection.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader:&mut Reader<R>,
        e:&BytesStart,
        empty_flag:bool,
    ) {
        match get_attribute(e, b"tabSelected") {
            Some(v) => {
                self.tab_selected.set_value_string(v);
            },
            None => {}
        }

        match get_attribute(e, b"workbookViewId") {
            Some(v) => {
                self.workbook_view_id.set_value_string(v);
            },
            None => {}
        }

        if empty_flag {
            return;
        }

        let mut buf = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Empty(ref e)) => {
                    match e.name() {
                        b"pane" => {
                            let mut obj = Pane::default();
                            obj.set_attributes(reader, e);
                            self.set_pane(obj);
                        },
                        b"selection" => {
                            let mut obj = Selection::default();
                            obj.set_attributes(reader, e);
                            self.set_selection(obj);
                        },
                        _ => (),
                    }
                },
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        b"sheetView" => return,
                        _ => (),
                    }
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "sheetView"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let empty_flag = self.pane.is_none() && self.selection.len() == 0;

        // sheetView
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        if self.tab_selected.get_value() == &true {
            attributes.push(("tabSelected", self.tab_selected.get_value_string()));
        }
        attributes.push(("workbookViewId", self.workbook_view_id.get_value_string()));

        write_start_tag(writer, "sheetView", attributes, empty_flag);

        if empty_flag == false {
            // pane
            match &self.pane {
                Some(v) => {v.write_to(writer)},
                None => {},
            }
            // selection
            for obj in &self.selection {
                obj.write_to(writer);
            }
            write_end_tag(writer, "sheetView");
        }
    }
}
