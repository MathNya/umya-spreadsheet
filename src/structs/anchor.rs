#[derive(Default, Debug, Clone)]
pub struct Anchor {
    left_column: u32,
    left_offset: u32,
    top_row: u32,
    top_offset: u32,
    right_column: u32,
    right_offset: u32,
    bottom_row: u32,
    bottom_offset: u32,
}
impl Anchor {
    pub fn get_left_column(&self) -> &u32 {
        &self.left_column
    }

    pub fn set_left_column(&mut self, value: u32) {
        self.left_column = value;
    }

    pub fn get_left_offset(&self) -> &u32 {
        &self.left_offset
    }

    pub fn set_left_offset(&mut self, value: u32) {
        self.left_offset = value;
    }

    pub fn get_top_row(&self) -> &u32 {
        &self.top_row
    }

    pub fn set_top_row(&mut self, value: u32) {
        self.top_row = value;
    }

    pub fn get_top_offset(&self) -> &u32 {
        &self.top_offset
    }

    pub fn set_top_offset(&mut self, value: u32) {
        self.top_offset = value;
    }

    pub fn get_right_column(&self) -> &u32 {
        &self.right_column
    }

    pub fn set_right_column(&mut self, value: u32) {
        self.right_column = value;
    }

    pub fn get_right_offset(&self) -> &u32 {
        &self.right_offset
    }

    pub fn set_right_offset(&mut self, value: u32) {
        self.right_offset = value;
    }

    pub fn get_bottom_row(&self) -> &u32 {
        &self.bottom_row
    }

    pub fn set_bottom_row(&mut self, value: u32) {
        self.bottom_row = value;
    }

    pub fn get_bottom_offset(&self) -> &u32 {
        &self.bottom_offset
    }

    pub fn set_bottom_offset(&mut self, value: u32) {
        self.bottom_offset = value;
    }

    pub(crate) fn _adjustment_insert_row(&mut self, num_rows: &u32) {
        self.top_row += num_rows;
        self.bottom_row += num_rows;
    }

    pub(crate) fn _adjustment_insert_column(&mut self, num_cols: &u32) {
        self.left_column += num_cols;
        self.right_column += num_cols;
    }

    pub(crate) fn _adjustment_remove_row(&mut self, num_rows: &u32) {
        self.top_row = if &self.top_row > num_rows {
            self.top_row - num_rows
        } else {
            1
        };
        self.bottom_row = if &self.bottom_row > num_rows {
            self.bottom_row - num_rows
        } else {
            1
        };
    }

    pub(crate) fn _adjustment_remove_column(&mut self, num_cols: &u32) {
        self.left_column = if &self.left_column > num_cols {
            self.left_column - num_cols
        } else {
            1
        };
        self.right_column = if &self.right_column > num_cols {
            self.right_column - num_cols
        } else {
            1
        };
    }
}
