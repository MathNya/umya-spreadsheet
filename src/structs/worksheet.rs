use crate::helper::const_str::*;
use crate::helper::coordinate::*;
use crate::helper::range::*;
use crate::reader::xlsx::worksheet::*;
use crate::structs::drawing::spreadsheet::WorksheetDrawing;
use crate::structs::office2010::excel::DataValidations as DataValidations2010;
use crate::structs::raw::RawWorksheet;
use crate::structs::AutoFilter;
use crate::structs::Cell;
use crate::structs::CellValue;
use crate::structs::Cells;
use crate::structs::Chart;
use crate::structs::Color;
use crate::structs::Column;
use crate::structs::ColumnBreaks;
use crate::structs::Columns;
use crate::structs::Comment;
use crate::structs::ConditionalFormatting;
use crate::structs::DataValidations;
use crate::structs::DefinedName;
use crate::structs::EnumValue;
use crate::structs::HeaderFooter;
use crate::structs::Hyperlink;
use crate::structs::Image;
use crate::structs::MediaObject;
use crate::structs::MergeCells;
use crate::structs::OleObjects;
use crate::structs::PageMargins;
use crate::structs::PageSetup;
use crate::structs::PivotTable;
use crate::structs::PrintOptions;
use crate::structs::Range;
use crate::structs::Row;
use crate::structs::RowBreaks;
use crate::structs::Rows;
use crate::structs::SharedStringTable;
use crate::structs::SheetFormatProperties;
use crate::structs::SheetProtection;
use crate::structs::SheetStateValues;
use crate::structs::SheetViews;
use crate::structs::Style;
use crate::structs::Stylesheet;
use crate::structs::Table;
use crate::traits;
use crate::traits::AdjustmentCoordinate;
use crate::traits::AdjustmentCoordinateWith2Sheet;
use crate::traits::AdjustmentCoordinateWithSheet;
use crate::traits::AdjustmentValue;
use crate::StringValue;
use std::collections::HashMap;
use thin_vec::ThinVec;

use super::EnumTrait;

/// A Worksheet Object.
#[derive(Clone, Debug, Default)]
pub struct Worksheet {
    raw_data_of_worksheet: Option<RawWorksheet>,
    r_id: Box<str>,
    sheet_id: Box<str>,
    title: Box<str>,
    state: EnumValue<SheetStateValues>,
    cell_collection: Cells,
    row_dimensions: Rows,
    column_dimensions: Columns,
    worksheet_drawing: WorksheetDrawing,
    sheet_state: Box<str>,
    page_setup: PageSetup,
    page_margins: PageMargins,
    header_footer: HeaderFooter,
    sheet_views: SheetViews,
    conditional_formatting_collection: ThinVec<ConditionalFormatting>,
    merge_cells: MergeCells,
    auto_filter: Option<AutoFilter>,
    comments: ThinVec<Comment>,
    active_cell: Box<str>,
    tab_color: Option<Color>,
    code_name: StringValue,
    ole_objects: OleObjects,
    defined_names: ThinVec<DefinedName>,
    print_options: PrintOptions,
    column_breaks: ColumnBreaks,
    row_breaks: RowBreaks,
    tables: ThinVec<Table>,
    pivot_tables: ThinVec<PivotTable>,
    data_validations: Option<DataValidations>,
    data_validations_2010: Option<DataValidations2010>,
    sheet_format_properties: SheetFormatProperties,
    sheet_protection: Option<SheetProtection>,
}

impl Worksheet {
    // ************************
    // Value
    // ************************

    /// Get value.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(&1, &1)`
    /// # Return value
    /// * `String` - Value of the specified cell.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(&0).unwrap();
    /// let value = worksheet.get_value("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let value = worksheet.get_value((1, 1));
    /// ```
    #[inline]
    pub fn get_value<T>(&self, coordinate: T) -> String
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_cell((col, row))
            .map(|v| v.get_value().into())
            .unwrap_or_default()
    }

    /// Get value number.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)`
    /// # Return value
    /// * `Option<f64>` - Value of the specified cell.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(&0).unwrap();
    /// let value = worksheet.get_value_number("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let value = worksheet.get_value_number((1, 1));
    /// ```
    #[inline]
    pub fn get_value_number<T>(&self, coordinate: T) -> Option<f64>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_cell((col, row)).and_then(|v| v.get_value_number())
    }

    /// Get formatted value.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(&1, &1)`
    /// # Return value
    /// * `String` - Formatted value of the specified cell.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(&0).unwrap();
    /// let value = worksheet.get_formatted_value("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let value = worksheet.get_formatted_value((1, 1));
    /// ```
    #[inline]
    pub fn get_formatted_value<T>(&self, coordinate: T) -> String
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.cell_collection
            .get_formatted_value_by_column_and_row(&col, &row)
    }

    // ************************
    // Cell
    // ************************
    /// Get Cell List.
    #[inline]
    pub fn get_cell_collection(&self) -> Vec<&Cell> {
        self.cell_collection.iter_collection().collect()
    }

    #[inline]
    pub fn get_cell_collection_sorted(&self) -> Vec<&Cell> {
        self.cell_collection
            .iter_cells_sorted_by_row_column()
            .collect()
    }

    /// Get Cell List in mutable.
    #[inline]
    pub fn get_cell_collection_mut(&mut self) -> Vec<&mut Cell> {
        self.cell_collection.get_collection_mut()
    }

    #[inline]
    pub fn get_collection_to_hashmap(&self) -> &HashMap<(u32, u32), Box<Cell>> {
        self.cell_collection.get_collection_to_hashmap()
    }

    #[inline]
    pub fn get_collection_to_hashmap_mut(&mut self) -> &mut HashMap<(u32, u32), Box<Cell>> {
        self.cell_collection.get_collection_to_hashmap_mut()
    }

    pub(crate) fn get_cell_collection_stream(
        &self,
        shared_string_table: &SharedStringTable,
        stylesheet: &Stylesheet,
    ) -> Cells {
        assert!(!self.is_deserialized(), "This Worksheet is Deserialized.");

        read_lite(
            self.raw_data_of_worksheet.as_ref().unwrap(),
            shared_string_table,
            stylesheet,
        )
        .unwrap()
    }

    /// (This method is crate only.)
    /// Get Cells.
    #[inline]
    pub(crate) fn get_cell_collection_crate(&self) -> &Cells {
        &self.cell_collection
    }

    /// (This method is crate only.)
    /// Get Cells in mutable.
    #[inline]
    pub(crate) fn get_cell_collection_crate_mut(&mut self) -> &mut Cells {
        &mut self.cell_collection
    }

    /// Get cell.
    /// # Note
    /// Cells with unset Value and Style will return None.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(&1, &1)`
    /// # Return value
    /// * `Option` - Cell in the Some.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(&0).unwrap();
    /// let cell = worksheet.get_cell("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let cell = worksheet.get_cell((1, 1));
    /// ```
    #[inline]
    pub fn get_cell<T>(&self, coordinate: T) -> Option<&Cell>
    where
        T: Into<CellCoordinates>,
    {
        self.cell_collection.get(coordinate)
    }

    /// Get cell with mutable.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(&1, &1)`
    /// # Return value
    /// * `&mut Cell` - Cell with mutable.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// let cell = worksheet.get_cell_mut("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let cell = worksheet.get_cell_mut((1, 1));
    /// ```
    pub fn get_cell_mut<T>(&mut self, coordinate: T) -> &mut Cell
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_row_dimension_mut(&row);
        let row_dimenshon = self.get_row_dimension_mut(&row).clone();
        let col_dimenshon = self.get_column_dimension_by_number_mut(&col).clone();
        self.cell_collection
            .get_mut((col, row), &row_dimenshon, &col_dimenshon)
    }

    #[inline]
    pub fn get_collection_by_column(&self, column_num: &u32) -> Vec<&Cell> {
        self.cell_collection
            .iter_cells_by_column(*column_num)
            .collect()
    }

    #[inline]
    pub fn get_collection_by_row(&self, row_num: &u32) -> Vec<&Cell> {
        self.cell_collection.iter_cells_by_row(*row_num).collect()
    }

    #[inline]
    pub fn get_collection_by_column_to_hashmap(&self, column_num: &u32) -> HashMap<u32, &Cell> {
        self.cell_collection
            .get_collection_by_column_to_hashmap(column_num)
    }

    #[inline]
    pub fn get_collection_by_row_to_hashmap(&self, row_num: &u32) -> HashMap<u32, &Cell> {
        self.cell_collection
            .get_collection_by_row_to_hashmap(row_num)
    }

    /// Set Cell
    /// # Arguments
    /// * `cell` - Cell
    pub fn set_cell(&mut self, cell: Cell) -> &mut Self {
        let row_dimenshon = self
            .get_row_dimension_mut(cell.get_coordinate().get_row_num())
            .clone();
        let col_dimenshon = self
            .get_column_dimension_by_number_mut(cell.get_coordinate().get_col_num())
            .clone();
        self.cell_collection
            .set(cell, &row_dimenshon, &col_dimenshon);
        self
    }

    /// Remove Cell
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(&1, &1)`
    /// # Examples
    /// ```
    /// worksheet.remove_cell("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// worksheet.remove_cell((1, 1));
    /// ```
    #[inline]
    pub fn remove_cell<T>(&mut self, coordinate: T) -> bool
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.cell_collection.remove(&col, &row)
    }

    /// Get cell value.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(&1, &1)`
    /// # Return value
    /// * `&CellValue` - CellValue.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(&0).unwrap();
    /// let cell_value = worksheet.get_cell_value("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let cell_value = worksheet.get_cell_value((1, 1));
    /// ```
    #[inline]
    pub fn get_cell_value<T>(&self, coordinate: T) -> &CellValue
    where
        T: Into<CellCoordinates>,
    {
        self.cell_collection.get_cell_value(coordinate)
    }

    /// Get cell value with mutable.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(&1, &1)`
    /// # Return value
    /// * `&mut CellValue` - CellValue with mutable.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// let cell_value = worksheet.get_cell_value_mut("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let cell_value = worksheet.get_cell_value_mut((1, 1));
    /// ```
    #[inline]
    pub fn get_cell_value_mut<T>(&mut self, coordinate: T) -> &mut CellValue
    where
        T: Into<CellCoordinates>,
    {
        self.get_cell_mut(coordinate).get_cell_value_mut()
    }

    /// Gets the cell value by specifying an range.
    /// # Arguments
    /// * `range` - range. ex) "A1:C5"
    /// # Return value
    /// *`Vec<&CellValue>` - CellValue List.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// let mut cell_value_List = worksheet.get_cell_value_by_range("A1:C5");
    /// ```
    #[inline]
    pub fn get_cell_value_by_range(&self, range: &str) -> Vec<&CellValue> {
        self.cell_collection
            .iter_all_cell_values_by_range_sorted_by_row(range)
            .collect()
    }

    /// Get style.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(&1, &1)`
    /// # Return value
    /// * `&Style` - Style.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(&0).unwrap();
    /// let style = worksheet.get_style("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let style = worksheet.get_style((1, 1));
    /// ```
    #[inline]
    pub fn get_style<T>(&self, coordinate: T) -> &Style
    where
        T: Into<CellCoordinates>,
    {
        self.cell_collection.get_style(coordinate)
    }

    /// Get style with mutable.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(&1, &1)`
    /// # Return value
    /// * `&mut Style` - Style with mutable.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// let style = worksheet.get_style_mut("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let style = worksheet.get_style_mut((1, 1));
    /// ```
    #[inline]
    pub fn get_style_mut<T>(&mut self, coordinate: T) -> &mut Style
    where
        T: Into<CellCoordinates>,
    {
        self.get_cell_mut(coordinate).get_style_mut()
    }

    #[inline]
    pub fn set_style<T>(&mut self, coordinate: T, style: Style) -> &mut Self
    where
        T: Into<CellCoordinates>,
    {
        self.get_cell_mut(coordinate).set_style(style);
        self
    }

    /// Set style by range.
    /// # Arguments
    /// * `range` - Specify the range. ex) "A1:B2"
    /// * `style` - Style
    /// # Return value
    /// * `&mut Self` - Self.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// let mut style = umya_spreadsheet::Style::default();
    /// style.get_borders_mut().get_bottom_mut().set_border_style(umya_spreadsheet::Border::BORDER_MEDIUM);
    /// worksheet.set_style_by_range("A1:A3", style);
    /// ```
    pub fn set_style_by_range(&mut self, range: &str, style: Style) -> &mut Self {
        let coordinate_list = get_coordinate_list(&range);

        let (col_num_start, row_num_start) = coordinate_list[0];
        if col_num_start == 0 {
            let (_, row_num_end) = coordinate_list[1];
            for row_num in row_num_start..=row_num_end {
                self.get_row_dimension_mut(&row_num)
                    .set_style(style.clone());
            }
            return self;
        }
        if row_num_start == 0 {
            let (col_num_end, _) = coordinate_list[1];
            for col_num in col_num_start..=col_num_end {
                self.get_column_dimension_by_number_mut(&col_num)
                    .set_style(style.clone());
            }
            return self;
        }

        for (col_num, row_num) in coordinate_list {
            self.set_style((col_num, row_num), style.clone());
        }
        self
    }

    /// Get the upper-left corner cell of a cell inside a merged cell range (if any).
    pub fn map_merged_cell<T>(&self, coordinate: T) -> (u32, u32)
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.merge_cells
            .get_range_collection()
            .into_iter()
            .find_map(|range| {
                let Some(start_col) = range.get_coordinate_start_col() else {
                    return None;
                };
                let Some(start_row) = range.get_coordinate_start_row() else {
                    return None;
                };
                let Some(end_col) = range.get_coordinate_end_col() else {
                    return None;
                };
                let Some(end_row) = range.get_coordinate_end_row() else {
                    return None;
                };
                if col >= *start_col.get_num()
                    && col <= *end_col.get_num()
                    && row >= *start_row.get_num()
                    && row <= *end_row.get_num()
                {
                    Some((*start_col.get_num(), *start_row.get_num()))
                } else {
                    None
                }
            })
            .unwrap_or((col, row))
    }

    // ************************
    // Comment
    // ************************
    /// Get Comments
    #[inline]
    pub fn get_comments(&self) -> &[Comment] {
        &self.comments
    }

    /// Get Comments in mutable.
    #[inline]
    pub fn get_comments_mut(&mut self) -> &mut ThinVec<Comment> {
        &mut self.comments
    }

    /// Get Comments convert to hashmap.
    #[inline]
    pub fn get_comments_to_hashmap(&self) -> HashMap<String, &Comment> {
        let mut result = HashMap::default();
        for comment in &self.comments {
            result.insert(comment.get_coordinate().to_string(), comment);
        }
        result
    }

    /// Set Comments.
    /// # Arguments
    /// * `value` - Comment List (Vec)
    #[inline]
    pub fn set_comments(&mut self, value: impl Into<ThinVec<Comment>>) {
        self.comments = value.into();
    }

    /// Add Comments.
    /// # Arguments
    /// * `value` - Comment
    #[inline]
    pub fn add_comments(&mut self, value: Comment) {
        self.comments.push(value);
    }

    /// Has Comments.
    #[inline]
    pub fn has_comments(&self) -> bool {
        !self.comments.is_empty()
    }

    // ************************
    // Conditional
    // ************************
    /// Get ConditionalFormatting list.
    #[inline]
    pub fn get_conditional_formatting_collection(&self) -> &[ConditionalFormatting] {
        &self.conditional_formatting_collection
    }

    /// Set ConditionalFormatting.
    /// # Arguments
    /// * `value` - ConditionalSet List (Vec)
    #[inline]
    pub fn set_conditional_formatting_collection(
        &mut self,
        value: impl Into<ThinVec<ConditionalFormatting>>,
    ) {
        self.conditional_formatting_collection = value.into();
    }

    /// Add ConditionalFormatting.
    /// # Arguments
    /// * `value` - ConditionalFormatting
    #[inline]
    pub fn add_conditional_formatting_collection(&mut self, value: ConditionalFormatting) {
        self.conditional_formatting_collection.push(value);
    }

    // ************************
    // Hyperlink
    // ************************
    /// (This method is crate only.)
    /// Get Hyperlink convert to hashmap.
    pub(crate) fn get_hyperlink_collection_to_hashmap(&self) -> HashMap<String, &Hyperlink> {
        let mut result: HashMap<String, &Hyperlink> = HashMap::new();
        for cell in self.cell_collection.iter_collection() {
            if let Some(hyperlink) = cell.get_hyperlink() {
                let coordition = coordinate_from_index(
                    cell.get_coordinate().get_col_num(),
                    cell.get_coordinate().get_row_num(),
                );
                result.insert(coordition, hyperlink);
            }
        }
        result
    }

    /// (This method is crate only.)
    /// Has Hyperlink
    #[inline]
    pub(crate) fn has_hyperlink(&self) -> bool {
        self.cell_collection.has_hyperlink()
    }

    // ************************
    // Merge Cells
    // ************************
    // Get Merge Cells
    #[inline]
    pub fn get_merge_cells(&self) -> &[Range] {
        self.merge_cells.get_range_collection()
    }

    // Get Merge Cells in mutable.
    #[inline]
    pub fn get_merge_cells_mut(&mut self) -> &mut ThinVec<Range> {
        self.merge_cells.get_range_collection_mut()
    }

    // Add Merge Cells.
    /// # Arguments
    /// * `range` - Range. ex) "A1:C5"
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// worksheet.add_merge_cells("A1:C5");
    /// ```
    #[inline]
    pub fn add_merge_cells<S: Into<String>>(&mut self, range: S) -> &mut Self {
        self.merge_cells.add_range(range);
        self
    }

    /// (This method is crate only.)
    // Get Merge Cells Object
    #[inline]
    pub(crate) fn get_merge_cells_crate(&self) -> &MergeCells {
        &self.merge_cells
    }

    /// (This method is crate only.)
    // Get Merge Cells Object in mutable.
    #[inline]
    pub(crate) fn get_merge_cells_crate_mut(&mut self) -> &mut MergeCells {
        &mut self.merge_cells
    }

    // ************************
    // Auto Filter
    // ************************
    // Get Auto Filter (Option).
    #[inline]
    pub fn get_auto_filter(&self) -> Option<&AutoFilter> {
        self.auto_filter.as_ref()
    }

    // Get Auto Filter (Option) in mutable.
    #[inline]
    pub fn get_auto_filter_mut(&mut self) -> Option<&mut AutoFilter> {
        self.auto_filter.as_mut()
    }

    // Set Auto Filter.
    /// # Arguments
    /// * `range` - Range. ex) "A2:K2"
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// worksheet.set_auto_filter("A2:K2");
    /// ```
    #[inline]
    pub fn set_auto_filter<S: Into<String>>(&mut self, range: S) {
        let mut auto_filter = AutoFilter::default();
        auto_filter.set_range(range);
        self.auto_filter = Some(auto_filter);
    }

    // Remove Auto Filter.
    #[inline]
    pub fn remove_auto_filter(&mut self) {
        self.auto_filter = None;
    }

    // ************************
    // Column Dimensions
    // ************************
    /// Get Column Dimension List.
    #[inline]
    pub fn get_column_dimensions(&self) -> &[Column] {
        self.column_dimensions.get_column_collection()
    }

    /// Get Column Dimension List in mutable.
    #[inline]
    pub fn get_column_dimensions_mut(&mut self) -> &mut ThinVec<Column> {
        self.column_dimensions.get_column_collection_mut()
    }

    /// Calculation Auto Width.
    #[inline]
    pub fn calculation_auto_width(&mut self) -> &mut Self {
        let cells = self.get_cell_collection_crate().clone();
        let merge_cells = self.get_merge_cells_crate().clone();
        self.get_column_dimensions_crate_mut()
            .calculation_auto_width(&cells, &merge_cells);
        self
    }

    /// Get Column Dimension.
    /// # Arguments
    /// * `column` - Column Char. ex) "A"
    #[inline]
    pub fn get_column_dimension(&self, column: &str) -> Option<&Column> {
        self.get_column_dimension_by_number(&column_index_from_string(column))
    }

    /// Get Column Dimension in mutable.
    /// # Arguments
    /// * `column` - Column Char. ex) "A"
    #[inline]
    pub fn get_column_dimension_mut(&mut self, column: &str) -> &mut Column {
        self.get_column_dimension_by_number_mut(&column_index_from_string(column))
    }

    /// Get Column Dimension.
    /// # Arguments
    /// * `col` - Column Number.
    #[inline]
    pub fn get_column_dimension_by_number(&self, col: &u32) -> Option<&Column> {
        self.get_column_dimensions_crate().get_column(col)
    }

    /// Get Column Dimension in mutable.
    /// # Arguments
    /// * `col` - Column Number.
    #[inline]
    pub fn get_column_dimension_by_number_mut(&mut self, col: &u32) -> &mut Column {
        self.get_column_dimensions_crate_mut().get_column_mut(col)
    }

    /// (This method is crate only.)
    /// Get Column Dimension.
    #[inline]
    pub(crate) fn get_column_dimensions_crate(&self) -> &Columns {
        &self.column_dimensions
    }

    /// (This method is crate only.)
    /// Get Column Dimension in mutable.
    #[inline]
    pub(crate) fn get_column_dimensions_crate_mut(&mut self) -> &mut Columns {
        &mut self.column_dimensions
    }

    /// (This method is crate only.)
    /// Set Column Dimension.
    #[inline]
    pub(crate) fn set_column_dimensions_crate(&mut self, value: Columns) -> &mut Self {
        self.column_dimensions = value;
        self
    }

    // ************************
    // Row Dimensions
    // ************************
    #[inline]
    pub fn has_sheet_data(&self) -> bool {
        self.row_dimensions.has_sheet_data()
    }

    /// Get Row Dimension List.
    #[inline]
    pub fn get_row_dimensions(&self) -> Vec<&Row> {
        self.row_dimensions.get_row_dimensions()
    }

    /// Get Row Dimension List in mutable.
    #[inline]
    pub fn get_row_dimensions_mut(&mut self) -> Vec<&mut Row> {
        self.row_dimensions.get_row_dimensions_mut()
    }

    /// Get Row Dimension convert Hashmap.
    #[inline]
    pub fn get_row_dimensions_to_hashmap(&self) -> &HashMap<u32, Box<Row>> {
        self.row_dimensions.get_row_dimensions_to_hashmap()
    }

    #[inline]
    pub fn get_row_dimensions_to_hashmap_mut(&mut self) -> &mut HashMap<u32, Box<Row>> {
        self.row_dimensions.get_row_dimensions_to_hashmap_mut()
    }

    /// Get Row Dimension.
    #[inline]
    pub fn get_row_dimension(&self, row: &u32) -> Option<&Row> {
        self.row_dimensions.get_row_dimension(row)
    }

    /// Get Row Dimension in mutable.
    #[inline]
    pub fn get_row_dimension_mut(&mut self, row: &u32) -> &mut Row {
        self.row_dimensions.get_row_dimension_mut(row)
    }

    /// (This method is crate only.)
    /// Set Row Dimension.
    #[inline]
    pub(crate) fn set_row_dimension(&mut self, value: Row) -> &mut Self {
        self.row_dimensions.set_row_dimension(value);
        self
    }

    /// (This method is crate only.)
    /// Get Row Dimension in mutable.
    #[inline]
    pub(crate) fn get_row_dimensions_crate_mut(&mut self) -> &mut Rows {
        &mut self.row_dimensions
    }

    /// (This method is crate only.)
    /// Get Row Dimension.
    #[inline]
    pub(crate) fn _get_row_dimensions_crate(&self) -> &Rows {
        &self.row_dimensions
    }

    // ************************
    // WorksheetDrawing
    // ************************
    /// Get WorksheetDrawing.
    #[inline]
    pub fn get_worksheet_drawing(&self) -> &WorksheetDrawing {
        &self.worksheet_drawing
    }

    /// Get WorksheetDrawing in mutable.
    #[inline]
    pub fn get_worksheet_drawing_mut(&mut self) -> &mut WorksheetDrawing {
        &mut self.worksheet_drawing
    }

    /// Set WorksheetDrawing.
    /// # Arguments
    /// * `value` - WorksheetDrawing
    #[inline]
    pub fn set_worksheet_drawing(&mut self, value: WorksheetDrawing) {
        self.worksheet_drawing = value;
    }

    /// Has WorksheetDrawing.
    #[inline]
    pub fn has_drawing_object(&self) -> bool {
        self.worksheet_drawing.has_drawing_object()
    }

    // ************************
    // update Coordinate
    // ************************
    /// Insert new rows.
    /// # Arguments
    /// * `row_index` - Specify point of insert. ex) 1
    /// * `num_rows` - Specify number to insert. ex) 2
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// worksheet.insert_new_row(&2, &3);
    /// ```
    #[inline]
    pub fn insert_new_row(&mut self, row_index: &u32, num_rows: &u32) {
        let title = self.title.clone();
        self.adjustment_insert_coordinate(&0, &0, row_index, num_rows);
        self.adjustment_insert_coordinate_with_sheet(&title, &0, &0, row_index, num_rows);
    }

    /// Adjust for references to other sheets.
    #[inline]
    pub fn insert_new_row_from_other_sheet(
        &mut self,
        sheet_name: &str,
        row_index: &u32,
        num_rows: &u32,
    ) {
        self.adjustment_insert_coordinate(&0, &0, row_index, num_rows);
        self.adjustment_insert_coordinate_with_sheet(sheet_name, &0, &0, row_index, num_rows);
    }

    /// Insert new columns.
    /// # Arguments
    /// * `column` - Specify point of insert. ex) "B"
    /// * `num_columns` - Specify number to insert. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// worksheet.insert_new_column("B", &3);
    /// ```
    #[inline]
    pub fn insert_new_column(&mut self, column: &str, num_columns: &u32) {
        self.insert_new_column_by_index(&column_index_from_string(column), num_columns);
    }

    /// Adjust for references to other sheets.
    #[inline]
    pub fn insert_new_column_from_other_sheet(
        &mut self,
        sheet_name: &str,
        column: &str,
        num_columns: &u32,
    ) {
        self.insert_new_column_by_index_from_other_sheet(
            sheet_name,
            &column_index_from_string(column),
            num_columns,
        );
    }

    /// Insert new columns.
    /// # Arguments
    /// * `column_index` - Specify point of insert. ex) 2
    /// * `num_columns` - Specify number to insert. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// worksheet.insert_new_column_by_index(&2, &3);
    /// ```
    #[inline]
    pub fn insert_new_column_by_index(&mut self, column_index: &u32, num_columns: &u32) {
        let title = self.title.clone();
        self.adjustment_insert_coordinate(column_index, num_columns, &0, &0);
        self.adjustment_insert_coordinate_with_sheet(&title, column_index, num_columns, &0, &0);
    }

    /// Adjust for references to other sheets.
    #[inline]
    pub fn insert_new_column_by_index_from_other_sheet(
        &mut self,
        sheet_name: &str,
        column_index: &u32,
        num_columns: &u32,
    ) {
        self.adjustment_insert_coordinate(column_index, num_columns, &0, &0);
        self.adjustment_insert_coordinate_with_sheet(sheet_name, column_index, num_columns, &0, &0);
    }

    /// Remove rows.
    /// # Arguments
    /// * `row_index` - Specify point of remove. ex) 1
    /// * `num_rows` - Specify number to remove. ex) 2
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// worksheet.remove_row(&2, &3);
    /// ```
    #[inline]
    pub fn remove_row(&mut self, row_index: &u32, num_rows: &u32) {
        let title = self.title.clone();
        self.adjustment_remove_coordinate(&0, &0, row_index, num_rows);
        self.adjustment_remove_coordinate_with_sheet(&title, &0, &0, row_index, num_rows);
    }

    /// Adjust for references to other sheets.
    #[inline]
    pub fn remove_row_from_other_sheet(
        &mut self,
        sheet_name: &str,
        row_index: &u32,
        num_rows: &u32,
    ) {
        self.adjustment_remove_coordinate(&0, &0, row_index, num_rows);
        self.adjustment_remove_coordinate_with_sheet(sheet_name, &0, &0, row_index, num_rows);
    }

    /// Remove columns.
    /// # Arguments
    /// * `sheet_name` - Specify the sheet name. ex) "Sheet1"
    /// * `column` - Specify point of remove. ex) "B"
    /// * `num_columns` - Specify number to remove. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// worksheet.remove_column("B", &3);
    /// ```
    #[inline]
    pub fn remove_column(&mut self, column: &str, num_columns: &u32) {
        self.remove_column_by_index(&column_index_from_string(column), num_columns);
    }

    /// Adjust for references to other sheets.
    #[inline]
    pub fn remove_column_from_other_sheet(
        &mut self,
        sheet_name: &str,
        column: &str,
        num_columns: &u32,
    ) {
        self.remove_column_by_index_from_other_sheet(
            sheet_name,
            &column_index_from_string(column),
            num_columns,
        );
    }

    /// Remove columns.
    /// # Arguments
    /// * `column_index` - Specify point of remove. ex) 2
    /// * `num_columns` - Specify number to remove. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// worksheet.remove_column_by_index(&2, &3);
    /// ```
    #[inline]
    pub fn remove_column_by_index(&mut self, column_index: &u32, num_columns: &u32) {
        let title = self.title.clone();
        self.adjustment_remove_coordinate(column_index, num_columns, &0, &0);
        self.adjustment_remove_coordinate_with_sheet(&title, column_index, num_columns, &0, &0);
    }

    /// Adjust for references to other sheets.
    #[inline]
    pub fn remove_column_by_index_from_other_sheet(
        &mut self,
        sheet_name: &str,
        column_index: &u32,
        num_columns: &u32,
    ) {
        self.adjustment_remove_coordinate(column_index, num_columns, &0, &0);
        self.adjustment_remove_coordinate_with_sheet(sheet_name, column_index, num_columns, &0, &0);
    }

    /// Get Code Name.
    #[inline]
    pub fn get_code_name(&self) -> Option<&str> {
        self.code_name.get_value()
    }

    /// Set Code Name.
    /// # Arguments
    /// * `value` - Code Name
    #[inline]
    pub fn set_code_name<S: Into<String>>(&mut self, value: S) {
        self.code_name.set_value(value);
    }

    /// Get Header Footer.
    #[inline]
    pub fn get_header_footer(&self) -> &HeaderFooter {
        &self.header_footer
    }

    /// Get Header Footer in mutable.
    #[inline]
    pub fn get_header_footer_mut(&mut self) -> &mut HeaderFooter {
        &mut self.header_footer
    }

    /// Set Header Footer.
    /// # Arguments
    /// * `value` - Header Footer
    #[inline]
    pub fn set_header_footer(&mut self, value: HeaderFooter) -> &mut Self {
        self.header_footer = value;
        self
    }

    /// Get Active Cell.
    #[inline]
    pub fn get_active_cell(&self) -> &str {
        &self.active_cell
    }

    /// Set Active Cell.
    /// # Arguments
    /// * `cell` - Cell ex) "A1"
    #[inline]
    pub fn set_active_cell<S: Into<String>>(&mut self, cell: S) {
        self.active_cell = cell.into().into_boxed_str();
    }

    /// Get R Id.
    #[inline]
    pub(crate) fn get_r_id(&self) -> &str {
        &self.r_id
    }

    /// (This method is crate only.)
    /// Set r Id.
    #[inline]
    pub(crate) fn set_r_id<S: Into<String>>(&mut self, value: S) {
        self.r_id = value.into().into_boxed_str();
    }

    /// Get Sheet Id.
    #[inline]
    pub fn get_sheet_id(&self) -> &str {
        &self.sheet_id
    }

    /// (This method is crate only.)
    /// Set Sheet Id.
    #[inline]
    pub(crate) fn set_sheet_id<S: Into<String>>(&mut self, value: S) {
        self.sheet_id = value.into().into_boxed_str();
    }

    /// Has Code Name.
    #[inline]
    pub fn has_code_name(&self) -> bool {
        self.code_name.has_value()
    }

    /// Get Tab Color.
    #[inline]
    pub fn get_tab_color(&self) -> Option<&Color> {
        self.tab_color.as_ref()
    }

    /// Get Tab Color in mutable.
    #[inline]
    pub fn get_tab_color_mut(&mut self) -> &mut Color {
        if self.tab_color.is_some() {
            return self.tab_color.as_mut().unwrap();
        }
        self.set_tab_color(Color::default());
        self.tab_color.as_mut().unwrap()
    }

    /// Set Tab Color.
    /// # Arguments
    /// * `value` - Color
    #[inline]
    pub fn set_tab_color(&mut self, value: Color) -> &mut Self {
        self.tab_color = Some(value);
        self
    }

    /// Remove Tab Color.
    #[inline]
    pub fn remove_tab_color(&mut self) -> &mut Self {
        self.tab_color = None;
        self
    }

    /// Calculate Worksheet Dimension.
    pub fn calculate_worksheet_dimension(&self) -> String {
        let (column, row) = self.cell_collection.get_highest_column_and_row();
        if row == 0 {
            return "A1".to_string();
        }
        let column_str = string_from_column_index(&column);
        format!("A1:{}{}", column_str, row)
    }

    // Get Highest Column and Row Index
    /// # Return value
    /// *`(u32, u32)` - (column, row)
    #[inline]
    pub fn get_highest_column_and_row(&self) -> (u32, u32) {
        self.cell_collection.get_highest_column_and_row()
    }

    // Get Highest Column Index
    #[inline]
    pub fn get_highest_column(&self) -> u32 {
        let (column, _row) = self.cell_collection.get_highest_column_and_row();
        column
    }

    // Get Highest Row Index
    #[inline]
    pub fn get_highest_row(&self) -> u32 {
        let (_column, row) = self.cell_collection.get_highest_column_and_row();
        row
    }

    /// Get SheetName.
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.title
    }

    /// Set SheetName.
    /// # Arguments
    /// * `sheet_name` - Sheet Name. [Caution] no duplicate other worksheet.
    pub fn set_name<S: Into<String>>(&mut self, sheet_name: S) -> &mut Self {
        self.title = sheet_name.into().into_boxed_str();
        let title = self.get_name().to_string();
        for defined_name in self.get_defined_names_mut() {
            defined_name.set_sheet_name(&title);
        }
        self
    }

    #[inline]
    pub(crate) fn has_state(&self) -> bool {
        self.state.has_value()
    }

    #[inline]
    pub fn get_state(&self) -> &SheetStateValues {
        self.state.get_value()
    }

    #[inline]
    pub(crate) fn get_state_str(&self) -> &str {
        self.state.get_value_string()
    }

    pub fn set_state(&mut self, value: SheetStateValues) -> &mut Self {
        self.state.set_value(value);
        self
    }

    #[inline]
    pub(crate) fn set_state_str(&mut self, value: &str) -> &mut Self {
        self.state.set_value_string(value);
        self
    }

    // Get Sheet State
    #[inline]
    pub fn get_sheet_state(&self) -> &str {
        &self.sheet_state
    }

    /// Set Sheet State.
    /// # Arguments
    /// * `value` - Sheet State.
    #[inline]
    pub fn set_sheet_state(&mut self, value: String) -> &mut Self {
        self.sheet_state = value.into_boxed_str();
        self
    }

    // Get Page Setup.
    #[inline]
    pub fn get_page_setup(&self) -> &PageSetup {
        &self.page_setup
    }

    // Get Page Setup in mutable.
    #[inline]
    pub fn get_page_setup_mut(&mut self) -> &mut PageSetup {
        &mut self.page_setup
    }

    /// Set Page Setup.
    /// # Arguments
    /// * `value` - PageSetup.
    #[inline]
    pub fn set_page_setup(&mut self, value: PageSetup) -> &mut Self {
        self.page_setup = value;
        self
    }

    // Get Page Margins.
    #[inline]
    pub fn get_page_margins(&self) -> &PageMargins {
        &self.page_margins
    }

    // Get Page Margins in mutable.
    #[inline]
    pub fn get_page_margins_mut(&mut self) -> &mut PageMargins {
        &mut self.page_margins
    }

    /// Set Page Margins.
    /// # Arguments
    /// * `value` - PageMargins.
    #[inline]
    pub fn set_page_margins(&mut self, value: PageMargins) -> &mut Self {
        self.page_margins = value;
        self
    }

    // Get SheetViews.
    #[inline]
    pub fn get_sheets_views(&self) -> &SheetViews {
        &self.sheet_views
    }

    // Get SheetViews in mutable.
    #[inline]
    pub fn get_sheet_views_mut(&mut self) -> &mut SheetViews {
        &mut self.sheet_views
    }

    /// Set SheetViews.
    /// # Arguments
    /// * `value` - SheetViews.
    #[inline]
    pub fn set_sheets_views(&mut self, value: SheetViews) -> &mut Self {
        self.sheet_views = value;
        self
    }

    // Get Ole Objects.
    #[inline]
    pub fn get_ole_objects(&self) -> &OleObjects {
        &self.ole_objects
    }

    // Get Ole Objects in mutable.
    #[inline]
    pub fn get_ole_objects_mut(&mut self) -> &mut OleObjects {
        &mut self.ole_objects
    }

    /// Set Ole Objects.
    /// # Arguments
    /// * `value` - OleObjects.
    #[inline]
    pub fn set_ole_objects(&mut self, value: OleObjects) -> &mut Self {
        self.ole_objects = value;
        self
    }

    /// Get Defined Name (Vec).
    #[inline]
    pub fn get_defined_names(&self) -> &[DefinedName] {
        &self.defined_names
    }

    /// Get Defined Name (Vec) in mutable.
    #[inline]
    pub fn get_defined_names_mut(&mut self) -> &mut ThinVec<DefinedName> {
        &mut self.defined_names
    }

    /// Set Defined Name (Vec).
    /// # Arguments
    /// * `value` - Vec<DefinedName>.
    #[inline]
    pub fn set_defined_names(&mut self, value: impl Into<ThinVec<DefinedName>>) {
        self.defined_names = value.into();
    }

    /// Add Defined Name.
    /// # Arguments
    /// * `value` - DefinedName.
    #[inline]
    pub fn add_defined_names(&mut self, value: DefinedName) {
        self.defined_names.push(value);
    }

    /// Add Defined Name.
    /// # Arguments
    /// * `name` - Name. ex) "DefinedName01"
    /// * `address` - Address. ex) "A1:A2"
    #[inline]
    pub fn add_defined_name<S: Into<String>>(&mut self, name: S, address: S) -> Result<(), &str> {
        let mut defined_name = DefinedName::default();
        defined_name.set_name(name.into());
        defined_name.set_address(address.into());
        self.add_defined_names(defined_name);
        Ok(())
    }

    /// Get Print Options.
    #[inline]
    pub fn get_print_options(&self) -> &PrintOptions {
        &self.print_options
    }

    /// Get Print Options in mutable.
    #[inline]
    pub fn get_print_options_mut(&mut self) -> &mut PrintOptions {
        &mut self.print_options
    }

    /// Set Print Options.
    /// # Arguments
    /// * `value` - PrintOptions.
    #[inline]
    pub fn set_print_options(&mut self, value: PrintOptions) -> &mut Self {
        self.print_options = value;
        self
    }

    /// Get Column Breaks.
    #[inline]
    pub fn get_column_breaks(&self) -> &ColumnBreaks {
        &self.column_breaks
    }

    /// Get Column Breaks in mutable.
    #[inline]
    pub fn get_column_breaks_mut(&mut self) -> &mut ColumnBreaks {
        &mut self.column_breaks
    }

    /// Set Column Breaks.
    /// # Arguments
    /// * `value` - ColumnBreaks.
    #[inline]
    pub fn set_column_breaks(&mut self, value: ColumnBreaks) -> &mut Self {
        self.column_breaks = value;
        self
    }

    /// Get Row Breaks.
    #[inline]
    pub fn get_row_breaks(&self) -> &RowBreaks {
        &self.row_breaks
    }

    /// Get Row Breaks in mutable.
    #[inline]
    pub fn get_row_breaks_mut(&mut self) -> &mut RowBreaks {
        &mut self.row_breaks
    }

    /// Set Row Breaks.
    /// # Arguments
    /// * `value` - RowBreaks.
    #[inline]
    pub fn set_row_breaks(&mut self, value: RowBreaks) -> &mut Self {
        self.row_breaks = value;
        self
    }

    #[inline]
    pub fn has_table(&self) -> bool {
        !self.tables.is_empty()
    }

    #[inline]
    pub fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }

    #[inline]
    pub fn get_tables(&self) -> &[Table] {
        &self.tables
    }

    #[inline]
    pub fn get_tables_mut(&mut self) -> &mut ThinVec<Table> {
        &mut self.tables
    }

    #[inline]
    pub fn has_pivot_table(&self) -> bool {
        !self.pivot_tables.is_empty()
    }

    #[inline]
    pub fn add_pivot_table(&mut self, pivot_table: PivotTable) {
        self.pivot_tables.push(pivot_table);
    }

    #[inline]
    pub fn get_pivot_tables(&self) -> &[PivotTable] {
        &self.pivot_tables
    }

    #[inline]
    pub fn get_pivot_tables_mut(&mut self) -> &mut ThinVec<PivotTable> {
        &mut self.pivot_tables
    }

    #[inline]
    pub fn get_data_validations(&self) -> Option<&DataValidations> {
        self.data_validations.as_ref()
    }

    #[inline]
    pub fn get_data_validations_mut(&mut self) -> Option<&mut DataValidations> {
        self.data_validations.as_mut()
    }

    #[inline]
    pub fn set_data_validations(&mut self, value: DataValidations) -> &mut Self {
        self.data_validations = Some(value);
        self
    }

    #[inline]
    pub fn remove_data_validations(&mut self) -> &mut Self {
        self.data_validations = None;
        self
    }

    #[inline]
    pub fn get_data_validations_2010(&self) -> Option<&DataValidations2010> {
        self.data_validations_2010.as_ref()
    }

    #[inline]
    pub fn get_data_validations_2010_mut(&mut self) -> Option<&mut DataValidations2010> {
        self.data_validations_2010.as_mut()
    }

    #[inline]
    pub fn set_data_validations_2010(&mut self, value: DataValidations2010) -> &mut Self {
        self.data_validations_2010 = Some(value);
        self
    }

    #[inline]
    pub fn remove_data_validations_2010(&mut self) -> &mut Self {
        self.data_validations_2010 = None;
        self
    }

    #[inline]
    pub fn get_sheet_format_properties(&self) -> &SheetFormatProperties {
        &self.sheet_format_properties
    }

    #[inline]
    pub fn get_sheet_format_properties_mut(&mut self) -> &mut SheetFormatProperties {
        &mut self.sheet_format_properties
    }

    #[inline]
    pub fn set_sheet_format_properties(&mut self, value: SheetFormatProperties) -> &mut Self {
        self.sheet_format_properties = value;
        self
    }

    /// Outputs all images contained in the worksheet.
    /// # Return value
    /// * `&Vec<Image>` - Image Object List.
    #[inline]
    pub fn get_image_collection(&self) -> &[Image] {
        self.get_worksheet_drawing().get_image_collection()
    }

    /// Outputs all images contained in the worksheet.
    /// # Return value
    /// * `&mut Vec<Image>` - Image Object List.
    #[inline]
    pub fn get_image_collection_mut(&mut self) -> &mut ThinVec<Image> {
        self.get_worksheet_drawing_mut().get_image_collection_mut()
    }

    #[inline]
    pub fn add_image(&mut self, value: Image) -> &mut Self {
        self.get_worksheet_drawing_mut().add_image(value);
        self
    }

    #[inline]
    pub fn get_image<T>(&self, coordinate: T) -> Option<&Image>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing().get_image(&col, &row)
    }

    #[inline]
    pub fn get_image_mut<T>(&mut self, coordinate: T) -> Option<&mut Image>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing_mut().get_image_mut(&col, &row)
    }

    #[inline]
    pub fn get_image_by_column_and_row_mut(&mut self, col: &u32, row: &u32) -> Option<&mut Image> {
        self.get_worksheet_drawing_mut().get_image_mut(col, row)
    }

    #[inline]
    pub fn get_images<T>(&self, coordinate: T) -> Vec<&Image>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing().get_images(&col, &row)
    }

    #[inline]
    pub fn get_images_mut<T>(&mut self, coordinate: T) -> Vec<&mut Image>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing_mut().get_images_mut(&col, &row)
    }

    /// Outputs all Charts contained in the worksheet.
    /// # Return value
    /// * `&Vec<Chart>` - Chart Object List.
    #[inline]
    pub fn get_chart_collection(&self) -> &[Chart] {
        self.get_worksheet_drawing().get_chart_collection()
    }

    /// Outputs all Charts contained in the worksheet.
    /// # Return value
    /// * `&mut Vec<Chart>` - Chart Object List.
    #[inline]
    pub fn get_chart_collection_mut(&mut self) -> &mut ThinVec<Chart> {
        self.get_worksheet_drawing_mut().get_chart_collection_mut()
    }

    #[inline]
    pub fn add_chart(&mut self, value: Chart) -> &mut Self {
        self.get_worksheet_drawing_mut().add_chart_collection(value);
        self
    }

    #[inline]
    pub fn get_chart<T>(&self, coordinate: T) -> Option<&Chart>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing().get_chart(&col, &row)
    }

    #[inline]
    pub fn get_chart_mut<T>(&mut self, coordinate: T) -> Option<&mut Chart>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing_mut().get_chart_mut(&col, &row)
    }

    #[inline]
    pub fn get_charts<T>(&self, coordinate: T) -> Vec<&Chart>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing().get_charts(&col, &row)
    }

    #[inline]
    pub fn get_charts_mut<T>(&mut self, coordinate: T) -> Vec<&mut Chart>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing_mut().get_charts_mut(&col, &row)
    }

    /// Outputs all media contained in the worksheet.
    /// # Return value
    /// * `Vec<&MediaObject>` - Media Object List.
    pub(crate) fn get_media_object_collection(&self) -> Vec<&MediaObject> {
        let mut list: Vec<&MediaObject> = Vec::new();
        for image in self.get_worksheet_drawing().get_image_collection() {
            for media_object in image.get_media_object() {
                let is_new = !list
                    .iter()
                    .any(|v| v.get_image_name() == media_object.get_image_name());
                if is_new {
                    list.push(media_object);
                }
            }
        }
        for ole_objects in self.get_ole_objects().get_ole_object() {
            let media_object = ole_objects.get_embedded_object_properties().get_image();
            let is_new = !list
                .iter()
                .any(|v| v.get_image_name() == media_object.get_image_name());
            if is_new {
                list.push(media_object);
            }
        }
        for ole_objects in self.get_ole_objects().get_ole_object() {
            if let Some(fill) = ole_objects.get_shape().get_fill() {
                if let Some(media_object) = fill.get_image() {
                    let is_new = !list
                        .iter()
                        .any(|v| v.get_image_name() == media_object.get_image_name());
                    if is_new {
                        list.push(media_object);
                    }
                }
            }
        }
        for comment in self.get_comments() {
            if let Some(fill) = comment.get_shape().get_fill() {
                if let Some(media_object) = fill.get_image() {
                    let is_new = !list
                        .iter()
                        .any(|v| v.get_image_name() == media_object.get_image_name());
                    if is_new {
                        list.push(media_object);
                    }
                }
            }
        }

        list
    }

    pub(crate) fn get_pivot_cache_definition_collection(&self) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        if let Some(raw_data) = &self.raw_data_of_worksheet {
            for relationships in raw_data.get_relationships_list() {
                for row in relationships.get_relationship_list() {
                    if row.get_type() == PIVOT_CACHE_DEF_NS {
                        result.push(row.get_raw_file().get_file_target());
                    }
                }
            }
        }
        result
    }

    /// (This method is crate only.)
    /// Has Defined Names.
    #[inline]
    pub(crate) fn has_defined_names(&self) -> bool {
        !self.get_defined_names().is_empty()
    }

    #[inline]
    pub(crate) fn is_deserialized(&self) -> bool {
        self.raw_data_of_worksheet.is_none()
    }

    #[inline]
    pub(crate) fn get_raw_data_of_worksheet(&self) -> &RawWorksheet {
        self.raw_data_of_worksheet
            .as_ref()
            .expect("Not found at raw data of worksheet.")
    }

    #[inline]
    pub(crate) fn set_raw_data_of_worksheet(&mut self, value: RawWorksheet) -> &mut Self {
        self.raw_data_of_worksheet = Some(value);
        self
    }

    #[inline]
    pub(crate) fn remove_raw_data_of_worksheet(&mut self) -> &mut Self {
        self.raw_data_of_worksheet = None;
        self
    }

    #[inline]
    pub fn get_sheet_protection(&self) -> Option<&SheetProtection> {
        self.sheet_protection.as_ref()
    }

    #[inline]
    pub fn get_sheet_protection_mut(&mut self) -> &mut SheetProtection {
        self.sheet_protection
            .get_or_insert(SheetProtection::default())
    }

    #[inline]
    pub fn set_sheet_protection(&mut self, value: SheetProtection) -> &mut Self {
        self.sheet_protection = Some(value);
        self
    }

    #[inline]
    pub fn remove_sheet_protection(&mut self) -> &mut Self {
        self.sheet_protection = None;
        self
    }

    /// (This method is crate only.)
    /// Has Ole Objects.
    #[inline]
    pub(crate) fn has_ole_objects(&self) -> bool {
        !self.ole_objects.get_ole_object().is_empty()
    }

    /// (This method is crate only.)
    /// Has Legacy Drawing.
    #[inline]
    pub(crate) fn has_legacy_drawing(&self) -> bool {
        self.has_comments() || self.has_ole_objects()
    }

    /// Moving a section of the sheet
    /// # Arguments
    /// 'range' - Specify like "A1:G8"
    /// 'row' - The number of rows to move by (negative numbers mean move 'left')
    /// 'column' - the number of columns to move by (negative numbers mean move 'up')
    #[inline]
    pub fn move_range(&mut self, range: &str, row: &i32, column: &i32) -> &mut Self {
        self.move_or_copy_range(&range, &row, &column, true)
    }

    /// Copying a section of the sheet
    /// # Arguments
    /// 'range' - Specify like "A1:G8"
    /// 'row' - The number of rows to move by (negative numbers mean move 'left')
    /// 'column' - the number of columns to move by (negative numbers mean move 'up')
    #[inline]
    pub fn copy_range(&mut self, range: &str, row: &i32, column: &i32) -> &mut Self {
        self.move_or_copy_range(&range, &row, &column, false)
    }

    // Moving or copying a section of the sheet
    #[inline]
    fn move_or_copy_range(
        &mut self,
        range: &str,
        row: &i32,
        column: &i32,
        is_move: bool,
    ) -> &mut Self {
        // Check to ensure coordinates to move are within range (eg: moving A1 cells to the left is
        // impossible)

        let (row_start, row_end, col_start, col_end) = get_start_and_end_point(&range);
        if (col_start as i32 + column) < 1
            || (row_start as i32 + row) < 1
            || (col_end as i32 + column) > 16384
            || (row_end as i32 + row) > 1048576
        {
            panic!("Out of Range.");
        }

        // Iterate row by row, collecting cell information (do I copy)
        let mut copy_cells: Vec<Cell> = self
            .cell_collection
            .iter_all_cells_by_range_sorted_by_row(range)
            .flatten()
            .map(|cell| cell.clone())
            .collect();

        // Delete cell information as iterating through in move mode
        if is_move {
            get_coordinate_list(&range)
                .iter()
                .for_each(|(col_num, row_num)| {
                    self.cell_collection.remove(col_num, row_num);
                    self.cell_collection.remove(
                        &((*col_num as i32 + column) as u32),
                        &((*row_num as i32 + row) as u32),
                    );
                });
        }

        // repaste by setting cell values
        for cell in &mut copy_cells {
            cell.get_coordinate_mut().offset_col_num(*column);
            cell.get_coordinate_mut().offset_row_num(*row);
            self.set_cell(cell.clone());
        }

        self
    }

    /// Remove invisible garbage data.
    /// Doing so may reduce file size.
    /// Processing may take some time.
    #[inline]
    pub fn cleanup(&mut self) {
        let (_, max_row) = self.get_highest_column_and_row();
        for row in (1..(max_row + 1)).rev() {
            if self.row_dimensions.get_row_dimension(&row).is_some() {
                let mut indexes: Vec<(u32, u32)> = Vec::new();
                {
                    let cells: Vec<&Cell> = self.cell_collection.iter_cells_by_row(row).collect();
                    for cell in cells {
                        if !cell.is_visually_empty() {
                            return;
                        }
                        indexes.push((
                            *cell.get_coordinate().get_row_num(),
                            *cell.get_coordinate().get_col_num(),
                        ));
                    }
                }

                self.row_dimensions
                    .get_row_dimensions_to_hashmap_mut()
                    .remove(&row);
                for (i_row, i_col) in indexes {
                    self.cell_collection.remove(&i_col, &i_row);
                }
            }
        }
    }

    #[inline]
    pub fn copy_cell_styling<T>(&mut self, source: T, target: T)
    where
        T: Into<CellCoordinates>,
    {
        let style = self.cell_collection.get_style(source).clone();
        self.get_cell_mut(target).set_style(style);
    }

    /// Copy the style of a given Row to the target.
    /// # Arguments
    /// * `source_row_no` - Source Row Number.
    /// * `target_row_no` - Target Row Number.
    /// * `start_col` - Start Column Number. If None, minimum value
    /// * `end_col` - End Column Number. If None, maximum value
    #[inline]
    pub fn copy_row_styling(
        &mut self,
        source_row_no: &u32,
        target_row_no: &u32,
        start_col: Option<&u32>,
        end_col: Option<&u32>,
    ) {
        let mut start_no = *start_col.unwrap_or(&1);
        let mut end_no = *end_col.unwrap_or(&self.get_highest_column());

        if let Some(row_style) = self
            ._get_row_dimensions_crate()
            .get_row_dimension(source_row_no)
            .map(Row::get_style)
            .cloned()
        {
            self.get_row_dimensions_crate_mut()
                .get_row_dimension_mut(target_row_no)
                .set_style(row_style);
        }

        for col_no in start_no..=end_no {
            self.copy_cell_styling((&col_no, source_row_no), (&col_no, target_row_no));
        }
    }

    /// Copy the style of a given Column to the target.
    /// # Arguments
    /// * `source_col_no` - Source Column Number.
    /// * `target_col_no` - Target Column Number.
    /// * `start_row` - Start Row Number. If None, minimum value
    /// * `end_row` - End Row Number. If None, maximum value
    #[inline]
    pub fn copy_col_styling(
        &mut self,
        source_col_no: &u32,
        target_col_no: &u32,
        start_row: Option<&u32>,
        end_row: Option<&u32>,
    ) {
        let mut start_no = *start_row.unwrap_or(&1);
        let mut end_no = *end_row.unwrap_or(&self.get_highest_row());

        if let Some(col_style) = self
            .get_column_dimensions_crate()
            .get_column(source_col_no)
            .map(Column::get_style)
            .cloned()
        {
            self.get_column_dimensions_crate_mut()
                .get_column_mut(target_col_no)
                .set_style(col_style);
        }

        for row_no in start_no..=end_no {
            self.copy_cell_styling((source_col_no, &row_no), (target_col_no, &row_no));
        }
    }
}
impl AdjustmentCoordinate for Worksheet {
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if offset_col_num != &0 {
            // column dimensions
            self.column_dimensions
                .adjustment_insert_value(root_col_num, offset_col_num);
        }
        if offset_row_num != &0 {
            // row dimensions
            self.get_row_dimensions_crate_mut()
                .adjustment_insert_value(root_row_num, offset_row_num);
        }
        if (offset_col_num == &0 && offset_row_num == &0) {
            return;
        }

        // defined_names
        for defined_name in &mut self.defined_names {
            defined_name.adjustment_insert_coordinate_with_sheet(
                &self.title,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }

        // cell
        self.cell_collection.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );

        // worksheet_drawing
        self.worksheet_drawing.adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );

        // comments
        for comment in &mut self.comments {
            comment.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }

        // conditional styles
        for conditional_styles in &mut self.conditional_formatting_collection {
            conditional_styles.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }

        // merge cells
        for merge_cell in self.get_merge_cells_mut() {
            merge_cell.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }

        // auto filter
        if let Some(v) = self.get_auto_filter_mut() {
            v.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        };
    }

    fn adjustment_remove_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if offset_col_num != &0 {
            // column dimensions
            self.column_dimensions
                .adjustment_remove_value(root_col_num, offset_col_num);
        }
        if offset_row_num != &0 {
            // row dimensions
            self.row_dimensions
                .adjustment_remove_value(root_row_num, offset_row_num);
        }
        if (offset_col_num == &0 && offset_row_num == &0) {
            return;
        }

        // defined_names
        let title = &self.title;
        self.defined_names.retain(|defined_name| {
            !defined_name.is_remove_coordinate_with_sheet(
                &title,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            )
        });

        for defined_name in &mut self.defined_names {
            defined_name.adjustment_remove_coordinate_with_sheet(
                &self.title,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }

        // cell
        self.cell_collection.adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );

        // worksheet_drawing
        self.get_worksheet_drawing_mut()
            .adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );

        // comments
        self.comments.retain(|x| {
            !(x.is_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num))
        });
        for comment in &mut self.comments {
            comment.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }

        // conditional styles
        self.conditional_formatting_collection.retain(|x| {
            !x.is_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num)
        });
        for conditional_styles in &mut self.conditional_formatting_collection {
            conditional_styles.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }

        // merge cells
        self.get_merge_cells_mut().retain(|x| {
            !(x.is_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num))
        });
        for merge_cell in self.get_merge_cells_mut() {
            merge_cell.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }

        // auto filter
        let is_remove = match self.get_auto_filter() {
            Some(v) => v.get_range().is_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            ),
            None => false,
        };
        if is_remove {
            self.remove_auto_filter();
        }
        if let Some(v) = self.get_auto_filter_mut() {
            v.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        };
    }
}
impl AdjustmentCoordinateWithSheet for Worksheet {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if (offset_col_num == &0 && offset_row_num == &0) {
            return;
        }

        // cell formula coordinate
        let title = self.title.clone();
        self.get_cell_collection_crate_mut()
            .adjustment_insert_coordinate_with_2sheet(
                &title,
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );

        // worksheet_drawing
        self.worksheet_drawing
            .adjustment_insert_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
    }

    fn adjustment_remove_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if (offset_col_num == &0 && offset_row_num == &0) {
            return;
        }

        // cell formula coordinate
        let title = self.title.clone();
        self.get_cell_collection_crate_mut()
            .adjustment_remove_coordinate_with_2sheet(
                &title,
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );

        // worksheet_drawing
        self.worksheet_drawing
            .adjustment_remove_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
    }
}
