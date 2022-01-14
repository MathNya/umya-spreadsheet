#[derive(Clone, Default, Debug)]
pub struct Image {
    image_name: String,
    image_data: Vec<u8>,
}
impl Image {
    pub fn get_image_name(&self) -> &str {
        &self.image_name
    }

    pub fn set_image_name<S: Into<String>>(&mut self, value: S) {
        self.image_name = value.into();
    }

    pub fn get_image_data(&self) -> &Vec<u8> {
        &self.image_data
    }

    pub fn set_image_data(&mut self, value: Vec<u8>) {
        self.image_data = value;
    }

    pub(crate) fn get_extension(&self) -> String {
        let v: Vec<&str> = self.image_name.split('.').collect();
        let extension = v.last().unwrap().clone();
        let extension_lower = extension.to_lowercase();
        extension_lower
    }

    pub(crate) fn is_jpeg(&self) -> bool {
        self.get_extension() == "jpeg"
    }

    pub(crate) fn is_jpg(&self) -> bool {
        self.get_extension() == "jpg"
    }

    pub(crate) fn is_png(&self) -> bool {
        self.get_extension() == "png"
    }

    pub(crate) fn is_tiff(&self) -> bool {
        self.get_extension() == "tiff"
    }

    pub(crate) fn is_emf(&self) -> bool {
        self.get_extension() == "emf"
    }
}
