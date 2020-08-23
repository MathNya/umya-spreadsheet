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
    pub(crate) fn set_left(&mut self, value:Border) {
        self.left = value;
    }
    pub fn get_right(&self)-> &Border {
        &self.right
    }
    pub(crate) fn set_right(&mut self, value:Border) {
        self.right = value;
    }
    pub fn get_top(&self)-> &Border {
        &self.top
    }
    pub(crate) fn set_top(&mut self, value:Border) {
        self.top = value;
    }
    pub fn get_bottom(&self)-> &Border {
        &self.bottom
    }
    pub(crate) fn set_bottom(&mut self, value:Border) {
        self.bottom = value;
    }
    pub fn get_diagonal(&self)-> &Border {
        &self.diagonal
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
    pub(crate) fn get_defalut_borders() -> Vec<Borders> {
        let mut def_1 = Borders::default();
        let mut left = Border::default();
        let mut right = Border::default();
        let mut top = Border::default();
        let mut bottom = Border::default();
        let mut diagonal = Border::default();
        left.set_border_style(String::from(Border::BORDER_NONE));
        right.set_border_style(String::from(Border::BORDER_NONE));
        top.set_border_style(String::from(Border::BORDER_NONE));
        bottom.set_border_style(String::from(Border::BORDER_NONE));
        diagonal.set_border_style(String::from(Border::BORDER_NONE));
        def_1.set_left(left);
        def_1.set_right(right);
        def_1.set_top(top);
        def_1.set_bottom(bottom);
        def_1.set_diagonal(diagonal);

        vec![def_1]
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