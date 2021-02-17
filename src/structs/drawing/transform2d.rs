// a:xfrm
#[derive(Default, Debug, Clone)]
pub struct Transform2D {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    rot: Option<String>,
    flip_v: Option<String>,
    flip_h: Option<String>,
}
impl Transform2D {
    pub fn get_x(&self) -> &usize {
        &self.x
    }

    pub fn set_x(&mut self, value:usize) {
        self.x = value;
    }

    pub fn get_y(&self) -> &usize {
        &self.y
    }

    pub fn set_y(&mut self, value:usize) {
        self.y = value;
    }

    pub fn get_width(&self) -> &usize {
        &self.width
    }

    pub fn set_width(&mut self, value:usize) {
        self.width = value;
    }

    pub fn get_height(&self) -> &usize {
        &self.height
    }
    
    pub fn set_height(&mut self, value:usize) {
        self.height = value;
    }

    pub fn get_rot(&self) -> &Option<String> {
        &self.rot
    }
    
    pub fn set_rot<S: Into<String>>(&mut self, value:S) {
        self.rot = Some(value.into());
    }

    pub fn get_flip_v(&self) -> &Option<String> {
        &self.flip_v
    }
    
    pub fn set_flip_v<S: Into<String>>(&mut self, value:S) {
        self.flip_v = Some(value.into());
    }

    pub fn get_flip_h(&self) -> &Option<String> {
        &self.flip_h
    }
    
    pub fn set_flip_h<S: Into<String>>(&mut self, value:S) {
        self.flip_h = Some(value.into());
    }
}
