#[derive(Clone, Default, Debug)]
pub struct MediaObject {
    title: Box<str>,
    name:  Box<str>,
    data:  Vec<u8>,
}
impl MediaObject {
    #[inline]
    pub fn image_title(&self) -> &str {
        &self.title
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use image_title()")]
    pub fn get_image_title(&self) -> &str {
        self.image_title()
    }

    #[inline]
    pub fn set_image_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.title = value.into().into_boxed_str();
        self
    }

    #[inline]
    pub fn image_name(&self) -> &str {
        &self.name
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use image_name()")]
    pub fn get_image_name(&self) -> &str {
        self.image_name()
    }

    #[inline]
    pub fn set_image_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name = value.into().into_boxed_str();
        self
    }

    #[inline]
    pub fn image_data(&self) -> &[u8] {
        &self.data
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use image_data()")]
    pub fn get_image_data(&self) -> &[u8] {
        self.image_data()
    }

    #[inline]
    pub fn set_image_data(&mut self, value: impl Into<Vec<u8>>) -> &mut Self {
        self.data = value.into();
        self
    }

    #[allow(clippy::cast_possible_truncation)]
    pub(crate) fn rid(&self, rel_list: &mut Vec<(String, String)>) -> u32 {
        rel_list
            .iter()
            .position(|(k, v)| k == "IMAGE" && v == &*self.name)
            .map_or_else(|| {
                rel_list.push((String::from("IMAGE"), self.name.to_string()));
                rel_list.len() as u32
            }, |index| (index + 1) as u32)
    }

    #[deprecated(since = "3.0.0", note = "Use rid()")]
    pub(crate) fn get_rid(&self, rel_list: &mut Vec<(String, String)>) -> u32 {
        self.rid(rel_list)
    }
}
