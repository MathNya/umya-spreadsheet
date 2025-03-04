use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub struct MediaObject {
    image_title: Box<str>,
    image_name: Box<str>,
    image_data: ThinVec<u8>,
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
    pub fn set_image_data(&mut self, value: impl Into<ThinVec<u8>>) -> &mut Self {
        self.image_data = value.into();
        self
    }

    pub(crate) fn get_rid(&self, rel_list: &mut Vec<(String, String)>) -> u32 {
        rel_list
            .iter()
            .position(|(k, v)| k == "IMAGE" && v == &*self.image_name)
            .map(|index| (index + 1) as u32)
            .unwrap_or_else(|| {
                rel_list.push((String::from("IMAGE"), self.image_name.to_string()));
                rel_list.len() as u32
            })
    }
}
