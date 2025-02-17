use super::{
    ColumnReference,
    RowReference,
};
use crate::{
    helper::coordinate::index_from_coordinate,
    traits::{
        AdjustmentCoordinate,
        AdjustmentValue,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Range {
    start_col: Option<ColumnReference>,
    start_row: Option<RowReference>,
    end_col:   Option<ColumnReference>,
    end_row:   Option<RowReference>,
}

impl Range {
    pub fn set_range<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let org_value = value.into();
        let coordinate_collection: Vec<&str> = org_value.split(':').collect();

        assert!(
            matches!(coordinate_collection.len(), 1 | 2),
            "Non-standard coordinate"
        );

        let (
            row,         //
            col,         //
            is_lock_col, //
            is_lock_row,
        ) = index_from_coordinate(coordinate_collection[0]);
        if let Some(v) = row {
            let mut coordinate_start_col = ColumnReference::default();
            coordinate_start_col.set_num(v);
            coordinate_start_col.set_is_lock(is_lock_col.unwrap());
            self.start_col = Some(coordinate_start_col);
        }
        if let Some(v) = col {
            let mut coordinate_start_row = RowReference::default();
            coordinate_start_row.set_num(v);
            coordinate_start_row.set_is_lock(is_lock_row.unwrap());
            self.start_row = Some(coordinate_start_row);
        }

        if coordinate_collection.len() == 2 {
            let (
                row,         //
                col,         //
                is_lock_col, //
                is_lock_row,
            ) = index_from_coordinate(coordinate_collection[1]);
            if let Some(v) = row {
                let mut coordinate_end_col = ColumnReference::default();
                coordinate_end_col.set_num(v);
                coordinate_end_col.set_is_lock(is_lock_col.unwrap());
                self.end_col = Some(coordinate_end_col);
            }
            if let Some(v) = col {
                let mut coordinate_end_row = RowReference::default();
                coordinate_end_row.set_num(v);
                coordinate_end_row.set_is_lock(is_lock_row.unwrap());
                self.end_row = Some(coordinate_end_row);
            }
        }
        self
    }

    #[inline]
    #[must_use]
    pub fn range(&self) -> String {
        let mut result = self.coordinate_start();
        if self.end_col.is_some() || self.end_row.is_some() {
            result = format!("{}:{}", result, &self.coordinate_end());
        }
        result
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use range()")]
    pub fn get_range(&self) -> String {
        self.range()
    }

    #[inline]
    #[must_use]
    pub fn coordinate_start_col(&self) -> Option<&ColumnReference> {
        self.start_col.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use coordinate_start_col()")]
    pub fn get_coordinate_start_col(&self) -> Option<&ColumnReference> {
        self.coordinate_start_col()
    }

    #[inline]
    pub fn coordinate_start_col_mut(&mut self) -> Option<&mut ColumnReference> {
        self.start_col.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use coordinate_start_col_mut()")]
    pub fn get_coordinate_start_col_mut(&mut self) -> Option<&mut ColumnReference> {
        self.coordinate_start_col_mut()
    }

    #[inline]
    #[must_use]
    pub fn coordinate_start_row(&self) -> Option<&RowReference> {
        self.start_row.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use coordinate_start_row()")]
    pub fn get_coordinate_start_row(&self) -> Option<&RowReference> {
        self.coordinate_start_row()
    }

    #[inline]
    pub fn coordinate_start_row_mut(&mut self) -> Option<&mut RowReference> {
        self.start_row.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use coordinate_start_row_mut()")]
    pub fn get_coordinate_start_row_mut(&mut self) -> Option<&mut RowReference> {
        self.coordinate_start_row_mut()
    }

    #[inline]
    #[must_use]
    pub fn coordinate_end_col(&self) -> Option<&ColumnReference> {
        self.end_col.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use coordinate_end_col()")]
    pub fn get_coordinate_end_col(&self) -> Option<&ColumnReference> {
        self.coordinate_end_col()
    }

    #[inline]
    pub fn coordinate_end_col_mut(&mut self) -> Option<&mut ColumnReference> {
        self.end_col.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use coordinate_end_col_mut()")]
    pub fn get_coordinate_end_col_mut(&mut self) -> Option<&mut ColumnReference> {
        self.coordinate_end_col_mut()
    }

    #[inline]
    #[must_use]
    pub fn coordinate_end_row(&self) -> Option<&RowReference> {
        self.end_row.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use coordinate_end_row()")]
    pub fn get_coordinate_end_row(&self) -> Option<&RowReference> {
        self.coordinate_end_row()
    }

    #[inline]
    pub fn coordinate_end_row_mut(&mut self) -> Option<&mut RowReference> {
        self.end_row.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use coordinate_end_row_mut()")]
    pub fn get_coordinate_end_row_mut(&mut self) -> Option<&mut RowReference> {
        self.coordinate_end_row_mut()
    }

    #[inline]
    pub(crate) fn coordinate_start(&self) -> String {
        let mut coordinate_str = String::new();
        if let Some(v) = &self.start_col {
            coordinate_str = v.coordinate();
        }
        if let Some(v) = &self.start_row {
            coordinate_str = format!("{}{}", coordinate_str, v.coordinate());
        }
        coordinate_str
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use sheet_name()")]
    pub(crate) fn get_coordinate_start(&self) -> String {
        self.coordinate_start()
    }

    #[inline]
    pub(crate) fn coordinate_end(&self) -> String {
        let mut coordinate_str = String::new();
        if let Some(v) = &self.end_col {
            coordinate_str = v.coordinate();
        }
        if let Some(v) = &self.end_row {
            coordinate_str = format!("{}{}", coordinate_str, v.coordinate());
        }
        coordinate_str
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use coordinate_end()")]
    pub(crate) fn get_coordinate_end(&self) -> String {
        self.coordinate_end()
    }
}
impl AdjustmentCoordinate for Range {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        if let Some(v) = &mut self.start_col {
            v.adjustment_insert_value(root_col_num, offset_col_num);
        }
        if let Some(v) = &mut self.start_row {
            v.adjustment_insert_value(root_row_num, offset_row_num);
        }
        if let Some(v) = &mut self.end_col {
            v.adjustment_insert_value(root_col_num, offset_col_num);
        }
        if let Some(v) = &mut self.end_row {
            v.adjustment_insert_value(root_row_num, offset_row_num);
        }
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        if let Some(v) = &mut self.start_col {
            v.adjustment_remove_value(root_col_num, offset_col_num);
        }
        if let Some(v) = &mut self.start_row {
            v.adjustment_remove_value(root_row_num, offset_row_num);
        }
        if let Some(v) = &mut self.end_col {
            v.adjustment_remove_value(root_col_num, offset_col_num);
        }
        if let Some(v) = &mut self.end_row {
            v.adjustment_remove_value(root_row_num, offset_row_num);
        }
    }

    #[inline]
    fn is_remove_coordinate(
        &self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) -> bool {
        let start_col_result = match &self.start_col {
            Some(v) => v.is_remove_value(root_col_num, offset_col_num),
            None => false,
        };
        let start_row_result = match &self.start_row {
            Some(v) => v.is_remove_value(root_row_num, offset_row_num),
            None => false,
        };
        let end_col_result = match &self.end_col {
            Some(v) => v.is_remove_value(root_col_num, offset_col_num),
            None => false,
        };
        let end_row_result = match &self.end_row {
            Some(v) => v.is_remove_value(root_row_num, offset_row_num),
            None => false,
        };
        start_col_result && start_row_result && end_col_result && end_row_result
    }
}
