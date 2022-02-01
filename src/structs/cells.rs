use super::Cell;
use super::CellValue;
use super::Style;
use helper::range::*;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
pub struct Cells {
    index: Vec<Cell>,
    default_cell_value: CellValue,
    default_style: Style,
}
impl Cells {
    pub(crate) fn get_collection(&self) -> &Vec<Cell> {
        &self.index
    }

    pub(crate) fn get_collection_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.index
    }

    pub(crate) fn get_collection_to_hashmap(&self) -> HashMap<String, &Cell> {
        let mut result = HashMap::default();
        for cell in &self.index {
            let coordinate = cell.get_coordinate().get_coordinate();
            result.insert(coordinate, cell);
        }
        result
    }

    pub(crate) fn get_collection_by_row(&self, row_num: &u32) -> BTreeMap<u32, &Cell> {
        let mut result = BTreeMap::default();
        for cell in &self.index {
            if row_num == cell.get_coordinate().get_row_num() {
                result.insert(cell.get_coordinate().get_col_num().clone(), cell);
            }
        }
        result
    }

    pub(crate) fn get_collection_by_column(&self, column_num: &u32) -> BTreeMap<u32, &Cell> {
        let mut result = BTreeMap::default();
        for cell in &self.index {
            if column_num == cell.get_coordinate().get_col_num() {
                result.insert(cell.get_coordinate().get_row_num().clone(), cell);
            }
        }
        result
    }

    pub(crate) fn get_highest_row_and_column(&self) -> HashMap<&str, &u32> {
        let mut col_max: &u32 = &0;
        let mut row_max: &u32 = &0;
        for cell in &self.index {
            if cell.get_coordinate().get_col_num() > &col_max {
                col_max = cell.get_coordinate().get_col_num();
            }
            if cell.get_coordinate().get_row_num() > &row_max {
                row_max = cell.get_coordinate().get_row_num();
            }
        }
        let mut result = HashMap::new();
        result.insert("column", col_max);
        result.insert("row", row_max);
        result
    }

    pub(crate) fn has(&self, col_num: &u32, row_num: &u32) -> bool {
        for cell in &self.index {
            if cell.get_coordinate().is_mine(col_num, row_num) {
                return true;
            }
        }
        false
    }

    pub(crate) fn get(&self, col_num: &u32, row_num: &u32) -> Option<&Cell> {
        for cell in &self.index {
            if cell.get_coordinate().is_mine(col_num, row_num) {
                return Some(cell);
            }
        }
        None
    }

    pub(crate) fn get_crate(&mut self, col_num: &u32, row_num: &u32) -> Option<&mut Cell> {
        for cell in &mut self.index {
            if cell.get_coordinate().is_mine(col_num, row_num) {
                return Some(cell);
            }
        }
        None
    }

    pub(crate) fn get_mut(&mut self, col_num: &u32, row_num: &u32) -> &mut Cell {
        if self.has(col_num, row_num) == false {
            let mut cell = Cell::default();
            cell.get_coordinate_mut().set_col_num(col_num.clone());
            cell.get_coordinate_mut().set_row_num(row_num.clone());
            self.add(cell);
        }
        self.get_crate(col_num, row_num).unwrap()
    }

    pub(crate) fn get_cell_value(&self, col_num: &u32, row_num: &u32) -> &CellValue {
        for cell in &self.index {
            if cell.get_coordinate().is_mine(col_num, row_num) {
                return cell.get_cell_value();
            }
        }
        &self.default_cell_value
    }

    pub(crate) fn get_style(&self, col_num: &u32, row_num: &u32) -> &Style {
        for cell in &self.index {
            if cell.get_coordinate().is_mine(col_num, row_num) {
                return cell.get_style();
            }
        }
        &self.default_style
    }

    pub(crate) fn set(&mut self, cell: Cell) -> &mut Self {
        let col_num = cell.get_coordinate().get_col_num();
        let row_num = cell.get_coordinate().get_row_num();
        self.index
            .retain(|x| !x.get_coordinate().is_mine(col_num, row_num));
        self.add(cell);
        self
    }

    pub(crate) fn add(&mut self, cell: Cell) {
        self.index.push(cell);
    }

    pub(crate) fn get_cell_value_by_range<S: Into<String>>(&self, range: S) -> Vec<&CellValue> {
        let mut result: Vec<&CellValue> = Vec::new();
        let range_upper = range.into().to_uppercase();
        let coordinate_list = get_coordinate_list(&range_upper);
        for (col_num, row_num) in coordinate_list {
            result.push(self.get_cell_value(&col_num, &row_num));
        }
        result
    }
}
