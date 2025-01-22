use std::collections::HashMap;

use super::{
    Cell,
    CellValue,
    Style,
};
use crate::{
    helper::{
        coordinate::CellCoordinates,
        range::get_coordinate_list,
    },
    structs::{
        Column,
        Row,
    },
    traits::{
        AdjustmentCoordinate,
        AdjustmentCoordinateWith2Sheet,
    },
};

#[derive(Clone, Default, Debug)]
pub struct Cells {
    map:                HashMap<(u32, u32), Box<Cell>>,
    default_cell_value: CellValue,
    default_style:      Style,
}
impl Cells {
    #[inline]
    pub fn collection(&self) -> Vec<&Cell> {
        self.map.values().map(Box::as_ref).collect()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use collection()")]
    pub fn get_collection(&self) -> Vec<&Cell> {
        self.collection()
    }

    #[must_use]
    pub fn collection_sorted(&self) -> Vec<&Cell> {
        let mut cells = self.collection();
        cells.sort_by(|a, b| {
            (
                a.coordinate().get_row_num(),
                a.coordinate().get_col_num(),
            )
                .cmp(&(
                    b.coordinate().get_row_num(),
                    b.coordinate().get_col_num(),
                ))
        });
        cells
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use collection_sorted()")]
    pub fn get_collection_sorted(&self) -> Vec<&Cell> {
        self.collection_sorted()
    }

    #[inline]
    pub(crate) fn collection_mut(&mut self) -> Vec<&mut Cell> {
        self.map.values_mut().map(Box::as_mut).collect()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use collection_mut()")]
    pub(crate) fn get_collection_mut(&mut self) -> Vec<&mut Cell> {
        self.collection_mut()
    }

    #[inline]
    #[must_use]
    pub fn collection_to_hashmap(&self) -> &HashMap<(u32, u32), Box<Cell>> {
        &self.map
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use collection_to_hashmap()")]
    pub fn get_collection_to_hashmap(&self) -> &HashMap<(u32, u32), Box<Cell>> {
        self.collection_to_hashmap()
    }

    #[inline]
    pub fn collection_by_column(&self, column_num: u32) -> Vec<&Cell> {
        self.map
            .values()
            .filter(|k| k.coordinate().get_col_num() == column_num)
            .map(Box::as_ref)
            .collect()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use collection_by_column()")]
    pub fn get_collection_by_column(&self, column_num: u32) -> Vec<&Cell> {
        self.collection_by_column(column_num)
    }

    #[inline]
    pub fn collection_by_row(&self, row_num: u32) -> Vec<&Cell> {
        self.map
            .values()
            .filter(|k| k.coordinate().get_row_num() == row_num)
            .map(Box::as_ref)
            .collect()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use collection_by_column()")]
    pub fn get_collection_by_row(&self, row_num: u32) -> Vec<&Cell> {
        self.collection_by_row(row_num)
    }

    #[inline]
    #[must_use]
    pub fn collection_by_column_to_hashmap(&self, column_num: u32) -> HashMap<u32, &Cell> {
        self.map
            .iter()
            .filter(|(k, _v)| k.1 == column_num)
            .map(|(k, v)| (k.0, v.as_ref()))
            .collect()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use collection_by_column_to_hashmap()")]
    pub fn get_collection_by_column_to_hashmap(&self, column_num: u32) -> HashMap<u32, &Cell> {
        self.collection_by_column_to_hashmap(column_num)
    }

    #[inline]
    #[must_use]
    pub fn collection_by_row_to_hashmap(&self, row_num: u32) -> HashMap<u32, &Cell> {
        self.map
            .iter()
            .filter(|(k, _v)| k.0 == row_num)
            .map(|(k, v)| (k.1, v.as_ref()))
            .collect()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use collection_by_row_to_hashmap()")]
    pub fn get_collection_by_row_to_hashmap(&self, row_num: u32) -> HashMap<u32, &Cell> {
        self.collection_by_row_to_hashmap(row_num)
    }

    #[inline]
    pub(crate) fn collection_to_hashmap_mut(&mut self) -> &mut HashMap<(u32, u32), Box<Cell>> {
        &mut self.map
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use collection_to_hashmap_mut()")]
    pub(crate) fn get_collection_to_hashmap_mut(&mut self) -> &mut HashMap<(u32, u32), Box<Cell>> {
        self.collection_to_hashmap_mut()
    }

    #[must_use]
    pub fn highest_column_and_row(&self) -> (u32, u32) {
        self.map
            .keys()
            .fold((0, 0), |(col_max, row_max), &(col, row)| {
                (col.max(col_max), row.max(row_max))
            })
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use highest_column_and_row()")]
    pub fn get_highest_column_and_row(&self) -> (u32, u32) {
        self.highest_column_and_row()
    }

    /// Has Hyperlink
    #[inline]
    #[must_use]
    pub fn has_hyperlink(&self) -> bool {
        self.map.values().any(|c| c.hyperlink().is_some())
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
                c.coordinate_mut().set_col_num(col);
                c.coordinate_mut().set_row_num(row);
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
    pub fn cell_value<T>(&self, coordinate: T) -> &CellValue
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.map
            .get(&(row.to_owned(), col.to_owned()))
            .map_or(&self.default_cell_value, |c| c.cell_value())
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use cell_value()")]
    pub fn get_cell_value<T>(&self, coordinate: T) -> &CellValue
    where
        T: Into<CellCoordinates>,
    {
        self.cell_value(coordinate)
    }

    #[inline]
    pub fn style<T>(&self, coordinate: T) -> &Style
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.map
            .get(&(row.to_owned(), col.to_owned()))
            .map_or(&self.default_style, |c| c.style())
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use style()")]
    pub fn get_style<T>(&self, coordinate: T) -> &Style
    where
        T: Into<CellCoordinates>,
    {
        self.style(coordinate)
    }

    #[inline]
    pub(crate) fn set(
        &mut self,
        cell: Cell,
        row_dimenshon: &Row,
        col_dimenshon: &Column,
    ) -> &mut Self {
        let col_num = cell.coordinate().get_col_num();
        let row_num = cell.coordinate().get_row_num();
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
        let col_num = cell.coordinate().get_col_num();
        let row_num = cell.coordinate().get_row_num();
        let k = (row_num.to_owned(), col_num.to_owned());
        self.map.insert(k, Box::new(cell));
    }

    #[inline]
    pub(crate) fn remove(&mut self, col_num: u32, row_num: u32) -> bool {
        let k = (row_num, col_num);
        self.map.remove(&k).is_some()
    }

    #[must_use]
    pub fn cell_by_range(&self, range: &str) -> Vec<Option<&Cell>> {
        let mut result: Vec<Option<&Cell>> = Vec::new();
        let coordinate_list = get_coordinate_list(range);
        for (col_num, row_num) in coordinate_list {
            result.push(self.get((col_num, row_num)));
        }
        result
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use style()")]
    pub fn get_cell_by_range(&self, range: &str) -> Vec<Option<&Cell>> {
        self.cell_by_range(range)
    }

    #[must_use]
    pub fn cell_value_by_range(&self, range: &str) -> Vec<&CellValue> {
        let mut result: Vec<&CellValue> = Vec::new();
        let coordinate_list = get_coordinate_list(range);
        for (col_num, row_num) in coordinate_list {
            result.push(self.cell_value((col_num, row_num)));
        }
        result
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use cell_value_by_range()")]
    pub fn get_cell_value_by_range(&self, range: &str) -> Vec<&CellValue> {
        self.cell_value_by_range(range)
    }

    #[inline]
    #[must_use]
    pub fn formatted_value_by_column_and_row(&self, col_num: u32, row_num: u32) -> String {
        match self.get((col_num, row_num)) {
            Some(v) => v.formatted_value(),
            None => String::new(),
        }
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use formatted_value_by_column_and_row()")]
    pub fn get_formatted_value_by_column_and_row(&self, col_num: u32, row_num: u32) -> String {
        self.formatted_value_by_column_and_row(col_num, row_num)
    }

    pub(crate) fn rebuild_map(&mut self) {
        self.map = self
            .collection_to_hashmap_mut()
            .iter_mut()
            .map(|(_, cell)| {
                (
                    (
                        cell.coordinate().get_row_num(),
                        cell.coordinate().get_col_num(),
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
        for ((..), cell) in self.collection_to_hashmap_mut() {
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
            !(x.coordinate().is_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            ))
        });
        for cell in self.collection_mut() {
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
        for ((..), cell) in self.collection_to_hashmap_mut() {
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
        for ((..), cell) in self.collection_to_hashmap_mut() {
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
