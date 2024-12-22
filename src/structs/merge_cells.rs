// mergeCell
use std::io::Cursor;

use quick_xml::Reader;
use quick_xml::Writer;
use quick_xml::events::{BytesStart, Event};

use super::Range;
use crate::reader::driver::{get_attribute, xml_read_loop};
use crate::writer::driver::{write_end_tag, write_start_tag};

#[derive(Clone, Default, Debug)]
pub(crate) struct MergeCells {
    range: Vec<Range>,
}

impl MergeCells {
    #[inline]
    pub(crate) fn get_range_collection(&self) -> &[Range] {
        &self.range
    }

    #[inline]
    pub(crate) fn get_range_collection_mut(&mut self) -> &mut Vec<Range> {
        &mut self.range
    }

    #[inline]
    pub(crate) fn add_range<S: Into<String>>(&mut self, range: S) -> &mut Self {
        let mut obj = Range::default();
        obj.set_range(range);
        self.range.push(obj);
        self
    }

    pub(crate) fn has_vertical(&self, row_num: u32) -> bool {
        self.get_range_collection().iter().any(|range| {
            let start_num = match range.get_coordinate_start_row() {
                Some(v) => v.get_num() <= row_num,
                None => true,
            };

            let end_num = match range.get_coordinate_end_row() {
                Some(v) => v.get_num() >= row_num,
                None => true,
            };

            start_num && end_num && start_num != end_num
        })
    }

    pub(crate) fn has_horizontal(&self, col_num: u32) -> bool {
        self.get_range_collection().iter().any(|range| {
            let start_num = match range.get_coordinate_start_col() {
                Some(v) => v.get_num() <= col_num,
                None => true,
            };

            let end_num = match range.get_coordinate_end_col() {
                Some(v) => v.get_num() >= col_num,
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
        if !self.get_range_collection().is_empty() {
            // mergeCells
            write_start_tag(
                writer,
                "mergeCells",
                vec![("count", self.get_range_collection().len().to_string().as_str())],
                false,
            );

            // mergeCell
            for merge_cell in self.get_range_collection() {
                write_start_tag(
                    writer,
                    "mergeCell",
                    vec![("ref", merge_cell.get_range().as_str())],
                    true,
                );
            }

            write_end_tag(writer, "mergeCells");
        }
    }
}
