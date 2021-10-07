#[derive(Clone, Default, Debug)]
pub struct Theme {
    theme_name: String,
    color_scheme_name: String,
    color_map: Vec<String>,
}
impl Theme {
    pub fn get_theme_name(&self)-> &str {
        &self.theme_name
    }
    pub(crate) fn set_theme_name<S: Into<String>>(&mut self, value:S) {
        self.theme_name = value.into();
    }
    pub fn get_color_scheme_name(&self)-> &str {
        &self.color_scheme_name
    }
    pub(crate) fn set_color_scheme_name<S: Into<String>>(&mut self, value:S) {
        self.color_scheme_name = value.into();
    }
    pub fn get_color_map(&self)-> &Vec<String> {
        &self.color_map
    }
    pub fn get_color_map_mut(&mut self)-> &mut Vec<String> {
        &mut self.color_map
    }
    pub fn set_color_map(&mut self, value:Vec<String>) {
        self.color_map = value;
    }
    pub(crate) fn add_color_map<S: Into<String>>(&mut self, value:S) {
        self.color_map.push(value.into());
    }
    pub(crate) fn get_defalut_value() -> Theme {
        let mut def = Theme::default();
        def.set_theme_name("Office Theme");
        def.set_color_scheme_name("Office");
        def.add_color_map("FFFFFF"); // lt1
        def.add_color_map("000000"); // dk1
        def.add_color_map("EEECE1"); // lt2
        def.add_color_map("1F497D"); // dk2
        def.add_color_map("4F81BD"); // accent1
        def.add_color_map("C0504D"); // accent2
        def.add_color_map("9BBB59"); // accent3
        def.add_color_map("8064A2"); // accent4
        def.add_color_map("4BACC6"); // accent5
        def.add_color_map("F79646"); // accent6
        def.add_color_map("0000FF"); // accent7
        def.add_color_map("800080"); // accent8
        def
    }
}