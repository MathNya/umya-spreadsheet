use super::layout::Layout;

#[derive(Default, Debug)]
pub struct Legend {
    position: String,
    overlay: bool,
    layout: Layout,
}
impl Legend  {
    pub fn get_position(&self)-> &str {
        &self.position
    }
    pub(crate) fn set_position<S: Into<String>>(&mut self, value:S) {
        self.position = value.into();
    }
    pub fn get_overlay(&self)-> &bool {
        &self.overlay
    }
    pub(crate) fn set_overlay(&mut self, value:bool) {
        self.overlay = value;
    }
    pub fn get_layout(&self)-> &Layout {
        &self.layout
    }
    pub(crate) fn set_layout(&mut self, value:Layout) {
        self.layout = value;
    }
}
