// autoFilter
use std::io::Cursor;
use super::Range;
use crate::{FilterColumn, traits::AdjustmentCoordinate};
use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};
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
pub struct AutoFilter {
    range: Range,
    filter_columns: Vec<FilterColumn>,
}

impl AutoFilter {
    #[inline]
    #[must_use]
    pub fn range(&self) -> &Range {
        &self.range
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use range()")]
    pub fn get_range(&self) -> &Range {
        self.range()
    }

    #[inline]
    pub fn range_mut(&mut self) -> &mut Range {
        &mut self.range
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use range_mut()")]
    pub fn get_range_mut(&mut self) -> &mut Range {
        self.range_mut()
    }

    #[inline]
    pub fn set_range<S: Into<String>>(&mut self, value: S) {
        let mut range = Range::default();
        range.set_range(value.into());
        self.range = range;
    }

    #[inline]
    #[must_use]
    pub fn filter_columns(&self) -> &[FilterColumn] {
        &self.filter_columns
    }

    #[inline]
    pub fn filter_columns_mut(&mut self) -> &mut Vec<FilterColumn> {
        &mut self.filter_columns
    }

    #[inline]
    pub fn add_filter_column(&mut self, value: FilterColumn) -> &mut Self {
        self.filter_columns.push(value);
        self
    }

    #[inline]
    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flg: bool,
    ) {
        if let Some(v) = get_attribute(e, b"ref") {
            self.range.set_range(v);
        }
        
        if empty_flg {
            return;
        }

        xml_read_loop!(
            reader,
            ref n @ (Event::Empty(ref e) | Event::Start(ref e)) => {
                let _is_empty = matches!(n, Event::Empty(_));
                if e.name().into_inner() == b"filterColumn" {
                    let mut obj = FilterColumn::default();
                    obj.set_attributes(reader, e);
                    self.add_filter_column(obj);
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"autoFilter" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "autoFilter")
        );
    }

    #[inline]
    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        let empty_flg = self.filter_columns.is_empty();

        // autoFilter
        let attributes: crate::structs::AttrCollection = vec![
            ("ref", self.range.range()).into(),
        ];
        write_start_tag(writer, "autoFilter", attributes, empty_flg);
        if empty_flg {
            return;
        }

        for obj in &self.filter_columns {
            obj.write_to(writer);
        }

        write_end_tag(writer, "autoFilter");
    }

}
impl AdjustmentCoordinate for AutoFilter {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.range.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.range.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }
}
