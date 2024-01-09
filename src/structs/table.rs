use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};

use super::coordinate::*;
use crate::helper::coordinate::*;
//use reader::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Table {
    name: String,
    area: (Coordinate, Coordinate),
    display_name: String,
    columns: Vec<TableColumn>,
    style_info: Option<TableStyleInfo>,
}
impl Table {
    pub fn new<T>(name: &str, area: (T, T)) -> Self
    where
        T: Into<CellCoordinates>,
    {
        let coord_beg = Self::cell_coord_to_coord(area.0);
        let coord_end = Self::cell_coord_to_coord(area.1);
        let name = name.to_string();
        Self {
            area: (coord_beg, coord_end),
            name: name.clone(),
            display_name: name,
            columns: Vec::<TableColumn>::default(),
            style_info: None,
        }
    }

    pub fn is_ok(&self) -> bool {
        if self.name.is_empty() || self.display_name.is_empty() {
            return false;
        }
        !(self.area.0.get_col_num() == &0
            || self.area.0.get_row_num() == &0
            || self.area.1.get_col_num() == &0
            || self.area.1.get_row_num() == &0
            || self.area.0.get_col_num() > self.area.1.get_col_num()
            || self.area.0.get_row_num() > self.area.1.get_row_num())
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
        if self.display_name.is_empty() {
            self.display_name = name.to_string();
        }
    }

    pub fn get_display_name(&self) -> &String {
        &self.display_name
    }

    pub fn set_display_name(&mut self, display_name: &str) {
        self.display_name = display_name.to_string();
    }

    pub fn get_area(&self) -> &(Coordinate, Coordinate) {
        &self.area
    }

    pub fn set_area<T>(&mut self, area: (T, T))
    where
        T: Into<CellCoordinates>,
    {
        let coord_beg = Self::cell_coord_to_coord(area.0);
        let coord_end = Self::cell_coord_to_coord(area.1);
        self.area = (coord_beg, coord_end);
    }

    pub fn add_column(&mut self, col: TableColumn) {
        self.columns.push(col);
    }

    pub fn get_columns(&self) -> &Vec<TableColumn> {
        &self.columns
    }

    pub fn has_style_info(&self) -> bool {
        self.style_info.is_some()
    }

    pub fn get_style_info(&self) -> Option<&TableStyleInfo> {
        self.style_info.as_ref()
    }

    pub fn set_style_info(&mut self, style_info: Option<TableStyleInfo>) {
        self.style_info = style_info;
    }

    fn cell_coord_to_coord<T>(cc: T) -> Coordinate
    where
        T: Into<CellCoordinates>,
    {
        let cell_coord: CellCoordinates = cc.into();
        let mut coord: Coordinate = Default::default();
        coord.set_col_num(cell_coord.col);
        coord.set_row_num(cell_coord.row);
        coord
    }
}

#[derive(Clone, Default, Debug)]
pub struct TableColumn {
    name: String,
}
impl TableColumn {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

#[derive(Clone, Default, Debug)]
pub struct TableStyleInfo {
    name: String,
    show_first_col: bool,
    show_last_col: bool,
    show_row_stripes: bool,
    show_col_stripes: bool,
}
impl TableStyleInfo {
    pub fn new(
        name: &str,
        show_first_col: bool,
        show_last_col: bool,
        show_row_stripes: bool,
        show_col_stripes: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            show_first_col,
            show_last_col,
            show_row_stripes,
            show_col_stripes,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn is_show_first_col(&self) -> bool {
        self.show_first_col
    }

    pub fn is_show_last_col(&self) -> bool {
        self.show_last_col
    }

    pub fn is_show_row_stripes(&self) -> bool {
        self.show_row_stripes
    }

    pub fn is_show_col_stripes(&self) -> bool {
        self.show_col_stripes
    }
}
