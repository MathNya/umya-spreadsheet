#[derive(Clone, Debug)]
pub struct Alignment {
    horizontal: String,
    vertical: String,
    text_rotation: i32,
    wrap_text: bool,
    shrink_to_fit: bool,
    indent: i32,
    read_order: i32,
}
impl Default for Alignment {
    fn default() -> Self {
        Self {
            horizontal: Alignment::HORIZONTAL_GENERAL.to_string(),
            vertical: "".into(),
            text_rotation: 0,
            wrap_text: false,
            shrink_to_fit: false,
            indent: 0,
            read_order:0
        }
    }
}
impl Alignment {
    // Horizontal alignment styles
    pub const HORIZONTAL_GENERAL: &'static str = "general";
    pub const HORIZONTAL_LEFT: &'static str = "left";
    pub const HORIZONTAL_RIGHT: &'static str = "right";
    pub const HORIZONTAL_CENTER: &'static str = "center";
    pub const HORIZONTAL_CENTER_CONTINUOUS: &'static str = "centerContinuous";
    pub const HORIZONTAL_JUSTIFY: &'static str = "justify";
    pub const HORIZONTAL_FILL: &'static str = "fill";
    pub const HORIZONTAL_DISTRIBUTED: &'static str = "distributed"; // Excel2007 only
    
    // Vertical alignment styles
    pub const VERTICAL_BOTTOM: &'static str = "bottom";
    pub const VERTICAL_TOP: &'static str = "top";
    pub const VERTICAL_CENTER: &'static str = "center";
    pub const VERTICAL_JUSTIFY: &'static str = "justify";
    pub const VERTICAL_DISTRIBUTED: &'static str = "distributed"; // Excel2007 only
    
    // Read order
    pub const READORDER_CONTEXT: usize = 0;
    pub const READORDER_LTR: usize = 1;
    pub const READORDER_RTL: usize = 2;

    pub fn get_horizontal(&self)-> &str {
        &self.horizontal
    }
    pub(crate) fn set_horizontal<S: Into<String>>(&mut self, value:S) {
        self.horizontal = value.into();
    }
    pub fn get_vertical(&self)-> &str {
        &self.vertical
    }
    pub(crate) fn set_vertical<S: Into<String>>(&mut self, value:S) {
        self.vertical = value.into();
    }
    pub(crate) fn is_empty(&self)-> bool {
        if &self.horizontal != Alignment::HORIZONTAL_GENERAL {
            return false;
        }
        if &self.vertical != "" {
            return false;
        }
        if &self.text_rotation != &0 {
            return false;
        }
        true
    }
    pub(crate) fn get_hash_code(&self)-> String {
        format!("{:x}", md5::compute(format!("{}{}{}{}{}{}{}",
        &self.horizontal,
        &self.vertical,
        &self.text_rotation,
        if self.wrap_text {"t"} else {"f"},
        if self.shrink_to_fit {"t"} else {"f"},
        &self.indent,
        &self.read_order
        )))
    }
}