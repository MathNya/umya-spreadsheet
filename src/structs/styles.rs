use super::Style;
use std::collections::HashMap;
use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct Styles {
    pub(crate) index: Vec<Style>,
}
impl Styles {
    pub(crate) fn get_collection(&self)-> &Vec<Style> {
        &self.index
    }

    pub(crate) fn get_collection_mut(&mut self)-> &mut Vec<Style> {
        &mut self.index
    }

    pub(crate) fn get_collection_to_hashmap(&self)-> HashMap<String, &Style> {
        let mut result = HashMap::default();
        for style in &self.index {
            let coordinate = style.get_coordinate().get_coordinate();
            result.insert(coordinate, style);
        }
        result
    }

    pub(crate) fn get_collection_by_row(&self, row_num:&usize)-> BTreeMap<usize, &Style> {
        let mut result = BTreeMap::default();
        for style in &self.index {
            if row_num == style.get_coordinate().get_row_num() {
                result.insert(style.get_coordinate().get_col_num().clone(), style);
            }
        }
        result
    }

    pub(crate) fn has(&self, col_num:&usize, row_num:&usize)-> bool {
        for style in &self.index {
            if style.get_coordinate().is_mine(col_num, row_num) {
                return true;
            }
        }
        false
    }

    pub(crate) fn get(&self, col_num:&usize, row_num:&usize)-> Option<&Style> {
        for style in &self.index {
            if style.get_coordinate().is_mine(col_num, row_num) {
                return Some(style);
            }
        }
        None
    }

    pub(crate) fn get_mut(&mut self, col_num:&usize, row_num:&usize)-> Option<&mut Style> {
        for style in &mut self.index {
            if style.get_coordinate().is_mine(col_num, row_num) {
                return Some(style);
            }
        }
        None
    }

    pub(crate) fn add(&mut self, style:Style) {
        self.index.push(style);
    }
}