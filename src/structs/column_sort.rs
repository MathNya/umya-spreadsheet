#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ColumnSort {
    pub(crate) col_num: u32,
    pub(crate) hash_code: String,
}
