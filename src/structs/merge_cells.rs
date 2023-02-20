// mergeCell
use super::Range;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub(crate) struct MergeCells {
    range: Vec<Range>,
}
impl MergeCells {
    pub(crate) fn get_range_collection(&self) -> &Vec<Range> {
        &self.range
    }

    pub(crate) fn get_range_collection_mut(&mut self) -> &mut Vec<Range> {
        &mut self.range
    }

    pub(crate) fn add_range<S: Into<String>>(&mut self, range: S) -> &mut Self {
        let mut obj = Range::default();
        obj.set_range(range);
        self.range.push(obj);
        self
    }

    pub(crate) fn _has_vertical(&self, row_num: &u32) -> bool {
        for range in self.get_range_collection() {
            let start_num = match range.get_coordinate_start_row() {
                Some(v) => v.get_num(),
                None => {
                    return true;
                }
            };
            let end_num = match range.get_coordinate_end_row() {
                Some(v) => v.get_num(),
                None => {
                    return true;
                }
            };
            if start_num <= row_num && row_num <= end_num && start_num != end_num {
                return true;
            }
        }
        false
    }

    pub(crate) fn has_horizontal(&self, col_num: &u32) -> bool {
        for range in self.get_range_collection() {
            let start_num = match range.get_coordinate_start_col() {
                Some(v) => v.get_num(),
                None => {
                    return true;
                }
            };
            let end_num = match range.get_coordinate_end_col() {
                Some(v) => v.get_num(),
                None => {
                    return true;
                }
            };
            if start_num <= col_num && col_num <= end_num && start_num != end_num {
                return true;
            }
        }
        false
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
                    b"mergeCell" => {
                        self.add_range(get_attribute(e, b"ref").unwrap());
                    }
                    _ => (),
                },
                Ok(Event::End(ref e)) => match e.name().into_inner() {
                    b"mergeCells" => return,
                    _ => (),
                },
                Ok(Event::Eof) => panic!("Error not find {} end element", "mergeCells"),
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            buf.clear();
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        if !self.get_range_collection().is_empty() {
            // mergeCells
            write_start_tag(
                writer,
                "mergeCells",
                vec![(
                    "count",
                    self.get_range_collection().len().to_string().as_str(),
                )],
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
