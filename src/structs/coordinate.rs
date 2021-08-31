use super::Column;
use super::Row;
use helper::coordinate::*;

#[derive(Clone, Default, Debug)]
pub struct Coordinate {
    column: Column,
    row: Row,
}
impl Coordinate {
    pub fn get_col_num(&self)-> &usize {
        &self.column.get_num()
    }

    pub fn set_col_num(&mut self, value:usize)-> &mut Self {
        self.column.set_num(value);
        self
    }

    pub fn get_row_num(&self) -> &usize {
        &self.row.get_num()
    }

    pub fn set_row_num(&mut self, value:usize)-> &mut Self {
        self.row.set_num(value);
        self
    }

    pub fn get_is_lock_col(&self) -> &bool {
        self.column.get_is_lock()
    }

    pub fn set_is_lock_col(&mut self, value:bool)-> &mut Self {
        self.column.set_is_lock(value);
        self
    }

    pub fn get_is_lock_row(&self) -> &bool {
        self.row.get_is_lock()
    }

    pub fn set_is_lock_row(&mut self, value:bool)-> &mut Self {
        self.row.set_is_lock(value);
        self
    }

    pub fn set_coordinate<S: Into<String>>(&mut self, value:S)-> &mut Self {
        let result = index_from_coordinate(value.into());
        self.column.set_num(result[0].unwrap());
        self.row.set_num(result[1].unwrap());
        self.column.set_is_lock_usize(result[2].unwrap());
        self.row.set_is_lock_usize(result[3].unwrap());
        self
    }

    pub fn get_coordinate(&self)-> String {
        coordinate_from_index_with_lock(&self.column.get_num(), &self.row.get_num(), &self.column.get_is_lock(), &self.row.get_is_lock())
    }

    pub(crate) fn is_mine(&self, col_num:&usize, row_num:&usize)->bool {
        self.column.is_mine(col_num) && self.row.is_mine(row_num)
    }

    pub(crate) fn adjustment_insert_coordinate(&mut self, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        self.column.adjustment_insert_coordinate(root_col_num, offset_col_num);
        self.row.adjustment_insert_coordinate(root_row_num, offset_row_num);
    }

    pub(crate) fn adjustment_remove_coordinate(&mut self, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        self.column.adjustment_remove_coordinate(root_col_num, offset_col_num);
        self.row.adjustment_remove_coordinate(root_row_num, offset_row_num);
    }

    pub(crate) fn is_remove(&self, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize)->bool {
        if self.column.is_remove(root_col_num, offset_col_num) {
            return true;
        }
        if self.row.is_remove(root_row_num, offset_row_num) {
            return true;
        }
        false
    }
}