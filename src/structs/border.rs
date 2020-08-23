use super::color::Color;

#[derive(Default, Debug, Clone)]
pub struct Border {
    border_style: String,
    color: Color,
}
impl Border {
    // Border style
    pub const BORDER_NONE: &'static str = "none";
    pub const BORDER_DASHDOT: &'static str = "dashDot";
    pub const BORDER_DASHDOTDOT: &'static str = "dashDotDot";
    pub const BORDER_DASHED: &'static str = "dashed";
    pub const BORDER_DOTTED: &'static str = "dotted";
    pub const BORDER_DOUBLE: &'static str = "double";
    pub const BORDER_HAIR: &'static str = "hair";
    pub const BORDER_MEDIUM: &'static str = "medium";
    pub const BORDER_MEDIUMDASHDOT: &'static str = "mediumDashDot";
    pub const BORDER_MEDIUMDASHDOTDOT: &'static str = "mediumDashDotDot";
    pub const BORDER_MEDIUMDASHED: &'static str = "mediumDashed";
    pub const BORDER_SLANTDASHDOT: &'static str = "slantDashDot";
    pub const BORDER_THICK: &'static str = "thick";
    pub const BORDER_THIN: &'static str = "thin";

    pub(crate) fn has_border_style(&self) -> bool {
        &self.border_style != Self::BORDER_NONE
    }
    pub fn get_border_style(&self)-> &String {
        &self.border_style
    }
    pub(crate) fn set_border_style(&mut self, value:String) {
        self.border_style = value;
    }
    pub fn get_color(&self)-> &Color {
        &self.color
    }
    pub(crate) fn get_color_mut(&mut self)-> &mut Color {
        &mut self.color
    }
    pub(crate) fn set_color(&mut self, value:Color) {
        self.color = value;
    }
    pub(crate) fn get_hash_code(&self)-> String {
        format!("{:x}", md5::compute(format!("{}{}",
            &self.border_style,
            &self.get_color().get_hash_code()
        )))
    }
}