use super::border::Border;

#[derive(Default, Debug, Clone)]
pub struct Borders {
    left: Border,
    right: Border,
    top: Border,
    bottom: Border,
    diagonal: Border,
    diagonal_direction: usize,
    all_borders: Border,
    outline: Border,
    inside: Border,
    vertical: Border,
    horizontal: Border,
}
impl Borders {
    // Diagonal directions
    pub const DIAGONAL_NONE: usize = 0;
    pub const DIAGONAL_UP: usize = 1;
    pub const DIAGONAL_DOWN: usize = 2;
    pub const DIAGONAL_BOTH: usize = 3;

    pub fn get_left(&self)-> &Border {
        &self.left
    }
    pub fn get_left_mut(&mut self)-> &mut Border {
        &mut self.left
    }
    pub(crate) fn set_left(&mut self, value:Border) {
        self.left = value;
    }
    pub fn get_right(&self)-> &Border {
        &self.right
    }
    pub fn get_right_mut(&mut self)-> &mut Border {
        &mut self.right
    }
    pub(crate) fn set_right(&mut self, value:Border) {
        self.right = value;
    }
    pub fn get_top(&self)-> &Border {
        &self.top
    }
    pub fn get_top_mut(&mut self)-> &mut Border {
        &mut self.top
    }
    pub(crate) fn set_top(&mut self, value:Border) {
        self.top = value;
    }
    pub fn get_bottom(&self)-> &Border {
        &self.bottom
    }
    pub fn get_bottom_mut(&mut self)-> &mut Border {
        &mut self.bottom
    }
    pub(crate) fn set_bottom(&mut self, value:Border) {
        self.bottom = value;
    }
    pub fn get_diagonal(&self)-> &Border {
        &self.diagonal
    }
    pub fn get_diagonal_mut(&mut self)-> &mut Border {
        &mut self.diagonal
    }
    pub(crate) fn set_diagonal(&mut self, value:Border) {
        self.diagonal = value;
    }
    pub fn get_diagonal_direction(&self)-> &usize {
        &self.diagonal_direction
    }
    pub(crate) fn set_diagonal_direction(&mut self, value:usize) {
        self.diagonal_direction = value;
    }
    pub(crate) fn get_defalut_value() -> Borders {
        let def = Borders::default();
        def
    }
    pub(crate) fn get_hash_code(&self)-> String {
        format!("{:x}", md5::compute(format!("{}{}{}{}{}{}",
            &self.get_left().get_hash_code(),
            &self.get_right().get_hash_code(),
            &self.get_top().get_hash_code(),
            &self.get_bottom().get_hash_code(),
            &self.get_diagonal().get_hash_code(),
            &self.get_diagonal_direction()
        )))
    }
}