use super::color::Color;

#[derive(Default, Debug, Clone)]
pub struct Fill {
    startcolor_index: usize,
    endcolor_index: usize,
    fill_type: String,
    rotation: i32,
    start_color: Color,
    end_color: Color
}
impl Fill {
    // Fill types
    pub const FILL_NONE: &'static str = "none";
    pub const FILL_SOLID: &'static str = "solid";
    pub const FILL_GRADIENT_LINEAR: &'static str = "linear";
    pub const FILL_GRADIENT_PATH: &'static str = "path";
    pub const FILL_PATTERN_DARKDOWN: &'static str = "darkDown";
    pub const FILL_PATTERN_DARKGRAY: &'static str = "darkGray";
    pub const FILL_PATTERN_DARKGRID: &'static str = "darkGrid";
    pub const FILL_PATTERN_DARKHORIZONTAL: &'static str = "darkHorizontal";
    pub const FILL_PATTERN_DARKTRELLIS: &'static str = "darkTrellis";
    pub const FILL_PATTERN_DARKUP: &'static str = "darkUp";
    pub const FILL_PATTERN_DARKVERTICAL: &'static str = "darkVertical";
    pub const FILL_PATTERN_GRAY0625: &'static str = "gray0625";
    pub const FILL_PATTERN_GRAY125: &'static str = "gray125";
    pub const FILL_PATTERN_LIGHTDOWN: &'static str = "lightDown";
    pub const FILL_PATTERN_LIGHTGRAY: &'static str = "lightGray";
    pub const FILL_PATTERN_LIGHTGRID: &'static str = "lightGrid";
    pub const FILL_PATTERN_LIGHTHORIZONTAL: &'static str = "lightHorizontal";
    pub const FILL_PATTERN_LIGHTTRELLIS: &'static str = "lightTrellis";
    pub const FILL_PATTERN_LIGHTUP: &'static str = "lightUp";
    pub const FILL_PATTERN_LIGHTVERTICAL: &'static str = "lightVertical";
    pub const FILL_PATTERN_MEDIUMGRAY: &'static str = "mediumGray";

    pub fn get_startcolor_index(&self)-> &usize {
        &self.startcolor_index
    }
    pub(crate) fn set_startcolor_index(&mut self, value:usize) {
        self.startcolor_index = value;
    }
    pub fn get_endcolor_index(&self)-> &usize {
        &self.endcolor_index
    }
    pub fn get_fill_type(&self)-> &str {
        &self.fill_type
    }
    pub(crate) fn set_fill_type(&mut self, value:String) {
        self.fill_type = value;
    }
    pub fn get_start_color(&self)-> &Color {
        &self.start_color
    }
    pub fn get_start_color_mut(&mut self)-> &mut Color {
        &mut self.start_color
    }
    pub(crate) fn set_start_color(&mut self, value:Color) {
        self.start_color = value;
    }
    pub fn get_end_color(&self)-> &Color {
        &self.end_color
    }
    pub fn get_end_color_mut(&mut self)-> &mut Color {
        &mut self.end_color
    }
    pub(crate) fn set_end_color(&mut self, value:Color) {
        self.end_color = value;
    }
    pub(crate) fn get_defalut_fills() -> Vec<Fill> {
        let mut def_1 = Fill::default();
        def_1.set_fill_type(String::from(Self::FILL_NONE));

        let mut def_2 = Fill::default();
        def_2.set_fill_type(String::from(Self::FILL_PATTERN_GRAY125));
        vec![def_1, def_2]
    }
    pub(crate) fn get_hash_code(&self)-> String {
        let start_color_hash_code = &self.start_color.get_hash_code();
        let end_color_hash_code = &self.end_color.get_hash_code();
        format!("{:x}", md5::compute(format!("{}{}{}{}",
            &self.fill_type,
            &self.rotation,
            if &self.fill_type != Self::FILL_NONE {start_color_hash_code} else {""},
            if &self.fill_type != Self::FILL_NONE {end_color_hash_code} else {""}
        )))
    }
}