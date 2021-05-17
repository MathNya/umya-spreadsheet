use super::super::helper::coordinate::*;

#[derive(Clone, Default, Debug)]
pub struct Coordinate {
    col_num: usize,
    row_num: usize,
    is_lock_col: bool,
    is_lock_row: bool,
}
impl Coordinate {
    pub fn get_col_num(&self)-> &usize {
        &self.col_num
    }

    pub fn set_col_num(&mut self, value:usize)-> &mut Coordinate {
        self.col_num = value;
        self
    }

    pub fn get_row_num(&self) -> &usize {
        &self.row_num
    }

    pub fn set_row_num(&mut self, value:usize)-> &mut Coordinate {
        self.row_num = value;
        self
    }

    pub fn get_is_lock_col(&self) -> &bool {
        &self.is_lock_col
    }

    pub fn set_is_lock_col(&mut self, value:bool)-> &mut Coordinate {
        self.is_lock_col = value;
        self
    }

    pub fn get_is_lock_row(&self) -> &bool {
        &self.is_lock_row
    }

    pub fn set_is_lock_row(&mut self, value:bool)-> &mut Coordinate {
        self.is_lock_row = value;
        self
    }

    pub fn set_coordinate<S: Into<String>>(&mut self, value:S)-> &mut Coordinate {
        let result = index_from_coordinate(value.into());
        self.col_num = result[0];
        self.row_num = result[1];
        self.is_lock_col = if result[2] == 1 { true } else { false };
        self.is_lock_col = if result[3] == 1 { true } else { false };
        self
    }

    pub fn get_coordinate(&self)-> String {
        coordinate_from_index_with_lock(&self.col_num, &self.row_num, &self.is_lock_col, &self.is_lock_row)
    }

    pub(crate) fn is_mine(&self, col_num:&usize, row_num:&usize)->bool {
        if &self.col_num != col_num {
            return false;
        }
        if &self.row_num != row_num {
            return false;
        }
        true
    }

    pub(crate) fn adjustment_insert_coordinate(&mut self, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        self.col_num = adjustment_insert_coordinate(&self.col_num, root_col_num, offset_col_num);
        self.row_num = adjustment_insert_coordinate(&self.row_num, root_row_num, offset_row_num);
    }

    pub(crate) fn adjustment_remove_coordinate(&mut self, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        self.col_num = adjustment_remove_coordinate(&self.col_num, root_col_num, offset_col_num);
        self.row_num = adjustment_remove_coordinate(&self.row_num, root_row_num, offset_row_num);
    }

    pub(crate) fn is_remove(&self, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize)->bool {
        if root_col_num > &0 {
            let col_result = &self.col_num >= root_col_num && &self.col_num < &(root_col_num + offset_col_num);
            return col_result;
        }
        if root_row_num > &0 {
            let row_result = &self.row_num >= root_row_num && &self.row_num < &(root_row_num + offset_row_num);
            return row_result;
        }
        false
    }
}