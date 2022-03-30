#[derive(Clone, Default, Debug)]
pub struct MediaObject {
    image_name: String,
    image_data: Vec<u8>,
}
impl MediaObject {
    pub fn get_image_name(&self) -> &str {
        &self.image_name
    }

    pub fn set_image_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.image_name = value.into();
        self
    }

    pub fn get_image_data(&self) -> &Vec<u8> {
        &self.image_data
    }

    pub fn set_image_data(&mut self, value: Vec<u8>) -> &mut Self {
        self.image_data = value;
        self
    }
}
