use super::layout::Layout;
use super::super::super::rich_text::RichText;

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
    pub(crate) fn set_caption(&mut self, value:RichText) {
        self.caption = value.into();
    }
    pub fn get_layout(&self) -> &Option<Layout> {
        &self.layout
    }
    pub(crate) fn set_layout(&mut self, value:Layout) {
        self.layout = Some(value);
    }
}