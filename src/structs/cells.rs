use super::Cell;
use super::CellValue;
use super::Style;
use hashbrown::HashMap;
use helper::coordinate::*;
use helper::range::*;

#[derive(Clone, Default, Debug)]
pub struct Cells {
    map: HashMap<(u32, u32), Cell>,
    default_cell_value: CellValue,
    default_style: Style,
}
impl Cells {
    pub fn get_collection(&self) -> Vec<&Cell> {
        self.map.values().collect()
    }

    pub(crate) fn get_collection_mut(&mut self) -> Vec<&mut Cell> {
        self.map.values_mut().collect()
    }

    pub fn get_collection_to_hashmap(&self) -> &HashMap<(u32, u32), Cell> {
        &self.map
    }

    pub fn get_collection_by_column(&self, column_num: &u32) -> Vec<&Cell> {
        self.map
            .values()
            .filter(|k| k.get_coordinate().get_col_num() == column_num)
            .collect()
    }

    pub fn get_collection_by_row(&self, row_num: &u32) -> Vec<&Cell> {
        self.map
            .values()
            .filter(|k| k.get_coordinate().get_row_num() == row_num)
            .collect()
    }

    pub fn get_collection_by_column_to_hashmap(&self, column_num: &u32) -> HashMap<u32, &Cell> {
        self.map
            .iter()
            .filter(|(k, _v)| &k.1 == column_num)
            .map(|(k, v)| (k.0, v))
            .collect()
    }

    pub fn get_collection_by_row_to_hashmap(&self, row_num: &u32) -> HashMap<u32, &Cell> {
        self.map
            .iter()
            .filter(|(k, _v)| &k.0 == row_num)
            .map(|(k, v)| (k.1, v))
            .collect()
    }

    pub(crate) fn get_collection_to_hashmap_mut(&mut self) -> &mut HashMap<(u32, u32), Cell> {
        &mut self.map
    }

    pub fn get_highest_column_and_row(&self) -> (u32, u32) {
        let mut col_max: u32 = 0;
        let mut row_max: u32 = 0;
        for key in self.map.keys() {
            if key.1 > col_max {
                col_max = key.1;
            }
            if key.0 > row_max {
                row_max = key.0;
            }
        }
        (col_max, row_max)
    }

    /// Has Hyperlink
    pub fn has_hyperlink(&self) -> bool {
        self.map.values().any(|c| c.get_hyperlink().is_some())
    }

    pub fn get<T>(&self, coordinate: T) -> Option<&Cell>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.map.get(&(row.to_owned(), col.to_owned()))
    }

    pub(crate) fn get_mut<T>(&mut self, coordinate: T) -> &mut Cell
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.map
            .entry((row.to_owned(), col.to_owned()))
            .or_insert_with(|| {
                let mut c = Cell::default();
                c.get_coordinate_mut().set_col_num(col);
                c.get_coordinate_mut().set_row_num(row);
                c
            })
    }

    pub fn get_cell_value<T>(&self, coordinate: T) -> &CellValue
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.map
            .get(&(row.to_owned(), col.to_owned()))
            .map(|c| c.get_cell_value())
            .unwrap_or(&self.default_cell_value)
    }

    pub fn get_style<T>(&self, coordinate: T) -> &Style
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.map
            .get(&(row.to_owned(), col.to_owned()))
            .map(|c| c.get_style())
            .unwrap_or(&self.default_style)
    }

    pub(crate) fn set(&mut self, cell: Cell) -> &mut Self {
        let col_num = cell.get_coordinate().get_col_num();
        let row_num = cell.get_coordinate().get_row_num();
        let target_cell = self.get_mut((col_num, row_num));
        target_cell.set_obj(cell);
        self
    }

    pub(crate) fn set_fast(&mut self, cell: Cell) -> &mut Self {
        self.add(cell);
        self
    }

    pub(crate) fn add(&mut self, cell: Cell) {
        let col_num = cell.get_coordinate().get_col_num();
        let row_num = cell.get_coordinate().get_row_num();
        let k = (row_num.to_owned(), col_num.to_owned());
        self.map.insert_unique_unchecked(k, cell);
    }

    pub(crate) fn remove(&mut self, col_num: &u32, row_num: &u32) -> bool {
        let k = (*row_num, *col_num);
        self.map.remove(&k).is_some()
    }

    pub fn get_cell_value_by_range(&self, range: &str) -> Vec<&CellValue> {
        let mut result: Vec<&CellValue> = Vec::new();
        let range_upper = range.to_uppercase();
        let coordinate_list = get_coordinate_list(&range_upper);
        for (col_num, row_num) in coordinate_list {
            result.push(self.get_cell_value((&col_num, &row_num)));
        }
        result
    }

    pub fn get_formatted_value_by_column_and_row(&self, col_num: &u32, row_num: &u32) -> String {
        match self.get((col_num, row_num)) {
            Some(v) => v.get_formatted_value(),
            None => "".into(),
        }
    }

    // ************************
    // update Coordinate
    // ************************
    /// (This method is crate only.)
    /// Adjustment Insert Coordinate
    pub(crate) fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        // update cell
        for ((_, _), cell) in self.get_collection_to_hashmap_mut() {
            cell.get_coordinate_mut().adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        self.rebuild_map();
    }

    /// (This method is crate only.)
    /// Adjustment Remove Coordinate
    pub(crate) fn adjustment_insert_formula_coordinate(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for ((_, _), cell) in self.get_collection_to_hashmap_mut() {
            cell.get_cell_value_mut()
                .adjustment_insert_formula_coordinate(
                    self_sheet_name,
                    sheet_name,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );
        }
    }

    /// (This method is crate only.)
    /// Adjustment Remove Coordinate
    pub(crate) fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        // update cell
        self.get_collection_mut().retain(|x| {
            !(x.get_coordinate().is_remove(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            ))
        });
        for cell in self.get_collection_mut() {
            cell.get_coordinate_mut().adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        self.rebuild_map();
    }

    /// (This method is crate only.)
    /// Adjustment Remove Coordinate
    pub(crate) fn adjustment_remove_formula_coordinate(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for ((_, _), cell) in self.get_collection_to_hashmap_mut() {
            cell.get_cell_value_mut()
                .adjustment_remove_formula_coordinate(
                    self_sheet_name,
                    sheet_name,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );
        }
    }

    pub(crate) fn rebuild_map(&mut self) {
        let mut rebuild: HashMap<(u32, u32), Cell> = HashMap::new();
        for ((_, _), cell) in self.get_collection_to_hashmap_mut() {
            let col_num = cell.get_coordinate().get_col_num();
            let row_num = cell.get_coordinate().get_row_num();
            let k = (row_num.to_owned(), col_num.to_owned());
            rebuild.insert(k, cell.clone());
        }
        self.map = rebuild;
    }
}
