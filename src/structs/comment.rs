use super::rich_text::RichText;
use super::color::Color;
use super::coordinate::Coordinate;

#[derive(Default, Debug, Clone)]
pub struct Comment {
    coordinate: Coordinate,
    author: String,
    text: RichText,
    width: String,
    margin_left: String,
    margin_top: String,
    visible: bool,
    height: String,
    fill_color: Color,
    alignment: String,
}
impl Comment {
    pub fn get_coordinate(&self)-> &Coordinate {
        &self.coordinate
    }

    pub fn get_coordinate_mut(&mut self)-> &mut Coordinate {
        &mut self.coordinate
    }

    pub fn get_author(&self)->&str {
        &self.author
    }

    pub(crate) fn set_author<S: Into<String>>(&mut self, value:S) {
        self.author = value.into();
    }

    pub fn get_text(&self)->&RichText {
        &self.text
    }

    pub(crate) fn set_text(&mut self, value:RichText) {
        self.text = value;
    }

    pub fn get_width(&self)->&str {
        &self.width
    }

    pub(crate) fn set_width<S: Into<String>>(&mut self, value:S) {
        self.width = value.into();
    }

    pub fn get_margin_left(&self)->&str {
        &self.margin_left
    }

    pub(crate) fn set_margin_left<S: Into<String>>(&mut self, value:S) {
        self.margin_left = value.into();
    }

    pub fn get_margin_top(&self)->&str {
        &self.margin_top
    }

    pub(crate) fn set_margin_top<S: Into<String>>(&mut self, value:S) {
        self.margin_top = value.into();
    }

    pub fn get_visible(&self)->&bool {
        &self.visible
    }

    pub(crate) fn set_visible(&mut self, value:bool) {
        self.visible = value;
    }

    pub fn get_height(&self)->&str {
        &self.height
    }

    pub(crate) fn set_height<S: Into<String>>(&mut self, value:S) {
        self.height = value.into();
    }
    
    pub fn get_fill_color(&self)->&Color {
        &self.fill_color
    }

    pub(crate) fn set_fill_color(&mut self, value:Color) {
        self.fill_color = value;
    }

    pub fn get_alignment(&self)->&str {
        &self.alignment
    }

    pub(crate) fn set_alignment<S: Into<String>>(&mut self, value:S) {
        self.alignment = value.into();
    }
}
