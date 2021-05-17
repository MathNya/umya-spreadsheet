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

    pub fn set_position<S: Into<String>>(&mut self, value:S)-> &mut Legend {
        self.position = value.into();
        self
    }

    pub fn get_overlay(&self)-> &bool {
        &self.overlay
    }

    pub fn set_overlay(&mut self, value:bool)-> &mut Legend {
        self.overlay = value;
        self
    }

    pub fn get_layout(&self)-> &Layout {
        &self.layout
    }

    pub fn set_layout(&mut self, value:Layout)-> &mut Legend {
        self.layout = value;
        self
    }
}
