use helper::coordinate::*;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ColumnReference {
    num: u32,
    is_lock: bool,
}

impl Default for ColumnReference {
    fn default() -> Self {
        Self {
            num: 1,
            is_lock: false,
        }
    }
}

impl ColumnReference {
    pub fn get_num(&self) -> &u32 {
        &self.num
    }

    pub fn set_num(&mut self, value: u32) -> &mut Self {
        self.num = value;
        self
    }

    pub fn get_is_lock(&self) -> &bool {
        &self.is_lock
    }

    pub fn set_is_lock(&mut self, value: bool) -> &mut Self {
        self.is_lock = value;
        self
    }

    pub fn set_is_lock_usize(&mut self, value: u32) -> &mut Self {
        self.is_lock = value == 1;
        self
    }

    pub(crate) fn get_coordinate(&self) -> String {
        format!(
            "{}{}",
            if &self.is_lock == &true { "$" } else { "" },
            string_from_column_index(&self.num),
        )
    }

    pub(crate) fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
    ) {
        self.num = adjustment_insert_coordinate(&self.num, root_col_num, offset_col_num);
    }

    pub(crate) fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
    ) {
        self.num = adjustment_remove_coordinate(&self.num, root_col_num, offset_col_num);
    }

    pub(crate) fn is_remove(&self, root_col_num: &u32, offset_col_num: &u32) -> bool {
        if root_col_num > &0 {
            let col_result =
                &self.num >= root_col_num && &self.num < &(root_col_num + offset_col_num);
            return col_result;
        }
        false
    }
}
