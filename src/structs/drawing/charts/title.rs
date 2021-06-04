use super::Layout;
use super::super::super::RichText;

#[derive(Default, Debug)]
pub struct Title {
    caption: RichText,
    layout: Option<Layout>,
}
impl Title {
    pub fn get_caption(&self) -> &RichText {
        &self.caption
    }

    pub fn get_caption_mut(&mut self) -> &mut RichText {
        &mut self.caption
    }

    pub fn set_caption(&mut self, value:RichText)-> &mut Title {
        self.caption = value.into();
        self
    }

    pub fn get_layout(&self)-> &Option<Layout> {
        &self.layout
    }

    pub fn set_layout(&mut self, value:Layout)-> &mut Title {
        self.layout = Some(value);
        self
    }
}