use super::color::Color;

#[derive(Default, Debug, Clone)]
pub struct Fill {
    startcolor_index: usize,
    endcolor_index: usize,
    fill_type: String,
    rotation: i32,
    start_color: Option<Color>,
    end_color: Option<Color>
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
    pub fn get_start_color(&self)-> &Option<Color> {
        &self.start_color
    }
    pub fn get_start_color_mut(&mut self)-> &mut Color {
        match self.start_color {
            Some(_) => {},
            None => self.start_color = Some(Color::default())
        }
        self.start_color.as_mut().unwrap()
    }
    pub(crate) fn set_start_color(&mut self, value:Color) {
        self.start_color = Some(value);
    }
    pub fn get_end_color(&self)-> &Option<Color> {
        &self.end_color
    }
    pub fn get_end_color_mut(&mut self)-> &mut Color {
        match self.end_color {
            Some(_) => {},
            None => self.end_color = Some(Color::default())
        }
        self.end_color.as_mut().unwrap()
    }
    pub(crate) fn set_end_color(&mut self, value:Color) {
        self.end_color = Some(value);
    }
    pub(crate) fn get_defalut_value() -> Fill {
        let mut def = Fill::default();
        def.set_fill_type(String::from(Self::FILL_NONE));
        def
    }
    pub(crate) fn get_hash_code(&self)-> String {
        format!("{:x}", md5::compute(format!("{}{}{}{}",
            &self.fill_type,
            &self.rotation,
            match &self.start_color {Some(v) => {v.get_hash_code()}, None => {"None".into()}},
            match &self.end_color {Some(v) => {v.get_hash_code()}, None => {"None".into()}},
        )))
    }
}