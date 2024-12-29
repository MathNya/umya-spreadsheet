use super::Range;
use crate::{
    helper::{
        address::split_address,
        coordinate::index_from_coordinate,
        utils::compile_regex,
    },
    traits::{
        AdjustmentCoordinate,
        AdjustmentCoordinateWithSheet,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Address {
    sheet_name: Box<str>,
    range:      Range,
}

impl Address {
    #[must_use]
    pub fn get_sheet_name(&self) -> &str {
        &self.sheet_name
    }

    pub fn set_sheet_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.sheet_name = value.into().into_boxed_str();
        self
    }

    #[must_use]
    pub fn get_range(&self) -> &Range {
        &self.range
    }

    pub fn get_range_mut(&mut self) -> &mut Range {
        &mut self.range
    }

    pub fn set_range(&mut self, value: Range) -> &mut Self {
        self.range = value;
        self
    }

    pub fn set_address<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let org_value = value.into();
        let (sheet_name, range) = split_address(&org_value);
        self.range.set_range(range);
        if !sheet_name.is_empty() {
            self.sheet_name = sheet_name.into();
        }
        self
    }

    #[must_use]
    pub fn get_address(&self) -> String {
        self.get_address_crate(false)
    }

    pub(crate) fn get_address_ptn2(&self) -> String {
        self.get_address_crate(true)
    }

    pub(crate) fn get_address_crate(&self, is_ptn2: bool) -> String {
        let range = self.range.get_range();
        if self.sheet_name.is_empty() {
            return range;
        }
        let mut with_space_char = "";
        let mut sheet_name = self.sheet_name.clone();
        if sheet_name.contains(char::is_whitespace) {
            with_space_char = "'";
        }
        if is_ptn2 {
            if sheet_name.contains('!') {
                with_space_char = "'";
            }
            if sheet_name.contains('\'') {
                with_space_char = "'";
                sheet_name = sheet_name.replace('\'', "''").into_boxed_str();
            }
            if sheet_name.contains('"') {
                with_space_char = "'";
            }
            if with_space_char.is_empty()
                && compile_regex!(r"[^0-9a-zA-Z]")
                    .is_match(&sheet_name)
                    .unwrap_or(false)
            {
                with_space_char = "'";
            }
            if with_space_char.is_empty()
                && (None, None, None, None) != index_from_coordinate(&sheet_name)
            {
                with_space_char = "'";
            }
        }
        format!(
            "{}{}{}!{}",
            &with_space_char, sheet_name, &with_space_char, range
        )
    }
}
impl AdjustmentCoordinateWithSheet for Address {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        if &*self.sheet_name == sheet_name {
            self.range.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    fn adjustment_remove_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        if &*self.sheet_name == sheet_name {
            self.range.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    fn is_remove_coordinate_with_sheet(
        &self,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) -> bool {
        &*self.sheet_name == sheet_name
            && self.range.is_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            )
    }
}
