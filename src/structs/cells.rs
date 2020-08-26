use super::cell::Cell;
use std::collections::HashMap; 
use super::super::helper::coordinate::*;

#[derive(Default, Debug)]
pub struct Cells {
    pub(crate) index: HashMap<String, Cell>,
}
impl Cells {
    pub fn get_collection(&self)-> &HashMap<String, Cell>
    {
        &self.index
    }
    pub(crate) fn get_highest_row_and_column(&self)-> HashMap<&str, String>
    {
        let mut col_max:String = String::from("1A");
        let mut col_max_org:String = String::from("A");
        let mut row_max:i32 = 0;
        for (coordinate, _) in &self.index {
            let cfs = coordinate_from_string(coordinate);
            let col_string = cfs.get(0).unwrap();
            let row = cfs.get(1).unwrap().parse().unwrap();

            if row > row_max {
                row_max = row;
            }
            let col = format!("{}{}", col_string.len(), col_string);
            if col > col_max {
                col_max = col;
                col_max_org = col_string.to_string();
            }
        }
        let mut result = HashMap::new();
        result.insert("row", row_max.to_string());
        result.insert("column", col_max_org);
        result
    }
    pub fn get_coordinates(&self)-> Vec<String>
    {
        let mut result = Vec::new();
        for (coordinate, _) in &self.index {
            result.push(coordinate.clone());
        }
        result
    }
    pub fn has(&self, coordinate:&String)-> bool
    {
        match self.index.get(coordinate) {
            Some(_) => { true },
            None => { false }
        }
    }
    pub fn get_index(&self)->&HashMap<String, Cell>
    {
        &self.index
    }
    pub fn get(&self, coordinate:&String) -> Result<&Cell, &'static str> {
        match self.index.get(coordinate) {
            Some(v) => return Ok(v),
            None => return Err("Not found.")
        }
    }
    pub fn get_mut(&mut self, coordinate:&String) -> &mut Cell
    {
        self.index.get_mut(coordinate).unwrap()
    }
    pub(crate) fn add(&mut self, coordinate:&String, cell:Cell)
    {
        self.index.insert(coordinate.clone(), cell);
    }
}