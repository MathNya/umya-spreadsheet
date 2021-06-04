use super::Font;

#[derive(Clone, Debug)]
pub struct TextElement {
    text: String,
    font: Option<Font>
}
impl Default for TextElement {
    fn default() -> Self {
        Self {
            text: "".into(),
            font: None,
        }
    }
}
impl TextElement  {
    pub fn get_text(&self)-> &str {
        &self.text
    }
    pub fn set_text<S: Into<String>>(&mut self, value:S) {
        self.text = value.into();
    }
    pub fn get_font(&self)-> &Option<Font> {
        &self.font
    }
    pub fn get_font_mut(&mut self)-> &mut Font {
        match &self.font {
            Some(_) => return self.font.as_mut().unwrap(),
            None => {}
        }
        self.set_font(Font::get_defalut_value());
        self.font.as_mut().unwrap()
    }
    pub fn set_font(&mut self, value:Font) {
        self.font = Some(value);
    }
    pub(crate) fn get_hash_code(&self)-> String {
        format!("{:x}", md5::compute(format!("{}{}",
            &self.text,
            match &self.font {Some(v) => {v.get_hash_code()}, None => {"None".into()}},
        )))
    }
}