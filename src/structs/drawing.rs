use super::shadow::Shadow;
use super::hyperlink::Hyperlink;

#[derive(Debug)]
pub struct Drawing {
    name: String,
    coordinates: String,
    offset_x: usize,
    offset_y: usize,
    width: usize,
    height: usize,
    resize_proportional: bool,
    rotation: usize,
    shadow: Shadow,
    hyperlink: Hyperlink,
}
impl Default for Drawing {
    fn default() -> Self {
        Self {
            name: String::from(""),
            coordinates: String::from(""),
            offset_x: 0,
            offset_y: 0,
            width: 0,
            height: 0,
            resize_proportional: false,
            rotation: 0,
            shadow: Shadow::default(),
            hyperlink: Hyperlink::default(),
        }
    }
}
impl Drawing {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub(crate) fn set_name<S: Into<String>>(&mut self, value:S) {
        self.name = value.into();
    }
    pub fn get_coordinates(&self) -> &str {
        &self.coordinates
    }
    pub(crate) fn set_coordinates<S: Into<String>>(&mut self, value:S) {
        self.coordinates = value.into();
    }
    pub fn get_offset_x(&self) -> &usize {
        &self.offset_x
    }
    pub(crate) fn set_offset_x(&mut self, value:usize) {
        self.offset_x = value;
    }
    pub fn get_offset_y(&self) -> &usize {
        &self.offset_y
    }
    pub(crate) fn set_offset_y(&mut self, value:usize) {
        self.offset_y = value;
    }
    pub fn get_width(&self) -> &usize {
        &self.width
    }
    pub(crate) fn set_width(&mut self, value:usize) {
        self.width = value;
    }
    pub fn get_height(&self) -> &usize {
        &self.height
    }
    pub(crate) fn set_height(&mut self, value:usize) {
        self.height = value;
    }
}
