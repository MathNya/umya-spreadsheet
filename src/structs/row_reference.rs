use helper::coordinate::*;
use traits::AdjustmentValue;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RowReference {
    num: u32,
    is_lock: bool,
}

impl Default for RowReference {
    fn default() -> Self {
        Self {
            num: 1,
            is_lock: false,
        }
    }
}

impl RowReference {
    pub fn get_num(&self) -> &u32 {
        &self.num
    }

    pub fn set_num(&mut self, value: u32) -> &mut Self {
        self.num = value;
        self
    }

    pub(crate) fn offset_num(&mut self, value: i32) -> &mut Self {
        if value > 0 {
            self.plus_num(value as u32);
        }
        if value < 0 {
            self.minus_num(-value as u32);
        }
        self
    }

    pub(crate) fn plus_num(&mut self, value: u32) -> &mut Self {
        self.num += value;
        self
    }

    pub(crate) fn minus_num(&mut self, value: u32) -> &mut Self {
        self.num -= value;
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

    pub fn get_coordinate(&self) -> String {
        format!("{}{}", if self.is_lock { "$" } else { "" }, self.num)
    }
}
impl AdjustmentValue for RowReference {
    fn adjustment_insert_value(&mut self, root_num: &u32, offset_num: &u32) {
        self.num = adjustment_insert_coordinate(&self.num, root_num, offset_num);
    }

    fn adjustment_remove_value(&mut self, root_num: &u32, offset_num: &u32) {
        self.num = adjustment_remove_coordinate(&self.num, root_num, offset_num);
    }

    fn is_remove_value(&self, root_num: &u32, offset_num: &u32) -> bool {
        is_remove_coordinate(&self.num, root_num, offset_num)
    }
}
