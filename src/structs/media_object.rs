#[derive(Clone, Default, Debug)]
pub struct MediaObject {
    image_title: Box<str>,
    image_name: Box<str>,
    image_data: Vec<u8>,
}
impl MediaObject {
    #[inline]
    pub fn get_image_title(&self) -> &str {
        &self.image_title
    }

    #[inline]
    pub fn set_image_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.image_title = value.into().into_boxed_str();
        self
    }

    #[inline]
    pub fn get_image_name(&self) -> &str {
        &self.image_name
    }

    #[inline]
    pub fn set_image_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.image_name = value.into().into_boxed_str();
        self
    }

    #[inline]
    pub fn get_image_data(&self) -> &[u8] {
        &self.image_data
    }

    #[inline]
    pub fn set_image_data(&mut self, value: impl Into<Vec<u8>>) -> &mut Self {
        self.image_data = value.into();
        self
    }

    pub(crate) fn get_rid(&self, rel_list: &mut Vec<(String, String)>) -> i32 {
        let find = rel_list
            .iter()
            .position(|(k, v)| k == "IMAGE" && v == &*self.image_name);
        if let Some(v) = find {
            (v + 1) as i32
        } else {
            rel_list.push((String::from("IMAGE"), self.image_name.to_string()));
            rel_list.len() as i32
        }
    }
}
