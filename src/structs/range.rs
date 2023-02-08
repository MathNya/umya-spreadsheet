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

        if coordinate_collection.is_empty() || coordinate_collection.len() > 2 {
            panic!("Non-standard coordinate");
        }

        if coordinate_collection.len() == 1 || coordinate_collection.len() == 2 {
            let coordinate_str = coordinate_collection[0].to_string();
            let (
                row,         //
                col,         //
                is_lock_col, //
                is_lock_row,
            ) = index_from_coordinate(coordinate_str);
            match row {
                Some(v) => {
                    let mut coordinate_start_col = ColumnReference::default();
                    coordinate_start_col.set_num(v);
                    coordinate_start_col.set_is_lock(is_lock_col.unwrap());
                    self.coordinate_start_col = Some(coordinate_start_col);
                }
                None => {}
            };
            match col {
                Some(v) => {
                    let mut coordinate_start_row = RowReference::default();
                    coordinate_start_row.set_num(v);
                    coordinate_start_row.set_is_lock(is_lock_row.unwrap());
                    self.coordinate_start_row = Some(coordinate_start_row);
                }
                None => {}
            }
        }

        if coordinate_collection.len() == 2 {
            let coordinate_str = coordinate_collection[1].to_string();
            let (
                row,         //
                col,         //
                is_lock_col, //
                is_lock_row,
            ) = index_from_coordinate(coordinate_str);
            match row {
                Some(v) => {
                    let mut coordinate_end_col = ColumnReference::default();
                    coordinate_end_col.set_num(v);
                    coordinate_end_col.set_is_lock(is_lock_col.unwrap());
                    self.coordinate_end_col = Some(coordinate_end_col);
                }
                None => {}
            };
            match col {
                Some(v) => {
                    let mut coordinate_end_row = RowReference::default();
                    coordinate_end_row.set_num(v);
                    coordinate_end_row.set_is_lock(is_lock_row.unwrap());
                    self.coordinate_end_row = Some(coordinate_end_row);
                }
                None => {}
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

    pub fn get_coordinate_start_col(&self) -> &Option<ColumnReference> {
        &self.coordinate_start_col
    }

    pub fn get_coordinate_start_col_mut(&mut self) -> &mut Option<ColumnReference> {
        &mut self.coordinate_start_col
    }

    pub fn get_coordinate_start_row(&self) -> &Option<RowReference> {
        &self.coordinate_start_row
    }

    pub fn get_coordinate_start_row_mut(&mut self) -> &mut Option<RowReference> {
        &mut self.coordinate_start_row
    }

    pub fn get_coordinate_end_col(&self) -> &Option<ColumnReference> {
        &self.coordinate_end_col
    }

    pub fn get_coordinate_end_col_mut(&mut self) -> &mut Option<ColumnReference> {
        &mut self.coordinate_end_col
    }

    pub fn get_coordinate_end_row(&self) -> &Option<RowReference> {
        &self.coordinate_end_row
    }

    pub fn get_coordinate_end_row_mut(&mut self) -> &mut Option<RowReference> {
        &mut self.coordinate_end_row
    }

    pub(crate) fn get_coordinate_start(&self) -> String {
        let mut coordinate_str = "".into();
        match &self.coordinate_start_col {
            Some(v) => {
                coordinate_str = v.get_coordinate();
            }
            None => {}
        };
        match &self.coordinate_start_row {
            Some(v) => {
                coordinate_str = format!("{}{}", coordinate_str, v.get_coordinate());
            }
            None => {}
        };
        coordinate_str
    }

    pub(crate) fn get_coordinate_end(&self) -> String {
        let mut coordinate_str = "".into();
        match &self.coordinate_end_col {
            Some(v) => {
                coordinate_str = v.get_coordinate();
            }
            None => {}
        };
        match &self.coordinate_end_row {
            Some(v) => {
                coordinate_str = format!("{}{}", coordinate_str, v.get_coordinate());
            }
            None => {}
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
        match &mut self.coordinate_start_col {
            Some(v) => {
                v.adjustment_insert_coordinate(root_col_num, offset_col_num);
            }
            None => {}
        }
        match &mut self.coordinate_start_row {
            Some(v) => {
                v.adjustment_insert_coordinate(root_row_num, offset_row_num);
            }
            None => {}
        }
        match &mut self.coordinate_end_col {
            Some(v) => {
                v.adjustment_insert_coordinate(root_col_num, offset_col_num);
            }
            None => {}
        }
        match &mut self.coordinate_end_row {
            Some(v) => {
                v.adjustment_insert_coordinate(root_row_num, offset_row_num);
            }
            None => {}
        }
    }

    pub(crate) fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        match &mut self.coordinate_start_col {
            Some(v) => {
                v.adjustment_remove_coordinate(root_col_num, offset_col_num);
            }
            None => {}
        }
        match &mut self.coordinate_start_row {
            Some(v) => {
                v.adjustment_remove_coordinate(root_row_num, offset_row_num);
            }
            None => {}
        }
        match &mut self.coordinate_end_col {
            Some(v) => {
                v.adjustment_remove_coordinate(root_col_num, offset_col_num);
            }
            None => {}
        }
        match &mut self.coordinate_end_row {
            Some(v) => {
                v.adjustment_remove_coordinate(root_row_num, offset_row_num);
            }
            None => {}
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
