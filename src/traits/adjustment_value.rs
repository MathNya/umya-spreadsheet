pub(crate) trait AdjustmentValue {
    fn adjustment_insert_value(&mut self, root_num: &u32, offset_num: &u32);

    fn adjustment_remove_value(&mut self, root_num: &u32, offset_num: &u32);
}
