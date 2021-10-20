use super::RichText;
use super::Color;
use super::Coordinate;
use super::Anchor;

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
    anchor: Anchor,
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

    pub fn set_author<S: Into<String>>(&mut self, value:S) {
        self.author = value.into();
    }

    pub fn get_text(&self)->&RichText {
        &self.text
    }

    pub fn get_text_mut(&mut self)->&mut RichText {
        &mut self.text
    }

    pub fn set_text(&mut self, value:RichText) {
        self.text = value;
    }

    pub fn get_width(&self)->&str {
        &self.width
    }

    pub fn set_width<S: Into<String>>(&mut self, value:S) {
        self.width = value.into();
    }

    pub fn get_margin_left(&self)->&str {
        &self.margin_left
    }

    pub fn set_margin_left<S: Into<String>>(&mut self, value:S) {
        self.margin_left = value.into();
    }

    pub fn get_margin_top(&self)->&str {
        &self.margin_top
    }

    pub fn set_margin_top<S: Into<String>>(&mut self, value:S) {
        self.margin_top = value.into();
    }

    pub fn get_visible(&self)->&bool {
        &self.visible
    }

    pub fn set_visible(&mut self, value:bool) {
        self.visible = value;
    }

    pub fn get_height(&self)->&str {
        &self.height
    }

    pub fn set_height<S: Into<String>>(&mut self, value:S) {
        self.height = value.into();
    }
    
    pub fn get_fill_color(&self)->&Color {
        &self.fill_color
    }

    pub fn set_fill_color(&mut self, value:Color) {
        self.fill_color = value;
    }

    pub fn get_alignment(&self)->&str {
        &self.alignment
    }

    pub fn set_alignment<S: Into<String>>(&mut self, value:S) {
        self.alignment = value.into();
    }

    pub fn get_anchor(&self)->&Anchor {
        &self.anchor
    }

    pub fn get_anchor_mut(&mut self)->&mut Anchor {
        &mut self.anchor
    }

    pub fn set_anchor(&mut self, value:Anchor) {
        self.anchor = value;
    }

    pub(crate) fn adjustment_insert_coordinate(&mut self, root_col_num:&u32, offset_col_num:&u32, root_row_num:&u32, offset_row_num:&u32) {
        let org_col_num = self.coordinate.get_col_num().clone();
        let org_row_num = self.coordinate.get_row_num().clone();
        self.coordinate.adjustment_insert_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
        if &org_col_num != self.coordinate.get_col_num() {
            self.anchor.adjustment_insert_colmun(offset_col_num);
        }
        if &org_row_num != self.coordinate.get_row_num() {
            self.anchor.adjustment_insert_row(offset_row_num);
        }
    }

    pub(crate) fn adjustment_remove_coordinate(&mut self, root_col_num:&u32, offset_col_num:&u32, root_row_num:&u32, offset_row_num:&u32) {
        let org_col_num = self.coordinate.get_col_num().clone();
        let org_row_num = self.coordinate.get_row_num().clone();
        self.coordinate.adjustment_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
        if &org_col_num != self.coordinate.get_col_num() {
            self.anchor.adjustment_remove_colmun(offset_col_num);
        }
        if &org_row_num != self.coordinate.get_row_num() {
            self.anchor.adjustment_remove_row(offset_row_num);
        }
    }
}
