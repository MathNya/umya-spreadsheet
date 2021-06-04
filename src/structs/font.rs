use super::Color;

#[derive(Debug, Clone)]
pub struct Font {
    name: String,
    size: usize,
    bold: bool,
    italic: bool,
    superscript: bool,
    subscript: bool,
    underline: String,
    strikethrough: bool,
    color: Color,
    charset: usize,
    family: usize,
    scheme: String,
}
impl Default for Font {
    fn default() -> Self {
        Self {
            name: "".into(),
            size: 0,
            bold: false,
            italic: false,
            superscript: false,
            subscript: false,
            underline: Font::UNDERLINE_NONE.to_string(),
            strikethrough: false,
            color: Color::default(),
            charset: 0,
            family: 0,
            scheme: "".into()
        }
    }
}
impl Font {
    // Charset
    pub const CHARSET_ANSI: usize = 0;
    pub const CHARSET_DEFAULT: usize = 1;
    pub const CHARSET_SYMBOL: usize = 2;
    pub const CHARSET_SHIFTJIS: usize = 128;
    pub const CHARSET_HANGEUL: usize = 129;
    pub const CHARSET_HANGUL: usize = 129;
    pub const CHARSET_GB2312: usize = 134;
    pub const CHARSET_CHINESEBIG5: usize = 136;
    pub const CHARSET_OEM: usize = 255;
    pub const CHARSET_JOHAB: usize = 130;
    pub const CHARSET_HEBREW: usize = 177;
    pub const CHARSET_ARABIC: usize = 178;
    pub const CHARSET_GREEK: usize = 161;
    pub const CHARSET_TURKISH: usize = 162;
    pub const CHARSET_VIETNAMESE: usize = 163;
    pub const CHARSET_THAI: usize = 222;
    pub const CHARSET_EASTEUROPE: usize = 238;
    pub const CHARSET_RUSSIAN: usize = 204;
    pub const CHARSET_MAC: usize = 77;
    pub const CHARSET_BALTIC: usize = 186;

    // Underline types
    pub const UNDERLINE_NONE: &'static str = "none";
    pub const UNDERLINE_DOUBLE: &'static str = "double";
    pub const UNDERLINE_DOUBLEACCOUNTING: &'static str = "doubleAccounting";
    pub const UNDERLINE_SINGLE: &'static str = "single";
    pub const UNDERLINE_SINGLEACCOUNTING: &'static str = "singleAccounting";

    pub fn get_name(&self)-> &str {
        &self.name
    }

    pub fn set_name<S: Into<String>>(&mut self, value:S)-> &mut Font {
        self.name = value.into();
        self
    }

    pub fn get_size(&self)-> &usize {
        &self.size
    }

    pub fn set_size(&mut self, value:usize)-> &mut Font {
        self.size = value;
        self
    }

    pub fn get_bold(&self)-> &bool {
        &self.bold
    }

    pub fn set_bold(&mut self, value:bool)-> &mut Font {
        self.bold = value;
        self
    }

    pub fn get_italic(&self)-> &bool {
        &self.italic
    }

    pub fn set_italic(&mut self, value:bool)-> &mut Font {
        self.italic = value;
        self
    }

    pub fn get_color(&self)-> &Color {
        &self.color
    }

    pub fn get_color_mut(&mut self)-> &mut Color {
        &mut self.color
    }

    pub fn set_color(&mut self, value:Color)-> &mut Font {
        self.color = value;
        self
    }

    pub fn get_underline(&self)-> &str {
        &self.underline
    }

    pub fn set_underline<S: Into<String>>(&mut self, value:S)-> &mut Font {
        self.underline = value.into();
        self
    }

    pub fn get_strikethrough(&self)-> &bool {
        &self.strikethrough
    }

    pub fn set_strikethrough(&mut self, value:bool)-> &mut Font {
        self.strikethrough = value;
        self
    }

    pub fn get_charset(&self)-> &usize {
        &self.charset
    }

    pub fn set_charset(&mut self, value:usize)-> &mut Font {
        self.charset = value;
        self
    }

    pub fn get_family(&self)-> &usize {
        &self.family
    }

    pub fn set_family(&mut self, value:usize)-> &mut Font {
        self.family = value;
        self
    }

    pub fn get_scheme(&self)-> &String {
        &self.scheme
    }

    pub fn set_scheme<S: Into<String>>(&mut self, value:S)-> &mut Font {
        self.scheme = value.into();
        self
    }

    pub(crate) fn get_defalut_value() -> Font {
        let mut def = Font::default();
        def.set_size(11);
        def.set_name("Calibri");
        def.get_color_mut().set_theme_index_and_argb(1, "000000");
        def.set_family(2);
        def.set_scheme("minor");
        def
    }

    pub(crate) fn get_hash_code(&self)-> String
    {
        format!("{:x}", md5::compute(format!("{}{}{}{}{}{}{}{}{}{}{}{}",
            &self.name,
            &self.size,
            if self.bold {"t"} else {"f"},
            if self.italic {"t"} else {"f"},
            if self.superscript {"t"} else {"f"},
            if self.subscript {"t"} else {"f"},
            &self.underline,
            if self.strikethrough {"t"} else {"f"},
            self.charset,
            self.family,
            self.scheme,
            &self.color.get_hash_code()
        )))
    }
}