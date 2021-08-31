use helper::coordinate::*;

#[derive(Clone, Default, Debug)]
pub struct Column {
    num: usize,
    is_lock: bool,
}
impl Column {
    pub fn get_num(&self)-> &usize {
        &self.num
    }

    pub fn set_num(&mut self, value:usize)-> &mut Self {
        self.num = value;
        self
    }

    pub fn get_is_lock(&self) -> &bool {
        &self.is_lock
    }

    pub fn set_is_lock(&mut self, value:bool)-> &mut Self {
        self.is_lock = value;
        self
    }

    pub fn set_is_lock_usize(&mut self, value:usize)-> &mut Self {
        self.is_lock = if value == 1 {true} else {false};
        self
    }

    pub(crate) fn get_coordinate(&self)-> String {
        format!(
            "{}{}",
            if &self.is_lock == &true {"$"} else {""},
            string_from_column_index(&self.num),
        )
    }

    pub(crate) fn is_mine(&self, num:&usize)->bool {
        if &self.num != num {
            return false;
        }
        true
    }

    pub(crate) fn adjustment_insert_coordinate(&mut self, root_col_num:&usize, offset_col_num:&usize) {
        self.num = adjustment_insert_coordinate(&self.num, root_col_num, offset_col_num);
    }

    pub(crate) fn adjustment_remove_coordinate(&mut self, root_col_num:&usize, offset_col_num:&usize) {
        self.num = adjustment_remove_coordinate(&self.num, root_col_num, offset_col_num);
    }

    pub(crate) fn is_remove(&self, root_col_num:&usize, offset_col_num:&usize)->bool {
        if root_col_num > &0 {
            let col_result = &self.num >= root_col_num && &self.num < &(root_col_num + offset_col_num);
            return col_result;
        }
        false
    }
}