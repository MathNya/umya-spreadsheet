// a:tailEnd
#[derive(Default, Debug)]
pub struct TailEnd {
    r#type: String,
}
impl TailEnd {
    pub fn get_type(&self) -> &str {
        &self.r#type
    }

    pub fn set_type<S: Into<String>>(&mut self, value:S) {
        self.r#type = value.into();
    }
}
