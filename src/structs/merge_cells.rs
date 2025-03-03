// mergeCell
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::Range;
use crate::{
    reader::driver::{
        get_attribute,
        xml_read_loop,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub(crate) struct MergeCells {
    range: Vec<Range>,
}

impl MergeCells {
    #[inline]
    pub(crate) fn range_collection(&self) -> &[Range] {
        &self.range
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use range_collection()")]
    pub(crate) fn get_range_collection(&self) -> &[Range] {
        self.range_collection()
    }

    #[inline]
    pub(crate) fn range_collection_mut(&mut self) -> &mut Vec<Range> {
        &mut self.range
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use range_collection_mut()")]
    pub(crate) fn get_range_collection_mut(&mut self) -> &mut Vec<Range> {
        self.range_collection_mut()
    }

    #[inline]
    pub(crate) fn add_range<S: Into<String>>(&mut self, range: S) -> &mut Self {
        let mut obj = Range::default();
        obj.set_range(range);
        self.range.push(obj);
        self
    }

    pub(crate) fn has_vertical(&self, row_num: u32) -> bool {
        self.range_collection().iter().any(|range| {
            let start_num = match range.coordinate_start_row() {
                Some(v) => v.num() <= row_num,
                None => true,
            };

            let end_num = match range.coordinate_end_row() {
                Some(v) => v.num() >= row_num,
                None => true,
            };

            start_num && end_num && start_num != end_num
        })
    }

    pub(crate) fn has_horizontal(&self, col_num: u32) -> bool {
        self.range_collection().iter().any(|range| {
            let start_num = match range.coordinate_start_col() {
                Some(v) => v.num() <= col_num,
                None => true,
            };

            let end_num = match range.coordinate_end_col() {
                Some(v) => v.num() >= col_num,
                None => true,
            };

            start_num && end_num && start_num != end_num
        })
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"mergeCell" {
                    self.add_range(get_attribute(e, b"ref").unwrap());
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"mergeCells" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "mergeCells")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.range_collection().is_empty() {
            // mergeCells
            write_start_tag(
                writer,
                "mergeCells",
                vec![
                    (
                        "count",
                        self.range_collection().len().to_string().as_str(),
                    )
                        .into(),
                ],
                false,
            );

            // mergeCell
            for merge_cell in self.range_collection() {
                write_start_tag(
                    writer,
                    "mergeCell",
                    vec![("ref", merge_cell.range().as_str()).into()],
                    true,
                );
            }

            write_end_tag(writer, "mergeCells");
        }
    }
}
