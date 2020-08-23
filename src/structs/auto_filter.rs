use super::column::Column;

#[derive(Default, Debug)]
pub struct AutoFilter {
    pub(crate) range: String,
    pub(crate) columns: Vec<Column>,
}
