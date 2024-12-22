// sheetViews
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::SheetView;
use crate::{
    reader::driver::xml_read_loop,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub struct SheetViews {
    sheet_view_list: Vec<SheetView>,
}

impl SheetViews {
    #[inline]
    #[must_use]
    pub fn get_sheet_view_list(&self) -> &[SheetView] {
        &self.sheet_view_list
    }

    #[inline]
    pub fn get_sheet_view_list_mut(&mut self) -> &mut Vec<SheetView> {
        &mut self.sheet_view_list
    }

    #[inline]
    pub fn add_sheet_view_list_mut(&mut self, value: SheetView) -> &mut Self {
        self.sheet_view_list.push(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"sheetView" {
                    let mut obj = SheetView::default();
                    obj.set_attributes(reader, e, true);
                    self.add_sheet_view_list_mut(obj);
                }
            },
            Event::Start(ref e) => {
                if e.name().into_inner() == b"sheetView" {
                    let mut obj = SheetView::default();
                    obj.set_attributes(reader, e, false);
                    self.add_sheet_view_list_mut(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"sheetViews" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "sheetViews")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // sheetViews
        if self.sheet_view_list.is_empty() {
            return;
        }
        write_start_tag(writer, "sheetViews", vec![], false);

        // sheetView
        for sheet_view in &self.sheet_view_list {
            sheet_view.write_to(writer);
        }

        write_end_tag(writer, "sheetViews");
    }
}
