use super::column::Column;

#[derive(Default, Debug)]
pub struct AutoFilter {
    range: String,
    columns: Vec<Column>,
}
impl AutoFilter {
    pub fn get_range(&self)->&str {
        &self.range
    }
    pub(crate) fn set_range<S: Into<String>>(&mut self, value:S) {
        self.range = value.into();
    }
}
