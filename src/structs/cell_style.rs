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

    pub fn set_name<S: Into<String>>(&mut self, value:S)-> &mut CellStyle {
        self.name = value.into();
        self
    }

    pub fn get_builtin_id(&self)-> &usize {
        &self.builtin_id
    }

    pub fn set_builtin_id(&mut self, value:usize)-> &mut CellStyle {
        self.builtin_id = value;
        self
    }

    pub fn get_style(&self)-> &Style {
        &self.style
    }

    pub fn get_style_mut(&mut self)-> &mut Style {
        &mut self.style
    }

    pub fn set_style(&mut self, value:Style)-> &mut CellStyle {
        self.style = value;
        self
    }
}