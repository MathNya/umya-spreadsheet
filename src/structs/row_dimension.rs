#[derive(Default, Debug)]
pub struct RowDimension {
    row_num: usize,
    height: f32,
    descent: f32,
    thick_bot: bool,
    custom_height: bool
}
impl RowDimension {
    pub(crate) fn get_row_num(&self) -> &usize {
        &self.row_num
    }

    pub(crate) fn set_row_num(&mut self, value:&usize) {
        self.row_num = value.clone();
    }
    
    pub fn get_height(&self)-> &f32 {
        &self.height
    }

    pub(crate) fn set_height(&mut self, value:f32) {
        self.height = value;
    }

    pub fn get_descent(&self)-> &f32 {
        &self.descent
    }

    pub(crate) fn set_descent(&mut self, value:f32) {
        self.descent = value;
    }

    pub fn get_thick_bot(&self)-> &bool {
        &self.thick_bot
    }

    pub(crate) fn set_thick_bot(&mut self, value:bool) {
        self.thick_bot = value;
    }

    pub fn get_custom_height(&self)-> &bool {
        &self.custom_height
    }
    
    pub(crate) fn set_custom_height(&mut self, value:bool) {
        self.custom_height = value;
    }

    pub(crate) fn adjustment_insert_coordinate(&mut self, root_row_num:&usize, offset_row_num:&usize) {
        if &self.row_num >= root_row_num {
            self.row_num = self.row_num + offset_row_num;
        }
    }

    pub(crate) fn adjustment_remove_coordinate(&mut self, root_row_num:&usize, offset_row_num:&usize) {
        if &self.row_num >= root_row_num {
            self.row_num = self.row_num - offset_row_num;
        }
    }
}
