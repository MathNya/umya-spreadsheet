use std::collections::HashMap;

use crate::{
    structs::Row,
    traits::AdjustmentValue,
};

#[derive(Clone, Default, Debug)]
pub(crate) struct Rows {
    rows: HashMap<u32, Box<Row>>,
}
impl Rows {
    #[inline]
    pub(crate) fn has_sheet_data(&self) -> bool {
        !self.rows.is_empty()
    }

    /// Get Row Dimension List.
    #[inline]
    pub(crate) fn row_dimensions(&self) -> Vec<&Row> {
        self.rows.values().map(Box::as_ref).collect()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_dimensions()")]
    pub(crate) fn get_row_dimensions(&self) -> Vec<&Row> {
        self.row_dimensions()
    }

    /// Get Row Dimension List in mutable.
    #[inline]
    pub(crate) fn row_dimensions_mut(&mut self) -> Vec<&mut Row> {
        self.rows.values_mut().map(Box::as_mut).collect()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_dimensions_mut()")]
    pub(crate) fn get_row_dimensions_mut(&mut self) -> Vec<&mut Row> {
        self.row_dimensions_mut()
    }

    /// Get Row Dimension convert Hashmap.
    #[inline]
    pub(crate) fn row_dimensions_to_hashmap(&self) -> &HashMap<u32, Box<Row>> {
        &self.rows
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_dimensions_to_hashmap()")]
    pub(crate) fn get_row_dimensions_to_hashmap(&self) -> &HashMap<u32, Box<Row>> {
        self.row_dimensions_to_hashmap()
    }

    #[inline]
    pub(crate) fn row_dimensions_to_hashmap_mut(&mut self) -> &mut HashMap<u32, Box<Row>> {
        &mut self.rows
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_dimensions_to_hashmap_mut()")]
    pub(crate) fn get_row_dimensions_to_hashmap_mut(&mut self) -> &mut HashMap<u32, Box<Row>> {
        self.row_dimensions_to_hashmap_mut()
    }

    /// Get Row Dimension.
    #[inline]
    pub(crate) fn row_dimension(&self, row: u32) -> Option<&Row> {
        self.rows.get(&row).map(Box::as_ref)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_dimension()")]
    pub(crate) fn get_row_dimension(&self, row: u32) -> Option<&Row> {
        self.row_dimension(row)
    }

    /// Get Row Dimension in mutable.
    #[inline]
    pub(crate) fn row_dimension_mut(&mut self, row: u32) -> &mut Row {
        self.rows.entry(row).or_insert_with(|| {
            let mut obj = Row::default();
            obj.set_row_num(row);
            Box::new(obj)
        })
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_dimension_mut()")]
    pub(crate) fn get_row_dimension_mut(&mut self, row: u32) -> &mut Row {
        self.row_dimension_mut(row)
    }

    /// (This method is crate only.)
    /// Set Row Dimension.
    #[inline]
    pub(crate) fn set_row_dimension(&mut self, value: Row) -> &mut Self {
        let row = value.row_num();
        self.rows.insert(row, Box::new(value));
        self
    }

    #[inline]
    pub(crate) fn rebuild_map(&mut self) {
        self.rows = self
            .row_dimensions_to_hashmap_mut()
            .iter_mut()
            .map(|(_, row)| (row.row_num(), std::mem::take(row)))
            .collect();
    }
}
impl AdjustmentValue for Rows {
    fn adjustment_insert_value(&mut self, root_num: u32, offset_num: u32) {
        for row_dimension in self.row_dimensions_mut() {
            row_dimension.adjustment_insert_value(root_num, offset_num);
        }
        self.rebuild_map();
    }

    fn adjustment_remove_value(&mut self, root_num: u32, offset_num: u32) {
        self.row_dimensions_to_hashmap_mut()
            .retain(|_, k| !(k.is_remove_value(root_num, offset_num)));
        for row_dimension in self.row_dimensions_mut() {
            row_dimension.adjustment_remove_value(root_num, offset_num);
        }
        self.rebuild_map();
    }
}
