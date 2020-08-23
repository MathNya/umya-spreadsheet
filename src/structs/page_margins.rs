#[derive(Default, Debug)]
pub struct PageMargins {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    header: f32,
    footer: f32,
}
impl PageMargins {
    pub fn get_left(&self)-> &f32 {
        &self.left
    }
    pub(crate) fn set_left(&mut self, value:f32) {
        self.left = value;
    }
    pub fn get_right(&self)-> &f32 {
        &self.right
    }
    pub(crate) fn set_right(&mut self, value:f32) {
        self.right = value;
    }
    pub fn get_top(&self)-> &f32 {
        &self.top
    }
    pub(crate) fn set_top(&mut self, value:f32) {
        self.top = value;
    }
    pub fn get_bottom(&self)-> &f32 {
        &self.bottom
    }
    pub(crate) fn set_bottom(&mut self, value:f32) {
        self.bottom = value;
    }
    pub fn get_header(&self)-> &f32 {
        &self.header
    }
    pub(crate) fn set_header(&mut self, value:f32) {
        self.header = value;
    }
    pub fn get_footer(&self)-> &f32 {
        &self.footer
    }
    pub(crate) fn set_footer(&mut self, value:f32) {
        self.footer = value;
    }
}