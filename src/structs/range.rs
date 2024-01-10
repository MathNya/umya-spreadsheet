use super::ColumnReference;
use super::RowReference;
use helper::coordinate::*;

#[derive(Clone, Default, Debug)]
pub struct Range {
    coordinate_start_col: Option<ColumnReference>,
    coordinate_start_row: Option<RowReference>,
    coordinate_end_col: Option<ColumnReference>,
    coordinate_end_row: Option<RowReference>,
}

impl Range {
    pub fn set_range<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let org_value = value.into();
        let coordinate_collection: Vec<&str> = org_value.split(':').collect();

        assert!(
            matches!(coordinate_collection.len(), 1 | 2),
            "Non-standard coordinate"
        );

        let (
            row,         //
            col,         //
            is_lock_col, //
            is_lock_row,
        ) = index_from_coordinate(coordinate_collection[0]);
        if let Some(v) = row {
            let mut coordinate_start_col = ColumnReference::default();
            coordinate_start_col.set_num(v);
            coordinate_start_col.set_is_lock(is_lock_col.unwrap());
            self.coordinate_start_col = Some(coordinate_start_col);
        };
        if let Some(v) = col {
            let mut coordinate_start_row = RowReference::default();
            coordinate_start_row.set_num(v);
            coordinate_start_row.set_is_lock(is_lock_row.unwrap());
            self.coordinate_start_row = Some(coordinate_start_row);
        }

        if coordinate_collection.len() == 2 {
            let (
                row,         //
                col,         //
                is_lock_col, //
                is_lock_row,
            ) = index_from_coordinate(coordinate_collection[1]);
            if let Some(v) = row {
                let mut coordinate_end_col = ColumnReference::default();
                coordinate_end_col.set_num(v);
                coordinate_end_col.set_is_lock(is_lock_col.unwrap());
                self.coordinate_end_col = Some(coordinate_end_col);
            };
            if let Some(v) = col {
                let mut coordinate_end_row = RowReference::default();
                coordinate_end_row.set_num(v);
                coordinate_end_row.set_is_lock(is_lock_row.unwrap());
                self.coordinate_end_row = Some(coordinate_end_row);
            }
        }
        self
    }

    pub fn get_range(&self) -> String {
        let mut result = self.get_coordinate_start();
        if self.coordinate_end_col.is_some() || self.coordinate_end_row.is_some() {
            result = format!("{}:{}", result, &self.get_coordinate_end());
        }
        result
    }

    pub fn get_coordinate_start_col(&self) -> Option<&ColumnReference> {
        self.coordinate_start_col.as_ref()
    }

    pub fn get_coordinate_start_col_mut(&mut self) -> Option<&mut ColumnReference> {
        self.coordinate_start_col.as_mut()
    }

    pub fn get_coordinate_start_row(&self) -> Option<&RowReference> {
        self.coordinate_start_row.as_ref()
    }

    pub fn get_coordinate_start_row_mut(&mut self) -> Option<&mut RowReference> {
        self.coordinate_start_row.as_mut()
    }

    pub fn get_coordinate_end_col(&self) -> Option<&ColumnReference> {
        self.coordinate_end_col.as_ref()
    }

    pub fn get_coordinate_end_col_mut(&mut self) -> Option<&mut ColumnReference> {
        self.coordinate_end_col.as_mut()
    }

    pub fn get_coordinate_end_row(&self) -> Option<&RowReference> {
        self.coordinate_end_row.as_ref()
    }

    pub fn get_coordinate_end_row_mut(&mut self) -> Option<&mut RowReference> {
        self.coordinate_end_row.as_mut()
    }

    pub(crate) fn get_coordinate_start(&self) -> String {
        let mut coordinate_str = "".into();
        if let Some(v) = &self.coordinate_start_col {
            coordinate_str = v.get_coordinate();
        };
        if let Some(v) = &self.coordinate_start_row {
            coordinate_str = format!("{}{}", coordinate_str, v.get_coordinate());
        };
        coordinate_str
    }

    pub(crate) fn get_coordinate_end(&self) -> String {
        let mut coordinate_str = "".into();
        if let Some(v) = &self.coordinate_end_col {
            coordinate_str = v.get_coordinate();
        };
        if let Some(v) = &self.coordinate_end_row {
            coordinate_str = format!("{}{}", coordinate_str, v.get_coordinate());
        };
        coordinate_str
    }

    pub(crate) fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if let Some(v) = &mut self.coordinate_start_col {
            v.adjustment_insert_coordinate(root_col_num, offset_col_num);
        }
        if let Some(v) = &mut self.coordinate_start_row {
            v.adjustment_insert_coordinate(root_row_num, offset_row_num);
        }
        if let Some(v) = &mut self.coordinate_end_col {
            v.adjustment_insert_coordinate(root_col_num, offset_col_num);
        }
        if let Some(v) = &mut self.coordinate_end_row {
            v.adjustment_insert_coordinate(root_row_num, offset_row_num);
        }
    }

    pub(crate) fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if let Some(v) = &mut self.coordinate_start_col {
            v.adjustment_remove_coordinate(root_col_num, offset_col_num);
        }
        if let Some(v) = &mut self.coordinate_start_row {
            v.adjustment_remove_coordinate(root_row_num, offset_row_num);
        }
        if let Some(v) = &mut self.coordinate_end_col {
            v.adjustment_remove_coordinate(root_col_num, offset_col_num);
        }
        if let Some(v) = &mut self.coordinate_end_row {
            v.adjustment_remove_coordinate(root_row_num, offset_row_num);
        }
    }

    pub(crate) fn is_remove(
        &self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) -> bool {
        let start_col_result = match &self.coordinate_start_col {
            Some(v) => v.is_remove(root_col_num, offset_col_num),
            None => false,
        };
        let start_row_result = match &self.coordinate_start_row {
            Some(v) => v.is_remove(root_row_num, offset_row_num),
            None => false,
        };
        let end_col_result = match &self.coordinate_end_col {
            Some(v) => v.is_remove(root_col_num, offset_col_num),
            None => false,
        };
        let end_row_result = match &self.coordinate_end_row {
            Some(v) => v.is_remove(root_row_num, offset_row_num),
            None => false,
        };
        start_col_result && start_row_result && end_col_result && end_row_result
    }
}
