// sheetViews
use super::SheetView;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct SheetViews {
    sheet_view_list: Vec<SheetView>,
}
impl SheetViews {
    pub fn get_sheet_view_list(&self) -> &Vec<SheetView> {
        &self.sheet_view_list
    }

    pub fn get_sheet_view_list_mut(&mut self) -> &mut Vec<SheetView> {
        &mut self.sheet_view_list
    }

    pub fn add_sheet_view_list_mut(&mut self, value: SheetView) -> &mut Self {
        self.sheet_view_list.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref e)) => match e.name().into_inner() {
                    b"sheetView" => {
                        let mut obj = SheetView::default();
                        obj.set_attributes(reader, e, true);
                        self.add_sheet_view_list_mut(obj);
                    }
                    _ => (),
                },
                Ok(Event::Start(ref e)) => match e.name().into_inner() {
                    b"sheetView" => {
                        let mut obj = SheetView::default();
                        obj.set_attributes(reader, e, false);
                        self.add_sheet_view_list_mut(obj);
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"sheetViews" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "sheetViews"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // sheetViews
        if !self.sheet_view_list.is_empty() {
            write_start_tag(writer, "sheetViews", vec![], false);

            // sheetView
            for sheet_view in &self.sheet_view_list {
                sheet_view.write_to(writer);
            }

            write_end_tag(writer, "sheetViews");
        }
    }
}
