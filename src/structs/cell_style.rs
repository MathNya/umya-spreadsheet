use super::style::Style;

#[derive(Default, Debug)]
pub struct CellStyle {
    name: String,
    builtin_id: usize,
    style: Style,
}
impl CellStyle {
    pub fn get_name(&self)-> &str {
        &self.name
    }
    pub(crate) fn set_name<S: Into<String>>(&mut self, value:S) {
        self.name = value.into();
    }
    pub fn get_builtin_id(&self)-> &usize {
        &self.builtin_id
    }
    pub(crate) fn set_builtin_id(&mut self, value:usize) {
        self.builtin_id = value;
    }
    pub fn get_style(&self)-> &Style {
        &self.style
    }
    pub(crate) fn set_style(&mut self, value:Style) {
        self.style = value;
    }
}