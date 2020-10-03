#[derive(Default, Debug)]
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
    pub(crate) fn set_color_map(&mut self, value:Vec<String>) {
        self.color_map = value;
    }
    pub(crate) fn add_color_map<S: Into<String>>(&mut self, value:S) {
        self.color_map.push(value.into());
    }
}