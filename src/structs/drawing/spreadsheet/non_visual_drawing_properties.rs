//p:cNvPr
#[derive(Default, Debug, Clone)]
pub struct NonVisualDrawingProperties  {
    name: String,
    id: String,
}
impl NonVisualDrawingProperties  {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name<S: Into<String>>(&mut self, value:S) {
        self.name = value.into();
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn set_id<S: Into<String>>(&mut self, value:S) {
        self.id = value.into();
    }
}
