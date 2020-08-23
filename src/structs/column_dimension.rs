#[derive(Default, Debug)]
pub struct ColumnDimension {
    column_index: String,
    width: f32,
    auto_size: bool,
}
impl ColumnDimension {
    pub fn get_column_index(&self)-> &str {
        &self.column_index
    }
    pub(crate) fn set_column_index<S: Into<String>>(&mut self, value:S) {
        self.column_index = value.into();
    }
    pub fn get_width(&self)-> &f32 {
        &self.width
    }
    pub(crate) fn set_width(&mut self, value:f32) {
        self.width = value;
    }
    pub fn get_auto_size(&self)-> &bool {
        &self.auto_size
    }
    pub(crate) fn set_auto_size(&mut self, value:bool) {
        self.auto_size = value;
    }
}
