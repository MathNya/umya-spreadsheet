use thin_vec::ThinVec;

#[derive(Clone, Default, Debug)]
pub struct MediaObject {
    image_name: Box<str>,
    image_data: ThinVec<u8>,
}
impl MediaObject {
    pub fn get_image_name(&self) -> &str {
        &self.image_name
    }

    pub fn set_image_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.image_name = value.into().into_boxed_str();
        self
    }

    pub fn get_image_data(&self) -> &[u8] {
        &self.image_data
    }

    pub fn set_image_data(&mut self, value: impl Into<ThinVec<u8>>) -> &mut Self {
        self.image_data = value.into();
        self
    }

    pub(crate) fn get_rid(&self, rel_list: &mut Vec<(String, String)>) -> i32 {
        let find = rel_list
            .iter()
            .position(|(k, v)| k == "IMAGE" && v == &*self.image_name);
        match find {
            Some(v) => return (v + 1) as i32,
            None => {
                rel_list.push((String::from("IMAGE"), self.image_name.to_string()));
                return rel_list.len() as i32;
            }
        }
    }
}
