#[derive(Default, Debug, Clone)]
pub struct Anchor {
    left_column: usize,
    left_offset: usize,
    top_row: usize,
    top_offset: usize,
    right_column: usize,
    right_offset: usize,
    bottom_row: usize,
    bottom_offset: usize,
}
impl Anchor {
    pub fn get_left_column(&self)->&usize {
        &self.left_column
    }

    pub fn set_left_column(&mut self, value:usize) {
        self.left_column = value;
    }
    
    pub fn get_left_offset(&self)->&usize {
        &self.left_offset
    }

    pub fn set_left_offset(&mut self, value:usize) {
        self.left_offset = value;
    }

    pub fn get_top_row(&self)->&usize {
        &self.top_row
    }

    pub fn set_top_row(&mut self, value:usize) {
        self.top_row = value;
    }

    pub fn get_top_offset(&self)->&usize {
        &self.top_offset
    }

    pub fn set_top_offset(&mut self, value:usize) {
        self.top_offset = value;
    }

    pub fn get_right_column(&self)->&usize {
        &self.right_column
    }

    pub fn set_right_column(&mut self, value:usize) {
        self.right_column = value;
    }

    pub fn get_right_offset(&self)->&usize {
        &self.right_offset
    }

    pub fn set_right_offset(&mut self, value:usize) {
        self.right_offset = value;
    }

    pub fn get_bottom_row(&self)->&usize {
        &self.bottom_row
    }

    pub fn set_bottom_row(&mut self, value:usize) {
        self.bottom_row = value;
    }

    pub fn get_bottom_offset(&self)->&usize {
        &self.bottom_offset
    }

    pub fn set_bottom_offset(&mut self, value:usize) {
        self.bottom_offset = value.into();
    }

    pub(crate) fn adjustment_insert_row(&mut self, num_rows:&usize) {
        self.top_row += num_rows;
        self.bottom_row += num_rows;
    }

    pub(crate) fn adjustment_insert_colmun(&mut self, num_cols:&usize) {
        self.left_column += num_cols;
        self.right_column += num_cols;
    }

    pub(crate) fn adjustment_remove_row(&mut self, num_rows:&usize) {
        self.top_row = if &self.top_row > num_rows { self.top_row - num_rows } else { 1 };
        self.bottom_row = if &self.bottom_row > num_rows { self.bottom_row - num_rows } else { 1 };
    }

    pub(crate) fn adjustment_remove_colmun(&mut self, num_cols:&usize) {
        self.left_column = if &self.left_column > num_cols { self.left_column - num_cols } else { 1 };
        self.right_column = if &self.right_column > num_cols { self.right_column - num_cols } else { 1 };
    }
}
