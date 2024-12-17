use super::Range;
use crate::traits::AdjustmentCoordinate;

#[derive(Clone, Default, Debug)]
pub struct AutoFilter {
    range: Range,
}

impl AutoFilter {
    #[inline]
    pub fn get_range(&self) -> &Range {
        &self.range
    }

    #[inline]
    pub fn get_range_mut(&mut self) -> &mut Range {
        &mut self.range
    }

    #[inline]
    pub(crate) fn set_range<S: Into<String>>(&mut self, value: S) {
        let mut range = Range::default();
        range.set_range(value.into());
        self.range = range;
    }
}
impl AdjustmentCoordinate for AutoFilter {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.range.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.range.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );
    }
}
