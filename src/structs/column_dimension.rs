#[derive(Default, Debug)]
pub struct ColumnDimension {
    col_num_start: usize,
    col_num_end: usize,
    width: f32,
    best_fit: bool,
}
impl ColumnDimension {
    pub fn get_col_num_start(&self)-> &usize {
        &self.col_num_start
    }

    pub(crate) fn set_col_num_start(&mut self, value:usize) {
        self.col_num_start = value;
    }

    pub fn get_col_num_end(&self)-> &usize {
        &self.col_num_end
    }

    pub(crate) fn set_col_num_end(&mut self, value:usize) {
        self.col_num_end = value;
    }

    pub fn get_width(&self)-> &f32 {
        &self.width
    }

    pub(crate) fn set_width(&mut self, value:f32) {
        self.width = value;
    }
    
    pub fn get_best_fit(&self)-> &bool {
        &self.best_fit
    }

    pub(crate) fn set_best_fit(&mut self, value:bool) {
        self.best_fit = value;
    }

    pub(crate) fn adjustment_insert_coordinate(&mut self, root_col_num:&usize, offset_col_num:&usize) {
        if &self.col_num_start >= root_col_num {
            self.col_num_start = self.col_num_start + offset_col_num;
        }
        if &self.col_num_end >= root_col_num {
            self.col_num_end = self.col_num_end + offset_col_num;
        }
    }

    pub(crate) fn adjustment_remove_coordinate(&mut self, root_col_num:&usize, offset_col_num:&usize) {
        if &self.col_num_start >= root_col_num {
            self.col_num_start = self.col_num_start - offset_col_num;
        }
        if &self.col_num_end >= root_col_num {
            self.col_num_end = self.col_num_end - offset_col_num;
        }
    }
}
