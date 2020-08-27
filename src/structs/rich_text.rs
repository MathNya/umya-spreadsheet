use super::text_element::TextElement;

#[derive(Clone, Debug)]
pub struct RichText  {
    rich_text_elements: Vec<TextElement>,
}
impl Default for RichText  {
    fn default() -> Self {
        Self {
            rich_text_elements: Vec::new(),
        }
    }
}
impl RichText  {
    pub fn get_rich_text_elements(&self)-> &Vec<TextElement> {
        &self.rich_text_elements
    }
    pub(crate) fn set_rich_text_elements(&mut self, value:Vec<TextElement>) {
        self.rich_text_elements = value;
    }
    pub(crate) fn add_rich_text_elements(&mut self, value:TextElement) {
        self.rich_text_elements.push(value);
    }
    pub(crate) fn get_hash_code(&self)-> String {
        let mut value = String::from("");
        for ele in &self.rich_text_elements {
            value += ele.get_hash_code().as_str();
        }
        format!("{:x}", md5::compute(format!("{}",
            value
        )))
    }
}