use super::Range;
use super::Column;

#[derive(Default, Debug)]
pub struct AutoFilter {
    range: Range,
    columns: Vec<Column>,
}
impl AutoFilter {
    pub fn get_range(&self)->&Range {
        &self.range
    }

    pub fn get_range_mut(&mut self)->&mut Range {
        &mut self.range
    }
    
    pub(crate) fn set_range<S: Into<String>>(&mut self, value:S) {
        let mut range = Range::default();
        range.set_range(value.into());
        self.range = range;
    }
}
