use super::ColumnReference;
use super::RowReference;
use helper::coordinate::*;
use traits::AdjustmentCoordinate;
use traits::AdjustmentValue;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Coordinate {
    column: ColumnReference,
    row: RowReference,
}

impl ToString for Coordinate {
    #[inline]
    fn to_string(&self) -> String {
        coordinate_from_index_with_lock(
            self.column.get_num(),
            self.row.get_num(),
            self.column.get_is_lock(),
            self.row.get_is_lock(),
        )
    }
}

impl Coordinate {
    #[inline]
    pub fn get_col_num(&self) -> &u32 {
        self.column.get_num()
    }

    #[inline]
    pub fn set_col_num(&mut self, value: u32) -> &mut Self {
        self.column.set_num(value);
        self
    }

    #[inline]
    pub(crate) fn offset_col_num(&mut self, value: i32) -> &mut Self {
        self.column.offset_num(value);
        self
    }

    #[inline]
    pub fn get_row_num(&self) -> &u32 {
        self.row.get_num()
    }

    #[inline]
    pub fn set_row_num(&mut self, value: u32) -> &mut Self {
        self.row.set_num(value);
        self
    }

    #[inline]
    pub(crate) fn offset_row_num(&mut self, value: i32) -> &mut Self {
        self.row.offset_num(value);
        self
    }

    #[inline]
    pub fn get_is_lock_col(&self) -> &bool {
        self.column.get_is_lock()
    }

    #[inline]
    pub fn set_is_lock_col(&mut self, value: bool) -> &mut Self {
        self.column.set_is_lock(value);
        self
    }

    #[inline]
    pub fn get_is_lock_row(&self) -> &bool {
        self.row.get_is_lock()
    }

    #[inline]
    pub fn set_is_lock_row(&mut self, value: bool) -> &mut Self {
        self.row.set_is_lock(value);
        self
    }

    /// Change coordinates
    /// Formula is not updated.
    #[inline]
    pub fn set_coordinate<S: AsRef<str>>(&mut self, value: S) -> &mut Self {
        let (c, r, cl, rl) = index_from_coordinate(value.as_ref());

        self.column.set_num(c.unwrap());
        self.row.set_num(r.unwrap());
        self.column.set_is_lock(cl.unwrap());
        self.row.set_is_lock(rl.unwrap());

        self
    }

    #[inline]
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
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.column
            .adjustment_insert_value(root_col_num, offset_col_num);
        self.row
            .adjustment_insert_value(root_row_num, offset_row_num);
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        self.column
            .adjustment_remove_value(root_col_num, offset_col_num);
        self.row
            .adjustment_remove_value(root_row_num, offset_row_num);
    }

    #[inline]
    fn is_remove_coordinate(
        &self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) -> bool {
        self.column.is_remove_value(root_col_num, offset_col_num)
            || self.row.is_remove_value(root_row_num, offset_row_num)
    }
}
