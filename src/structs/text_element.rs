#[derive(Clone, Debug)]
pub struct TextElement {
    text: String,
}
impl Default for TextElement {
    fn default() -> Self {
        Self {
            text: "".into(),
        }
    }
}
impl TextElement  {
    pub fn get_text(&self)-> &str {
        &self.text
    }
    pub(crate) fn set_text<S: Into<String>>(&mut self, value:S) {
        self.text = value.into();
    }
}