use super::{
    coordinate::Coordinate, BooleanValue, EnumValue, StringValue, TotalsRowFunctionValues,
    UInt32Value,
};
use crate::helper::coordinate::CellCoordinates;
use thin_vec::ThinVec;
//use reader::driver::*;

#[derive(Clone, Default, Debug)]
pub struct Table {
    name: Box<str>,
    area: (Coordinate, Coordinate),
    display_name: Box<str>,
    columns: ThinVec<TableColumn>,
    style_info: Option<Box<TableStyleInfo>>,
    totals_row_shown: BooleanValue,
    totals_row_count: UInt32Value,
}
impl Table {
    #[inline]
    pub fn new<T>(name: &str, area: (T, T)) -> Self
    where
        T: Into<CellCoordinates>,
    {
        let coord_beg = Self::cell_coord_to_coord(area.0);
        let coord_end = Self::cell_coord_to_coord(area.1);
        let name: Box<str> = name.into();
        Self {
            area: (coord_beg, coord_end),
            name: name.clone(),
            display_name: name,
            columns: ThinVec::<TableColumn>::default(),
            style_info: None,
            totals_row_shown: BooleanValue::default(),
            totals_row_count: UInt32Value::default(),
        }
    }

    #[inline]
    #[must_use]
    pub fn is_ok(&self) -> bool {
        !(self.name.is_empty()
            || self.display_name.is_empty()
            || self.area.0.get_col_num() == 0
            || self.area.0.get_row_num() == 0
            || self.area.1.get_col_num() == 0
            || self.area.1.get_row_num() == 0
            || self.area.0.get_col_num() > self.area.1.get_col_num()
            || self.area.0.get_row_num() > self.area.1.get_row_num())
    }

    #[inline]
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    #[inline]
    pub fn set_name(&mut self, name: &str) {
        self.name = name.into();
        if self.display_name.is_empty() {
            self.display_name = name.into();
        }
    }

    #[inline]
    #[must_use]
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    #[inline]
    pub fn set_display_name(&mut self, display_name: &str) {
        self.display_name = display_name.into();
    }

    #[inline]
    #[must_use]
    pub fn get_area(&self) -> &(Coordinate, Coordinate) {
        &self.area
    }

    #[inline]
    pub fn set_area<T>(&mut self, area: (T, T))
    where
        T: Into<CellCoordinates>,
    {
        let coord_beg = Self::cell_coord_to_coord(area.0);
        let coord_end = Self::cell_coord_to_coord(area.1);
        self.area = (coord_beg, coord_end);
    }

    #[inline]
    pub fn add_column(&mut self, col: TableColumn) {
        self.columns.push(col);
    }

    #[inline]
    #[must_use]
    pub fn get_columns(&self) -> &[TableColumn] {
        &self.columns
    }

    #[inline]
    pub(crate) fn has_style_info(&self) -> bool {
        self.style_info.is_some()
    }

    #[inline]
    #[must_use]
    pub fn get_style_info(&self) -> Option<&TableStyleInfo> {
        self.style_info.as_deref()
    }

    #[inline]
    pub fn set_style_info(&mut self, style_info: Option<TableStyleInfo>) {
        self.style_info = style_info.map(Box::new);
    }

    #[inline]
    pub(crate) fn has_totals_row_shown(&self) -> bool {
        self.totals_row_shown.has_value()
    }

    #[inline]
    #[must_use]
    pub fn get_totals_row_shown(&self) -> bool {
        self.totals_row_shown.get_value()
    }

    #[inline]
    pub(crate) fn get_totals_row_shown_str(&self) -> &str {
        self.totals_row_shown.get_value_string()
    }

    #[inline]
    pub fn set_totals_row_shown(&mut self, value: bool) {
        self.totals_row_shown.set_value(value);
    }

    #[inline]
    pub(crate) fn set_totals_row_shown_str(&mut self, value: &str) {
        self.totals_row_shown.set_value_string(value);
    }

    #[inline]
    pub(crate) fn has_totals_row_count(&self) -> bool {
        self.totals_row_count.has_value()
    }

    #[inline]
    #[must_use]
    pub fn get_totals_row_count(&self) -> u32 {
        self.totals_row_count.get_value()
    }

    #[inline]
    pub(crate) fn get_totals_row_count_str(&self) -> String {
        self.totals_row_count.get_value_string()
    }

    #[inline]
    pub fn set_totals_row_count(&mut self, value: u32) {
        self.totals_row_count.set_value(value);
    }

    #[inline]
    pub(crate) fn set_totals_row_count_str(&mut self, value: &str) {
        self.totals_row_count.set_value_string(value);
    }

    #[inline]
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
    totals_row_label: StringValue,
    totals_row_function: EnumValue<TotalsRowFunctionValues>,
    calculated_column_formula: Option<String>,
}
impl TableColumn {
    #[inline]
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            totals_row_label: StringValue::default(),
            totals_row_function: EnumValue::default(),
            calculated_column_formula: None,
        }
    }

    #[inline]
    #[must_use]
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    #[inline]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn has_totals_row_label(&self) -> bool {
        self.totals_row_label.has_value()
    }

    #[inline]
    #[must_use]
    pub fn get_totals_row_label(&self) -> Option<&str> {
        self.totals_row_label.get_value()
    }

    #[inline]
    pub(crate) fn get_totals_row_label_str(&self) -> &str {
        self.totals_row_label.get_value_str()
    }

    #[inline]
    pub fn set_totals_row_label(&mut self, value: &str) {
        self.totals_row_label.set_value(value);
    }

    #[inline]
    pub(crate) fn set_totals_row_label_str(&mut self, value: &str) {
        self.totals_row_label.set_value_string(value);
    }

    #[inline]
    #[allow(dead_code)]
    pub(crate) fn has_totals_row_function(&self) -> bool {
        self.totals_row_function.has_value()
    }

    #[inline]
    #[must_use]
    pub fn get_totals_row_function(&self) -> &TotalsRowFunctionValues {
        self.totals_row_function.get_value()
    }

    #[inline]
    pub(crate) fn get_totals_row_function_str(&self) -> &str {
        self.totals_row_function.get_value_string()
    }

    #[inline]
    pub fn set_totals_row_function(&mut self, value: TotalsRowFunctionValues) {
        self.totals_row_function.set_value(value);
    }

    #[inline]
    pub(crate) fn set_totals_row_function_str(&mut self, value: &str) {
        self.totals_row_function.set_value_string(value);
    }

    #[inline]
    #[must_use]
    pub fn get_calculated_column_formula(&self) -> Option<&String> {
        self.calculated_column_formula.as_ref()
    }

    #[inline]
    pub(crate) fn set_calculated_column_formula(&mut self, value: String) {
        self.calculated_column_formula = Some(value);
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
    #[inline]
    #[must_use]
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

    #[inline]
    #[must_use]
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    #[inline]
    #[must_use]
    pub fn is_show_first_col(&self) -> bool {
        self.show_first_col
    }

    #[inline]
    #[must_use]
    pub fn is_show_last_col(&self) -> bool {
        self.show_last_col
    }

    #[inline]
    #[must_use]
    pub fn is_show_row_stripes(&self) -> bool {
        self.show_row_stripes
    }

    #[inline]
    #[must_use]
    pub fn is_show_col_stripes(&self) -> bool {
        self.show_col_stripes
    }
}
