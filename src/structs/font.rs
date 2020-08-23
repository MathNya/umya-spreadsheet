use super::color::Color;

#[derive(Default, Debug, Clone)]
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
    color_index: usize,
    charset: usize,
    family: usize,
    scheme: String,
}
impl Font {
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

    pub fn get_name(&self)-> &str {
        &self.name
    }
    pub(crate) fn set_name<S: Into<String>>(&mut self, value:S) {
        self.name = value.into();
    }
    pub fn get_size(&self)-> &usize {
        &self.size
    }
    pub(crate) fn set_size(&mut self, value:usize) {
        self.size = value;
    }
    pub fn get_italic(&self)-> &bool {
        &self.italic
    }
    pub(crate) fn set_italic(&mut self, value:bool) {
        self.italic = value;
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
    pub fn get_color_index(&self)-> &usize {
        &self.color_index
    }
    pub(crate) fn set_color_index(&mut self, value:usize) {
        self.color_index = value;
    }
    pub fn get_strikethrough(&self)-> &bool {
        &self.strikethrough
    }
    pub(crate) fn set_strikethrough(&mut self, value:bool) {
        self.strikethrough = value;
    }
    pub fn get_charset(&self)-> &usize {
        &self.charset
    }
    pub(crate) fn set_charset(&mut self, value:usize) {
        self.charset = value;
    }
    pub fn get_family(&self)-> &usize {
        &self.family
    }
    pub(crate) fn set_family(&mut self, value:usize) {
        self.family = value;
    }
    pub fn get_scheme(&self)-> &String {
        &self.scheme
    }
    pub(crate) fn set_scheme<S: Into<String>>(&mut self, value:S) {
        self.scheme = value.into();
    }
    pub(crate) fn get_defalut_fonts(theme_color_map:&Vec<String>) -> Vec<Font> {
        let mut def_1 = Font::default();
        def_1.set_size(11);
        def_1.set_name("ＭＳ Ｐゴシック");
        def_1.get_color_mut().set_theme_index(1, theme_color_map);
        def_1.set_family(2);
        def_1.set_scheme("minor");

        let mut def_2 = Font::default();
        def_2.set_size(6);
        def_2.set_name("ＭＳ Ｐゴシック");
        def_2.set_family(3);
        def_2.set_charset(128);
        def_2.set_scheme("minor");
        vec![def_1, def_2]
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