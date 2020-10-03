const INDEXED_COLORS: &'static [&'static str] = &[
    "FF000000", //  System Colour #1 - Black
    "FFFFFFFF", //  System Colour #2 - White
    "FFFF0000", //  System Colour #3 - Red
    "FF00FF00", //  System Colour #4 - Green
    "FF0000FF", //  System Colour #5 - Blue
    "FFFFFF00", //  System Colour #6 - Yellow
    "FFFF00FF", //  System Colour #7- Magenta
    "FF00FFFF", //  System Colour #8- Cyan
    "FF800000", //  Standard Colour #9
    "FF008000", //  Standard Colour #10
    "FF000080", //  Standard Colour #11
    "FF808000", //  Standard Colour #12
    "FF800080", //  Standard Colour #13
    "FF008080", //  Standard Colour #14
    "FFC0C0C0", //  Standard Colour #15
    "FF808080", //  Standard Colour #16
    "FF9999FF", //  Chart Fill Colour #17
    "FF993366", //  Chart Fill Colour #18
    "FFFFFFCC", //  Chart Fill Colour #19
    "FFCCFFFF", //  Chart Fill Colour #20
    "FF660066", //  Chart Fill Colour #21
    "FFFF8080", //  Chart Fill Colour #22
    "FF0066CC", //  Chart Fill Colour #23
    "FFCCCCFF", //  Chart Fill Colour #24
    "FF000080", //  Chart Line Colour #25
    "FFFF00FF", //  Chart Line Colour #26
    "FFFFFF00", //  Chart Line Colour #27
    "FF00FFFF", //  Chart Line Colour #28
    "FF800080", //  Chart Line Colour #29
    "FF800000", //  Chart Line Colour #30
    "FF008080", //  Chart Line Colour #31
    "FF0000FF", //  Chart Line Colour #32
    "FF00CCFF", //  Standard Colour #33
    "FFCCFFFF", //  Standard Colour #34
    "FFCCFFCC", //  Standard Colour #35
    "FFFFFF99", //  Standard Colour #36
    "FF99CCFF", //  Standard Colour #37
    "FFFF99CC", //  Standard Colour #38
    "FFCC99FF", //  Standard Colour #39
    "FFFFCC99", //  Standard Colour #40
    "FF3366FF", //  Standard Colour #41
    "FF33CCCC", //  Standard Colour #42
    "FF99CC00", //  Standard Colour #43
    "FFFFCC00", //  Standard Colour #44
    "FFFF9900", //  Standard Colour #45
    "FFFF6600", //  Standard Colour #46
    "FF666699", //  Standard Colour #47
    "FF969696", //  Standard Colour #48
    "FF003366", //  Standard Colour #49
    "FF339966", //  Standard Colour #50
    "FF003300", //  Standard Colour #51
    "FF333300", //  Standard Colour #52
    "FF993300", //  Standard Colour #53
    "FF993366", //  Standard Colour #54
    "FF333399", //  Standard Colour #55
    "FF333333", //  Standard Colour #56
];

#[derive(Debug, Clone)]
pub struct Color {
    indexed: Option<usize>,
    theme_index: Option<usize>,
    argb: String,
    tint: f64,
}
impl Default for Color {
    fn default() -> Self {
        Self {
            indexed: None,
            theme_index: None,
            argb:"".into(),
            tint: 0f64
        }
    }
}
impl Color {
    // Colors
    pub const COLOR_BLACK: &'static str = "FF000000";
    pub const COLOR_WHITE: &'static str = "FFFFFFFF";
    pub const COLOR_RED: &'static str = "FFFF0000";
    pub const COLOR_DARKRED: &'static str = "FF800000";
    pub const COLOR_BLUE: &'static str = "FF0000FF";
    pub const COLOR_DARKBLUE: &'static str = "FF000080";
    pub const COLOR_GREEN: &'static str = "FF00FF00";
    pub const COLOR_DARKGREEN: &'static str = "FF008000";
    pub const COLOR_YELLOW: &'static str = "FFFFFF00";
    pub const COLOR_DARKYELLOW: &'static str = "FF808000";

    pub fn get_argb(&self)-> &str {
        &self.argb
    }
    pub fn set_argb<S: Into<String>>(&mut self, value:S) -> Result<(), &'static str> {
        self.indexed =None;
        self.theme_index = None;
        self.argb = value.into();
        Ok(())
    }
    pub(crate) fn is_set_indexed(&self)-> bool {
        match self.indexed {
            Some(_) => true,
            None => false
        }
    }
    pub fn get_indexed(&self)-> &Option<usize> {
        &self.indexed
    }
    pub fn set_indexed(&mut self, index:usize) -> Result<(), &'static str> {
        self.indexed = Some(index);
        self.theme_index = None;
        self.argb = match INDEXED_COLORS.get(index - 1) {
            Some(v) => {v.to_string()},
            None => {String::from("")}
        };
        Ok(())
    }
    pub(crate) fn is_set_theme_index(&self)-> bool {
        match self.theme_index {
            Some(_) => true,
            None => false
        }
    }
    pub fn get_theme_index(&self)-> &Option<usize> {
        &self.theme_index
    }
    pub fn set_theme_index(&mut self, index:usize, theme_color_map:&Vec<String>) -> Result<(), &'static str>  {
        self.indexed = None;
        self.theme_index = Some(index);
        self.argb = match theme_color_map.get(index) {
            Some(v) => {v.to_string()},
            None => {String::from("")}
        };
        Ok(())
    }
    pub(crate) fn set_theme_index_and_argb<S: Into<String>>(&mut self, index:usize, argb:S) {
        self.indexed = None;
        self.theme_index = Some(index);
        self.argb = argb.into();
    }
    pub fn get_tint(&self)-> &f64 {
        &self.tint
    }
    pub(crate) fn set_tint(&mut self, value:f64) -> Result<(), &'static str> {
        self.tint = value;
        Ok(())
    }
    pub(crate) fn get_hash_code(&self)-> String {
        format!("{:x}", md5::compute(format!("{}{}{}{}",
            match &self.indexed {Some(v)=> v.to_string(), None => "None".into()},
            match &self.theme_index {Some(v)=> v.to_string(), None => "None".into()},
            &self.argb,
            &self.tint
        )))
    }
}
