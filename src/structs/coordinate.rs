use std::fmt;

use super::{
    ColumnReference,
    RowReference,
};
use crate::{
    helper::coordinate::{
        coordinate_from_index_with_lock,
        index_from_coordinate,
    },
    traits::{
        AdjustmentCoordinate,
        AdjustmentValue,
    },
};

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Coordinate {
    column: ColumnReference,
    row:    RowReference,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            coordinate_from_index_with_lock(
                self.column.get_num(),
                self.row.get_num(),
                self.column.get_is_lock(),
                self.row.get_is_lock(),
            )
        )
    }
}

impl Coordinate {
    #[must_use]
    pub fn get_col_num(&self) -> u32 {
        self.column.get_num()
    }

    pub fn set_col_num(&mut self, value: u32) -> &mut Self {
        self.column.set_num(value);
        self
    }

    pub(crate) fn offset_col_num(&mut self, value: i32) -> &mut Self {
        self.column.offset_num(value);
        self
    }

    #[must_use]
    pub fn get_row_num(&self) -> u32 {
        self.row.get_num()
    }

    pub fn set_row_num(&mut self, value: u32) -> &mut Self {
        self.row.set_num(value);
        self
    }

    pub(crate) fn offset_row_num(&mut self, value: i32) -> &mut Self {
        self.row.offset_num(value);
        self
    }

    #[must_use]
    pub fn get_is_lock_col(&self) -> bool {
        self.column.get_is_lock()
    }

    pub fn set_is_lock_col(&mut self, value: bool) -> &mut Self {
        self.column.set_is_lock(value);
        self
    }

    #[must_use]
    pub fn get_is_lock_row(&self) -> bool {
        self.row.get_is_lock()
    }

    pub fn set_is_lock_row(&mut self, value: bool) -> &mut Self {
        self.row.set_is_lock(value);
        self
    }

    /// Change coordinates
    /// Formula is not updated.
    pub fn set_coordinate<S: AsRef<str>>(&mut self, value: S) -> &mut Self {
        let (c, r, cl, rl) = index_from_coordinate(value.as_ref());

        self.column.set_num(c.unwrap());
        self.row.set_num(r.unwrap());
        self.column.set_is_lock(cl.unwrap());
        self.row.set_is_lock(rl.unwrap());

        self
    }

    #[must_use]
    pub fn get_coordinate(&self) -> String {
        coordinate_from_index_with_lock(
            self.column.get_num(),
            self.row.get_num(),
            self.column.get_is_lock(),
            self.row.get_is_lock(),
        )
    }
}
impl AdjustmentCoordinate for Coordinate {
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.column
            .adjustment_insert_value(root_col_num, offset_col_num);
        self.row
            .adjustment_insert_value(root_row_num, offset_row_num);
    }

    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.column
            .adjustment_remove_value(root_col_num, offset_col_num);
        self.row
            .adjustment_remove_value(root_row_num, offset_row_num);
    }

    fn is_remove_coordinate(
        &self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) -> bool {
        self.column.is_remove_value(root_col_num, offset_col_num)
            || self.row.is_remove_value(root_row_num, offset_row_num)
    }
}
