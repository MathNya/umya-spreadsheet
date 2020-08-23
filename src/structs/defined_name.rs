#[derive(Default, Debug)]
pub struct DefinedName {
    name: String,
    worksheet: String,
    value: String,
    local_only: bool,
    is_formula: bool,
}
impl DefinedName {
    pub fn get_name(&self)-> &str {
        &self.name
    }
    pub(crate) fn set_name<S: Into<String>>(&mut self, value:S) {
        self.name = value.into();
    }
    pub fn get_worksheet(&self)-> &str {
        &self.worksheet
    }
    pub(crate) fn set_worksheet<S: Into<String>>(&mut self, value:S) {
        self.worksheet = value.into();
    }
    pub fn get_value(&self)-> &str {
        &self.value
    }
    pub(crate) fn set_value<S: Into<String>>(&mut self, value:S) {
        self.value = value.into();
    }
    pub fn get_local_only(&self)-> &bool {
        &self.local_only
    }
    pub(crate) fn set_local_only(&mut self, value:bool) {
        self.local_only = value;
    }
    pub fn get_is_formula(&self)-> &bool {
        &self.is_formula
    }
    pub(crate) fn set_is_formula(&mut self, value:bool) {
        self.is_formula = value;
    }
}