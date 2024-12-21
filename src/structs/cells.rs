use super::Cell;
use super::CellValue;
use super::Style;
use crate::helper::coordinate::CellCoordinates;
use crate::helper::range::get_coordinate_list;
use crate::structs::Column;
use crate::structs::Row;
use crate::traits::AdjustmentCoordinate;
use crate::traits::AdjustmentCoordinateWith2Sheet;
use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
pub struct Cells {
    map: HashMap<(u32, u32), Box<Cell>>,
    default_cell_value: CellValue,
    default_style: Style,
}
impl Cells {
    #[inline]
    pub fn get_collection(&self) -> Vec<&Cell> {
        self.map.values().map(Box::as_ref).collect()
    }

    #[must_use]
    pub fn get_collection_sorted(&self) -> Vec<&Cell> {
        let mut cells = self.get_collection();
        cells.sort_by(|a, b| {
            (
                a.get_coordinate().get_row_num(),
                a.get_coordinate().get_col_num(),
            )
                .cmp(&(
                    b.get_coordinate().get_row_num(),
                    b.get_coordinate().get_col_num(),
                ))
        });
        cells
    }

    #[inline]
    pub(crate) fn get_collection_mut(&mut self) -> Vec<&mut Cell> {
        self.map.values_mut().map(Box::as_mut).collect()
    }

    #[inline]
    #[must_use]
    pub fn get_collection_to_hashmap(&self) -> &HashMap<(u32, u32), Box<Cell>> {
        &self.map
    }

    #[inline]
    pub fn get_collection_by_column(&self, column_num: u32) -> Vec<&Cell> {
        self.map
            .values()
            .filter(|k| k.get_coordinate().get_col_num() == column_num)
            .map(Box::as_ref)
            .collect()
    }

    #[inline]
    pub fn get_collection_by_row(&self, row_num: u32) -> Vec<&Cell> {
        self.map
            .values()
            .filter(|k| k.get_coordinate().get_row_num() == row_num)
            .map(Box::as_ref)
            .collect()
    }

    #[inline]
    #[must_use]
    pub fn get_collection_by_column_to_hashmap(&self, column_num: u32) -> HashMap<u32, &Cell> {
        self.map
            .iter()
            .filter(|(k, _v)| k.1 == column_num)
            .map(|(k, v)| (k.0, v.as_ref()))
            .collect()
    }

    #[inline]
    #[must_use]
    pub fn get_collection_by_row_to_hashmap(&self, row_num: u32) -> HashMap<u32, &Cell> {
        self.map
            .iter()
            .filter(|(k, _v)| k.0 == row_num)
            .map(|(k, v)| (k.1, v.as_ref()))
            .collect()
    }

    #[inline]
    pub(crate) fn get_collection_to_hashmap_mut(&mut self) -> &mut HashMap<(u32, u32), Box<Cell>> {
        &mut self.map
    }

    #[must_use]
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
    #[inline]
    #[must_use]
    pub fn has_hyperlink(&self) -> bool {
        self.map.values().any(|c| c.get_hyperlink().is_some())
    }

    #[inline]
    pub fn get<T>(&self, coordinate: T) -> Option<&Cell>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.map
            .get(&(row.to_owned(), col.to_owned()))
            .map(Box::as_ref)
    }

    pub(crate) fn get_mut<T>(
        &mut self,
        coordinate: T,
        row_dimenshon: &Row,
        col_dimenshon: &Column,
    ) -> &mut Cell
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
                if col_dimenshon.has_style() {
                    c.set_style(col_dimenshon.get_style().clone());
                }
                if row_dimenshon.has_style() {
                    c.set_style(row_dimenshon.get_style().clone());
                }
                Box::new(c)
            })
    }

    #[inline]
    pub fn get_cell_value<T>(&self, coordinate: T) -> &CellValue
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.map
            .get(&(row.to_owned(), col.to_owned()))
            .map_or(&self.default_cell_value, |c| c.get_cell_value())
    }

    #[inline]
    pub fn get_style<T>(&self, coordinate: T) -> &Style
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.map
            .get(&(row.to_owned(), col.to_owned()))
            .map_or(&self.default_style, |c| c.get_style())
    }

    #[inline]
    pub(crate) fn set(
        &mut self,
        cell: Cell,
        row_dimenshon: &Row,
        col_dimenshon: &Column,
    ) -> &mut Self {
        let col_num = cell.get_coordinate().get_col_num();
        let row_num = cell.get_coordinate().get_row_num();
        let target_cell = self.get_mut((col_num, row_num), row_dimenshon, col_dimenshon);
        target_cell.set_obj(cell);
        self
    }

    #[inline]
    pub(crate) fn set_fast(&mut self, cell: Cell) -> &mut Self {
        self.add(cell);
        self
    }

    #[inline]
    pub(crate) fn add(&mut self, cell: Cell) {
        let col_num = cell.get_coordinate().get_col_num();
        let row_num = cell.get_coordinate().get_row_num();
        let k = (row_num.to_owned(), col_num.to_owned());
        self.map.insert(k, Box::new(cell));
    }

    #[inline]
    pub(crate) fn remove(&mut self, col_num: u32, row_num: u32) -> bool {
        let k = (row_num, col_num);
        self.map.remove(&k).is_some()
    }

    #[must_use]
    pub fn get_cell_by_range(&self, range: &str) -> Vec<Option<&Cell>> {
        let mut result: Vec<Option<&Cell>> = Vec::new();
        let range_upper = range.to_uppercase();
        let coordinate_list = get_coordinate_list(&range_upper);
        for (col_num, row_num) in coordinate_list {
            result.push(self.get((col_num, row_num)));
        }
        result
    }

    #[must_use]
    pub fn get_cell_value_by_range(&self, range: &str) -> Vec<&CellValue> {
        let mut result: Vec<&CellValue> = Vec::new();
        let range_upper = range.to_uppercase();
        let coordinate_list = get_coordinate_list(&range_upper);
        for (col_num, row_num) in coordinate_list {
            result.push(self.get_cell_value((col_num, row_num)));
        }
        result
    }

    #[inline]
    #[must_use]
    pub fn get_formatted_value_by_column_and_row(&self, col_num: u32, row_num: u32) -> String {
        match self.get((col_num, row_num)) {
            Some(v) => v.get_formatted_value(),
            None => String::new(),
        }
    }

    pub(crate) fn rebuild_map(&mut self) {
        self.map = self
            .get_collection_to_hashmap_mut()
            .iter_mut()
            .map(|(_, cell)| {
                (
                    (
                        cell.get_coordinate().get_row_num(),
                        cell.get_coordinate().get_col_num(),
                    ),
                    std::mem::take(cell),
                )
            })
            .collect();
    }
}
impl AdjustmentCoordinate for Cells {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        // update cell
        for ((_, _), cell) in self.get_collection_to_hashmap_mut() {
            cell.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        self.rebuild_map();
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        // update cell
        self.map.retain(|_, x| {
            !(x.get_coordinate().is_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            ))
        });
        for cell in self.get_collection_mut() {
            cell.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        self.rebuild_map();
    }
}
impl AdjustmentCoordinateWith2Sheet for Cells {
    #[inline]
    fn adjustment_insert_coordinate_with_2sheet(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        for ((_, _), cell) in self.get_collection_to_hashmap_mut() {
            cell.adjustment_insert_coordinate_with_2sheet(
                self_sheet_name,
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    #[inline]
    fn adjustment_remove_coordinate_with_2sheet(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        for ((_, _), cell) in self.get_collection_to_hashmap_mut() {
            cell.adjustment_remove_coordinate_with_2sheet(
                self_sheet_name,
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }
}
