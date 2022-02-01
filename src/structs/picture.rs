use super::Hyperlink;
use super::Shadow;
use super::Worksheet;

#[derive(Default, Debug)]
pub struct Picture {
    image_counter: i32,
    image_index: i32,
    name: String,
    description: String,
    worksheet: Worksheet,
    coordinates: String,
    offset_x: i32,
    offset_v: i32,
    width: i32,
    height: i32,
    resize_proportional: bool,
    rotation: i32,
    shadow: Shadow,
    hyperlink: Hyperlink,
}
impl Picture {
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name<S: Into<String>>(&mut self, value: S) {
        self.name = value.into();
    }
}
