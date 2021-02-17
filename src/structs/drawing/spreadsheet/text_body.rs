use super::body_properties::BodyProperties;
use super::list_style::ListStyle;
use super::text_body_content::TextBodyContent;

#[derive(Default, Debug)]
pub struct TextBody {
    body_properties: BodyProperties,
    list_style: ListStyle,
    text_body_contentes: Vec<TextBodyContent>,
}
impl TextBody {
    pub fn get_body_properties(&self)-> &BodyProperties {
        &self.body_properties
    }
    
    pub fn get_body_properties_mut(&mut self)-> &mut BodyProperties {
        &mut self.body_properties
    }

    pub fn set_body_properties(&mut self, value:BodyProperties) {
        self.body_properties = value;
    }

    pub fn get_list_style(&self)-> &ListStyle {
        &self.list_style
    }

    pub fn get_list_style_mut(&mut self)-> &mut ListStyle {
        &mut self.list_style
    }

    pub fn set_list_style(&mut self, value:ListStyle) {
        self.list_style = value;
    }

    pub fn get_text_body_contentes(&self)-> &Vec<TextBodyContent> {
        &self.text_body_contentes
    }

    pub fn get_text_body_contentes_mut(&mut self)-> &mut Vec<TextBodyContent> {
        &mut self.text_body_contentes
    }

    pub fn add_text_body_contentes(&mut self, value:TextBodyContent) {
        self.text_body_contentes.push(value);
    }
}