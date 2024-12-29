#[derive(Clone, Default, Debug)]
pub struct MediaObject {
    title: Box<str>,
    name:  Box<str>,
    data:  Vec<u8>,
}
impl MediaObject {
    pub fn get_image_title(&self) -> &str {
        &self.title
    }

    pub fn set_image_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.title = value.into().into_boxed_str();
        self
    }

    pub fn get_image_name(&self) -> &str {
        &self.name
    }

    pub fn set_image_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.name = value.into().into_boxed_str();
        self
    }

    pub fn get_image_data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_image_data(&mut self, value: impl Into<Vec<u8>>) -> &mut Self {
        self.data = value.into();
        self
    }

    pub(crate) fn get_rid(&self, rel_list: &mut Vec<(String, String)>) -> i32 {
        let find = rel_list
            .iter()
            .position(|(k, v)| k == "IMAGE" && v == &*self.name);
        if let Some(v) = find {
            (v + 1).try_into().unwrap()
        } else {
            rel_list.push((String::from("IMAGE"), self.name.to_string()));
            rel_list.len().try_into().unwrap()
        }
    }
}
