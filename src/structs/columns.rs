// fills
use std::io::Cursor;

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
    structs::{
        Cells,
        Column,
        MergeCells,
        Stylesheet,
    },
    traits::AdjustmentValue,
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
};

#[derive(Clone, Default, Debug)]
pub(crate) struct Columns {
    column: Vec<Column>,
}

impl Columns {
    #[inline]
    pub(crate) fn get_column_collection(&self) -> &[Column] {
        &self.column
    }

    #[inline]
    pub(crate) fn get_column_collection_mut(&mut self) -> &mut Vec<Column> {
        &mut self.column
    }

    #[inline]
    pub(crate) fn get_column(&self, value: u32) -> Option<&Column> {
        self.column
            .iter()
            .find(|&column| value == column.get_col_num())
    }

    pub(crate) fn get_column_mut(&mut self, value: u32) -> &mut Column {
        if self.get_column(value).is_none() {
            let mut obj = Column::default();
            obj.set_col_num(value);
            self.set_column(obj);
        }
        for column in self.get_column_collection_mut() {
            if value == column.get_col_num() {
                return column;
            }
        }
        panic!("Column not found.");
    }

    #[inline]
    pub(crate) fn set_column(&mut self, value: Column) -> &mut Self {
        self.column.push(value);
        self
    }

    pub(crate) fn calculation_auto_width(
        &mut self,
        cells: &Cells,
        merge_cells: &MergeCells,
    ) -> &mut Self {
        for column in self.get_column_collection_mut() {
            let has_horizontal = merge_cells.has_horizontal(column.get_col_num());
            if has_horizontal {
                continue;
            }
            column.calculation_auto_width(cells);
        }
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
        stylesheet: &Stylesheet,
    ) {
        xml_read_loop!(
            reader,
            Event::Empty(ref e) => {
                if e.name().into_inner() == b"col" {
                    let mut obj = Column::default();
                    obj.set_attributes(reader, e, stylesheet);
                    let min = get_attribute(e, b"min").unwrap().parse::<u32>().unwrap();
                    let max = get_attribute(e, b"max").unwrap().parse::<u32>().unwrap();
                    for i in min..=max {
                        obj.set_col_num(i);
                        self.set_column(obj.clone());
                    }
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"cols" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "cols")
        );
    }

    pub(crate) fn write_to(
        &self,
        writer: &mut Writer<Cursor<Vec<u8>>>,
        stylesheet: &mut Stylesheet,
    ) {
        if self.column.is_empty() {
            return;
        }

        // cols
        write_start_tag(writer, "cols", vec![], false);

        // col

        let mut column_copy = self.column.clone();
        column_copy.sort_by_key(Column::get_col_num);
        let mut column_iter = column_copy.iter();
        let mut column_raw = column_iter.next();
        let mut obj = column_raw.unwrap();
        let mut min = obj.get_col_num();
        let mut max = min;

        loop {
            column_raw = column_iter.next();
            if let Some(column) = column_raw {
                if column.get_col_num() == max + 1
                    && column.get_hash_code() == obj.get_hash_code()
                    && column.get_style() == obj.get_style()
                {
                    max += 1;
                } else {
                    Self::write_to_column(writer, min, max, obj, stylesheet);
                    obj = column;
                    min = obj.get_col_num();
                    max = min;
                }
            } else {
                Self::write_to_column(writer, min, max, obj, stylesheet);
                break;
            }
        }

        write_end_tag(writer, "cols");
    }

    pub(crate) fn write_to_column(
        writer: &mut Writer<Cursor<Vec<u8>>>,
        min: u32,
        max: u32,
        column: &Column,
        stylesheet: &mut Stylesheet,
    ) {
        // col
        let mut attributes: crate::structs::AttrCollection = Vec::new();
        let min_str = min.to_string();
        let max_str = max.to_string();
        attributes.push(("min", min_str).into());
        attributes.push(("max", max_str).into());
        let width = column.width.get_value_string();
        attributes.push(("width", &width).into());
        if column.hidden.value() {
            attributes.push(("hidden", column.hidden.value_string()).into());
        }
        if column.best_fit.value() {
            attributes.push(("bestFit", column.best_fit.value_string()).into());
        }
        attributes.push(("customWidth", "1").into());
        let xf_index_str: String;
        let xf_index = stylesheet.set_style(column.get_style());
        if xf_index > 0 {
            xf_index_str = xf_index.to_string();
            attributes.push(("style", &xf_index_str).into());
        }
        write_start_tag(writer, "col", attributes, true);
    }
}
impl AdjustmentValue for Columns {
    fn adjustment_insert_value(&mut self, root_num: u32, offset_num: u32) {
        for column_dimension in &mut self.column {
            column_dimension.adjustment_insert_value(root_num, offset_num);
        }
    }

    fn adjustment_remove_value(&mut self, root_num: u32, offset_num: u32) {
        self.get_column_collection_mut()
            .retain(|x| !(x.is_remove_value(root_num, offset_num)));
        for column_dimension in &mut self.column {
            column_dimension.adjustment_remove_value(root_num, offset_num);
        }
    }
}
