use super::cell::Cell;
use std::collections::HashMap;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct Cells {
    index: Vec<Cell>,
}
impl Cells {
    pub(crate) fn get_collection(&self)-> &Vec<Cell> {
        &self.index
    }

    pub(crate) fn get_collection_to_hashmap(&self)-> HashMap<String, &Cell> {
        let mut result = HashMap::default();
        for cell in &self.index {
            let coordinate = cell.get_coordinate();
            result.insert(coordinate, cell);
        }
        result
    }

    pub(crate) fn get_collection_by_row(&self, row_num:&usize)-> BTreeMap<usize, &Cell> {
        let mut result = BTreeMap::default();
        for cell in &self.index {
            if row_num == cell.get_row_num() {
                result.insert(cell.get_col_num().clone(), cell);
            }
        }
        result
    }

    pub(crate) fn get_highest_row_and_column(&self)-> HashMap<&str, &usize> {
        let mut col_max:&usize = &0;
        let mut row_max:&usize = &0;
        for cell in &self.index {
            if cell.get_col_num() > &col_max {
                col_max = cell.get_col_num();
            }
            if cell.get_row_num() > &row_max {
                row_max = cell.get_row_num();
            }
        }
        let mut result = HashMap::new();
        result.insert("column", col_max);
        result.insert("row", row_max);
        result
    }

    pub(crate) fn has(&self, col_num:&usize, row_num:&usize)-> bool {
        for cell in &self.index {
            if cell.is_mine(col_num, row_num) {
                return true;
            }
        }
        false
    }

    pub(crate) fn get_index(&self)->&Vec<Cell> {
        &self.index
    }

    pub(crate) fn get(&self, col_num:&usize, row_num:&usize)-> Option<&Cell> {
        for cell in &self.index {
            if cell.is_mine(col_num, row_num) {
                return Some(cell);
            }
        }
        None
    }

    pub(crate) fn get_mut(&mut self, col_num:&usize, row_num:&usize)-> Option<&mut Cell> {
        for mut cell in &mut self.index {
            if cell.is_mine(col_num, row_num) {
                return Some(cell);
            }
        }
        None
    }

    pub(crate) fn add(&mut self, cell:Cell) {
        self.index.push(cell);
    }
}