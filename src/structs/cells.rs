use super::Cell;
use super::CellValue;
use super::Style;
use crate::helper::coordinate::*;
use crate::helper::range::*;
use crate::structs::Column;
use crate::structs::Row;
use crate::traits::AdjustmentCoordinate;
use crate::traits::AdjustmentCoordinateWith2Sheet;
use crate::traits::AdjustmentCoordinateWithSheet;
use std::collections::{BTreeSet, HashMap};

#[derive(Clone, Default, Debug)]
pub struct Cells {
    map: HashMap<(u32, u32), Box<Cell>>,
    row_column_index: BTreeSet<(u32, u32)>,
    column_row_index: BTreeSet<(u32, u32)>,
    default_cell_value: CellValue,
    default_style: Style,
}
impl Cells {
    /// Iterates all [`Cell`]s in arbitrary order (not sorted).
    #[inline]
    pub fn iter_collection(&self) -> impl Iterator<Item = &Cell> {
        self.map.values().map(Box::as_ref)
    }

    #[deprecated(
        since = "2.2.4",
        note = "Please use iter_collection().collect::<Vec<_>>() instead"
    )]
    #[inline(always)]
    pub fn get_collection(&self) -> Vec<&Cell> {
        self.iter_collection().collect()
    }

    #[inline]
    pub fn is_row_empty(&self, row_num: u32) -> bool {
        self.row_column_index
            .range((row_num, 0)..=(row_num, u32::MAX))
            .next()
            .is_none()
    }

    #[inline]
    pub fn is_col_empty(&self, col_num: u32) -> bool {
        self.column_row_index
            .range((col_num, 0)..=(col_num, u32::MAX))
            .next()
            .is_none()
    }

    /// Iterates all cell coordinates, sorted by row then by column.
    /// Coordinate returned is (column, row).
    #[inline(always)]
    pub fn iter_coordinates_sorted_by_row_column(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.row_column_index
            .iter()
            .copied()
            .map(|(row, col)| (col, row))
    }

    /// Iterates all [`Cell`]s, sorted by row then by column.
    /// Coordinate returned is (column, row).
    #[inline]
    pub fn iter_cells_sorted_by_row_column(&self) -> impl Iterator<Item = &Cell> {
        self.iter_coordinates_sorted_by_row_column()
            .map(|(col, row)| self.map.get(&(row, col)).unwrap().as_ref())
    }

    /// Iterates all cell coordinates, sorted by column then by row.
    /// Coordinate returned is (column, row).
    #[inline(always)]
    pub fn iter_coordinates_sorted_by_column_row(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.column_row_index.iter().copied()
    }

    /// Iterates all [`Cell`]s, sorted by column then by row.
    /// Coordinate returned is (column, row).
    #[inline]
    pub fn iter_cells_sorted_by_column_row(&self) -> impl Iterator<Item = &Cell> {
        self.iter_coordinates_sorted_by_column_row()
            .map(|(col, row)| self.map.get(&(row, col)).unwrap().as_ref())
    }

    #[deprecated(
        since = "2.2.4",
        note = "Please use iter_cells_sorted_by_row_column().collect::<Vec<_>>() instead"
    )]
    #[inline(always)]
    pub fn get_collection_sorted(&self) -> Vec<&Cell> {
        self.iter_cells_sorted_by_row_column().collect()
    }

    #[inline]
    pub(crate) fn get_collection_mut(&mut self) -> Vec<&mut Cell> {
        self.map.values_mut().map(Box::as_mut).collect()
    }

    #[inline]
    pub fn get_collection_to_hashmap(&self) -> &HashMap<(u32, u32), Box<Cell>> {
        &self.map
    }

    /// Iterates all rows cells in a given column, sorted by the row index.
    #[inline]
    pub fn iter_rows_with_cells_by_column(
        &self,
        column_num: u32,
    ) -> impl Iterator<Item = u32> + '_ {
        self.column_row_index
            .range((column_num, 0)..=(column_num, u32::MAX))
            .copied()
            .map(|(_, row)| row)
    }

    /// Iterates all [`Cell`]s in a given column, sorted by the row index.
    #[inline]
    pub fn iter_cells_by_column(&self, column_num: u32) -> impl Iterator<Item = &Cell> {
        self.iter_rows_with_cells_by_column(column_num)
            .map(move |row| self.map.get(&(row, column_num)).unwrap().as_ref())
    }

    #[deprecated(
        since = "2.2.4",
        note = "Please use iter_cells_by_column(column_num).collect::<Vec<_>>() instead"
    )]
    #[inline(always)]
    pub fn get_collection_by_column(&self, column_num: &u32) -> Vec<&Cell> {
        self.iter_cells_by_column(*column_num).collect()
    }

    /// Iterates all column cells in a given column, sorted by the column index.
    #[inline]
    pub fn iter_columns_with_cells_by_row(&self, row_num: u32) -> impl Iterator<Item = u32> + '_ {
        self.row_column_index
            .range((row_num, 0)..=(row_num, u32::MAX))
            .copied()
            .map(|(_, col)| col)
    }

    /// Iterates all [`Cell`]s in a given column, sorted by the column index.
    #[inline]
    pub fn iter_cells_by_row(&self, row_num: u32) -> impl Iterator<Item = &Cell> {
        self.iter_columns_with_cells_by_row(row_num)
            .map(move |col| self.map.get(&(row_num, col)).unwrap().as_ref())
    }

    #[deprecated(
        since = "2.2.4",
        note = "Please use iter_cells_by_row(row_num).collect::<Vec<_>>() instead"
    )]
    #[inline(always)]
    pub fn get_collection_by_row(&self, row_num: &u32) -> Vec<&Cell> {
        self.iter_cells_by_row(*row_num).collect()
    }

    /// Iterates all coordinates in a range, sorted by row then by column.
    /// Coordinate returned is (column, row).
    #[inline]
    pub fn iter_coordinates_by_range_sorted_by_row(
        &self,
        row_start: u32,
        row_end: u32,
        col_start: u32,
        col_end: u32,
    ) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.row_column_index
            .range((row_start, col_start)..=(row_end, col_end))
            .copied()
            .filter(move |(_, col)| (col_start..=col_end).contains(col))
            .map(|(row, col)| (col, row))
    }

    /// Iterates all [`Cell`]s in a range, sorted by row then by column.
    #[inline]
    pub fn iter_cells_by_range_sorted_by_row(
        &self,
        row_start: u32,
        row_end: u32,
        col_start: u32,
        col_end: u32,
    ) -> impl Iterator<Item = &Cell> {
        self.iter_coordinates_by_range_sorted_by_row(row_start, row_end, col_start, col_end)
            .map(move |(col, row)| self.map.get(&(row, col)).unwrap().as_ref())
    }

    /// Iterates all coordinates in a range, sorted by column then by row.
    /// Coordinate returned is (column, row).
    #[inline]
    pub fn iter_coordinates_by_range_sorted_by_column(
        &self,
        col_start: u32,
        col_end: u32,
        row_start: u32,
        row_end: u32,
    ) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.column_row_index
            .range((col_start, row_start)..=(col_end, row_end))
            .copied()
            .filter(move |(_, row)| (row_start..=row_end).contains(row))
    }

    /// Iterates all [`Cell`]s in a range, sorted by column then by row.
    /// Coordinate returned is (column, row).
    #[inline]
    pub fn iter_cells_by_range_sorted_by_column(
        &self,
        col_start: u32,
        col_end: u32,
        row_start: u32,
        row_end: u32,
    ) -> impl Iterator<Item = &Cell> {
        self.iter_coordinates_by_range_sorted_by_column(col_start, col_end, row_start, row_end)
            .map(move |(col, row)| self.map.get(&(row, col)).unwrap().as_ref())
    }

    #[inline]
    pub fn get_collection_by_column_to_hashmap(&self, column_num: &u32) -> HashMap<u32, &Cell> {
        self.iter_cells_by_column(*column_num)
            .map(|cell| (*cell.get_coordinate().get_row_num(), cell))
            .collect()
    }

    #[inline]
    pub fn get_collection_by_row_to_hashmap(&self, row_num: &u32) -> HashMap<u32, &Cell> {
        self.iter_cells_by_row(*row_num)
            .map(|cell| (*cell.get_coordinate().get_col_num(), cell))
            .collect()
    }

    #[inline]
    pub(crate) fn get_collection_to_hashmap_mut(&mut self) -> &mut HashMap<(u32, u32), Box<Cell>> {
        &mut self.map
    }

    #[inline(always)]
    pub fn get_highest_column_and_row(&self) -> (u32, u32) {
        (
            self.column_row_index.last().copied().unwrap_or((0, 0)).0,
            self.row_column_index.last().copied().unwrap_or((0, 0)).0,
        )
    }

    /// Has Hyperlink
    #[inline]
    pub fn has_hyperlink(&self) -> bool {
        self.map.values().find_map(|c| c.get_hyperlink()).is_some()
    }

    #[inline]
    pub fn get<T>(&self, coordinate: T) -> Option<&Cell>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.map.get(&(row, col)).map(Box::as_ref)
    }

    pub(crate) fn get_mut<T>(
        &mut self,
        coordinate: T,
        row_dimension: &Row,
        col_dimension: &Column,
    ) -> &mut Cell
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.map.entry((row, col)).or_insert_with(|| {
            let mut c = Cell::default();
            c.get_coordinate_mut().set_col_num(col);
            c.get_coordinate_mut().set_row_num(row);
            if col_dimension.has_style() {
                c.set_style(col_dimension.get_style().clone());
            }
            if row_dimension.has_style() {
                c.set_style(row_dimension.get_style().clone());
            }

            self.row_column_index.insert((row, col));
            self.column_row_index.insert((col, row));

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
            .get(&(row, col))
            .map(|c| c.get_cell_value())
            .unwrap_or(&self.default_cell_value)
    }

    #[inline]
    pub fn get_style<T>(&self, coordinate: T) -> &Style
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.map
            .get(&(row, col))
            .map(|c| c.get_style())
            .unwrap_or(&self.default_style)
    }

    #[inline]
    pub(crate) fn set(
        &mut self,
        cell: Cell,
        row_dimension: &Row,
        col_dimension: &Column,
    ) -> &mut Self {
        let col_num = cell.get_coordinate().get_col_num();
        let row_num = cell.get_coordinate().get_row_num();
        let target_cell = self.get_mut((col_num, row_num), row_dimension, col_dimension);
        target_cell.set_obj(cell);
        self
    }

    #[inline]
    pub(crate) fn set_fast(&mut self, cell: Cell) -> &mut Self {
        self.add(cell);
        self
    }

    pub(crate) fn add(&mut self, cell: Cell) {
        let col_num = *cell.get_coordinate().get_col_num();
        let row_num = *cell.get_coordinate().get_row_num();
        self.map.insert((row_num, col_num), Box::new(cell));
        self.row_column_index.insert((row_num, col_num));
        self.column_row_index.insert((col_num, row_num));
    }

    #[inline]
    pub(crate) fn remove(&mut self, col_num: &u32, row_num: &u32) -> bool {
        let k = (*row_num, *col_num);
        let r = self.map.remove(&k).is_some();
        if r {
            self.row_column_index.remove(&k);
            self.column_row_index.remove(&(k.1, k.0));
        }
        r
    }

    pub fn iter_all_coordinates_by_range_sorted_by_row(
        &self,
        range: &str,
    ) -> impl Iterator<Item = Option<(u32, u32)>> + '_ {
        let (row_start, row_end, col_start, col_end) = get_start_and_end_point(range);

        let mut iter =
            self.iter_coordinates_by_range_sorted_by_row(row_start, row_end, col_start, col_end);

        let mut current = iter.next();

        (row_start..=row_end)
            .flat_map(move |row| (col_start..=col_end).map(move |col| (row, col)))
            .map(move |x| {
                if let Some((cur_col, cur_row)) = current {
                    if x < (cur_row, cur_col) {
                        None
                    } else {
                        current = iter.next();
                        Some((x.1, x.0))
                    }
                } else {
                    None
                }
            })
    }

    pub fn iter_all_cells_by_range_sorted_by_row(
        &self,
        range: &str,
    ) -> impl Iterator<Item = Option<&Cell>> + '_ {
        self.iter_all_coordinates_by_range_sorted_by_row(range)
            .map(move |coordinate| {
                coordinate.map(move |(col, row)| self.map.get(&(row, col)).unwrap().as_ref())
            })
    }

    #[inline]
    pub fn iter_all_cell_values_by_range_sorted_by_row(
        &self,
        range: &str,
    ) -> impl Iterator<Item = &CellValue> + '_ {
        self.iter_all_coordinates_by_range_sorted_by_row(range)
            .map(|coordinate| {
                coordinate.map_or(&self.default_cell_value, |c| self.get_cell_value(c))
            })
    }

    pub fn iter_all_coordinates_by_range_sorted_by_column(
        &self,
        range: &str,
    ) -> impl Iterator<Item = Option<(u32, u32)>> + '_ {
        let (row_start, row_end, col_start, col_end) = get_start_and_end_point(range);

        let mut iter =
            self.iter_coordinates_by_range_sorted_by_column(col_start, col_end, row_start, row_end);

        let mut current = iter.next();

        (col_start..=col_end)
            .flat_map(move |col| (row_start..=row_end).map(move |row| (col, row)))
            .map(move |coordinate| {
                if let Some(cur) = current {
                    if coordinate < cur {
                        None
                    } else {
                        current = iter.next();
                        Some(coordinate)
                    }
                } else {
                    None
                }
            })
    }

    pub fn iter_all_cells_by_range_sorted_by_column(
        &self,
        range: &str,
    ) -> impl Iterator<Item = Option<&Cell>> + '_ {
        self.iter_all_coordinates_by_range_sorted_by_column(range)
            .map(move |coordinate| {
                coordinate.map(move |(col, row)| self.map.get(&(row, col)).unwrap().as_ref())
            })
    }

    #[inline]
    pub fn iter_all_cell_values_by_range_sorted_by_column(
        &self,
        range: &str,
    ) -> impl Iterator<Item = &CellValue> + '_ {
        self.iter_all_coordinates_by_range_sorted_by_column(range)
            .map(|coordinate| {
                coordinate.map_or(&self.default_cell_value, |c| self.get_cell_value(c))
            })
    }

    #[deprecated(
        since = "2.2.4",
        note = "Please use iter_all_cells_by_range_sorted_by_row(range).collect::<Vec<_>>() instead"
    )]
    #[inline(always)]
    pub fn get_cell_by_range(&self, range: &str) -> Vec<Option<&Cell>> {
        self.iter_all_cells_by_range_sorted_by_row(range).collect()
    }

    #[deprecated(
        since = "2.2.4",
        note = "Please use iter_all_cell_values_by_range_sorted_by_row(range).collect::<Vec<_>>() instead"
    )]
    #[inline(always)]
    pub fn get_cell_value_by_range(&self, range: &str) -> Vec<&CellValue> {
        self.iter_all_cell_values_by_range_sorted_by_row(range)
            .collect::<Vec<_>>()
    }

    #[inline]
    pub fn get_formatted_value_by_column_and_row(&self, col_num: &u32, row_num: &u32) -> String {
        match self.get((col_num, row_num)) {
            Some(v) => v.get_formatted_value(),
            None => "".into(),
        }
    }

    pub(crate) fn rebuild_map_and_indices(&mut self) {
        self.map = self
            .get_collection_to_hashmap_mut()
            .iter_mut()
            .map(|(_, cell)| {
                (
                    (
                        *cell.get_coordinate().get_row_num(),
                        *cell.get_coordinate().get_col_num(),
                    ),
                    std::mem::take(cell),
                )
            })
            .collect();

        self.row_column_index = self.map.keys().copied().collect();

        self.column_row_index = self
            .map
            .keys()
            .copied()
            .map(|(col, row)| (row, col))
            .collect();
    }
}
impl AdjustmentCoordinate for Cells {
    #[inline]
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        // update cell
        for cell in self.get_collection_to_hashmap_mut().values_mut() {
            cell.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
        self.rebuild_map_and_indices();
    }

    #[inline]
    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        // update cell
        self.map.retain(|k, x| {
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
        self.rebuild_map_and_indices();
    }
}
impl AdjustmentCoordinateWith2Sheet for Cells {
    #[inline]
    fn adjustment_insert_coordinate_with_2sheet(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for cell in self.get_collection_to_hashmap_mut().values_mut() {
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
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        for cell in self.get_collection_to_hashmap_mut().values_mut() {
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
