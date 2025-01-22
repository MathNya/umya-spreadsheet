use std::collections::HashMap;

use crate::{
    StringValue,
    helper::{
        const_str::PIVOT_CACHE_DEF_NS,
        coordinate::{
            CellCoordinates,
            column_index_from_string,
            coordinate_from_index,
            string_from_column_index,
        },
        range::{
            get_coordinate_list,
            get_start_and_end_point,
        },
    },
    reader::xlsx::worksheet::read_lite,
    structs::{
        AutoFilter,
        Cell,
        CellValue,
        Cells,
        Chart,
        Color,
        Column,
        ColumnBreaks,
        Columns,
        Comment,
        ConditionalFormatting,
        DataValidations,
        DefinedName,
        EnumValue,
        HeaderFooter,
        Hyperlink,
        Image,
        MediaObject,
        MergeCells,
        OleObjects,
        PageMargins,
        PageSetup,
        PivotTable,
        PrintOptions,
        Range,
        Row,
        RowBreaks,
        Rows,
        SharedStringTable,
        SheetFormatProperties,
        SheetProtection,
        SheetStateValues,
        SheetViews,
        Style,
        Stylesheet,
        Table,
        drawing::spreadsheet::WorksheetDrawing,
        office2010::excel::DataValidations as DataValidations2010,
        raw::RawWorksheet,
    },
    traits::{
        AdjustmentCoordinate,
        AdjustmentCoordinateWith2Sheet,
        AdjustmentCoordinateWithSheet,
        AdjustmentValue,
    },
};

/// A Worksheet Object.
#[derive(Clone, Debug, Default)]
pub struct Worksheet {
    raw_data_of_worksheet:             Option<RawWorksheet>,
    r_id:                              Box<str>,
    sheet_id:                          Box<str>,
    title:                             Box<str>,
    state:                             EnumValue<SheetStateValues>,
    cells:                             Cells,
    rows:                              Rows,
    columns:                           Columns,
    worksheet_drawing:                 WorksheetDrawing,
    sheet_state:                       Box<str>,
    page_setup:                        PageSetup,
    page_margins:                      PageMargins,
    header_footer:                     HeaderFooter,
    sheet_views:                       SheetViews,
    conditional_formatting_collection: Vec<ConditionalFormatting>,
    merge_cells:                       MergeCells,
    auto_filter:                       Option<AutoFilter>,
    comments:                          Vec<Comment>,
    active_cell:                       Box<str>,
    tab_color:                         Option<Color>,
    code_name:                         StringValue,
    ole_objects:                       OleObjects,
    defined_names:                     Vec<DefinedName>,
    print_options:                     PrintOptions,
    column_breaks:                     ColumnBreaks,
    row_breaks:                        RowBreaks,
    tables:                            Vec<Table>,
    pivot_tables:                      Vec<PivotTable>,
    data_validations:                  Option<DataValidations>,
    data_validations_2010:             Option<DataValidations2010>,
    sheet_format_properties:           SheetFormatProperties,
    sheet_protection:                  Option<SheetProtection>,
}

impl Worksheet {
    // ************************
    // Value
    // ************************

    /// Get value.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(1,
    ///   1)`
    /// # Return value
    /// * `String` - Value of the specified cell.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.sheet(0).unwrap();
    /// let value = worksheet.value("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let value = worksheet.value((1, 1));
    /// ```
    #[inline]
    pub fn value<T>(&self, coordinate: T) -> String
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.cell((col, row))
            .map(|v| v.value().into())
            .unwrap_or_default()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use value()")]
    pub fn get_value<T>(&self, coordinate: T) -> String
    where
        T: Into<CellCoordinates>,
    {
        self.value(coordinate)
    }

    /// Get value number.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)`
    /// # Return value
    /// * `Option<f64>` - Value of the specified cell.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.sheet(0).unwrap();
    /// let value = worksheet.value_number("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let value = worksheet.value_number((1, 1));
    /// ```
    #[inline]
    pub fn value_number<T>(&self, coordinate: T) -> Option<f64>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.cell((col, row)).and_then(Cell::value_number)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use value_number()")]
    pub fn get_value_number<T>(&self, coordinate: T) -> Option<f64>
    where
        T: Into<CellCoordinates>,
    {
        self.value_number(coordinate)
    }

    /// Get formatted value.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(1,
    ///   1)`
    /// # Return value
    /// * `String` - Formatted value of the specified cell.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.sheet(0).unwrap();
    /// let value = worksheet.formatted_value("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let value = worksheet.formatted_value((1, 1));
    /// ```
    #[inline]
    pub fn formatted_value<T>(&self, coordinate: T) -> String
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.cells.formatted_value_by_column_and_row(col, row)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use formatted_value()")]
    pub fn get_formatted_value<T>(&self, coordinate: T) -> String
    where
        T: Into<CellCoordinates>,
    {
        self.formatted_value(coordinate)
    }

    // ************************
    // Cell
    // ************************
    /// Get Cell List.
    #[inline]
    #[must_use]
    pub fn cells(&self) -> Vec<&Cell> {
        self.cells.collection()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use cells()")]
    pub fn get_cells(&self) -> Vec<&Cell> {
        self.cells()
    }

    #[inline]
    #[must_use]
    pub fn cells_sorted(&self) -> Vec<&Cell> {
        self.cells.collection_sorted()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use cells_sorted()")]
    pub fn get_cells_sorted(&self) -> Vec<&Cell> {
        self.cells_sorted()
    }

    /// Get Cell List in mutable.
    #[inline]
    pub fn cells_mut(&mut self) -> Vec<&mut Cell> {
        self.cells.collection_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use cells_mut()")]
    pub fn get_cells_mut(&mut self) -> Vec<&mut Cell> {
        self.cells_mut()
    }

    #[inline]
    #[must_use]
    pub fn collection_to_hashmap(&self) -> &HashMap<(u32, u32), Box<Cell>> {
        self.cells.collection_to_hashmap()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use collection_to_hashmap()")]
    pub fn get_collection_to_hashmap(&self) -> &HashMap<(u32, u32), Box<Cell>> {
        self.collection_to_hashmap()
    }

    #[inline]
    pub fn collection_to_hashmap_mut(&mut self) -> &mut HashMap<(u32, u32), Box<Cell>> {
        self.cells.collection_to_hashmap_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use collection_to_hashmap_mut()")]
    pub fn get_collection_to_hashmap_mut(&mut self) -> &mut HashMap<(u32, u32), Box<Cell>> {
        self.collection_to_hashmap_mut()
    }

    pub(crate) fn cells_stream(
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
    }

    #[deprecated(since = "3.0.0", note = "Use cells_stream()")]
    pub(crate) fn get_cells_stream(
        &self,
        shared_string_table: &SharedStringTable,
        stylesheet: &Stylesheet,
    ) -> Cells {
        self.cells_stream(shared_string_table, stylesheet)
    }

    /// (This method is crate only.)
    /// Get Cells.
    #[inline]
    pub(crate) fn cells_crate(&self) -> &Cells {
        &self.cells
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use cells_crate()")]
    pub(crate) fn get_cells_crate(&self) -> &Cells {
        self.cells_crate()
    }

    /// (This method is crate only.)
    /// Get Cells in mutable.
    #[inline]
    pub(crate) fn cells_crate_mut(&mut self) -> &mut Cells {
        &mut self.cells
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use cells_crate_mut()")]
    pub(crate) fn get_cells_crate_mut(&mut self) -> &mut Cells {
        self.cells_crate_mut()
    }

    /// Get cell.
    /// # Note
    /// Cells with unset Value and Style will return None.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(1,
    ///   1)`
    /// # Return value
    /// * `Option` - Cell in the Some.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.sheet(0).unwrap();
    /// let cell = worksheet.cell("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let cell = worksheet.cell((1, 1));
    /// ```
    #[inline]
    pub fn cell<T>(&self, coordinate: T) -> Option<&Cell>
    where
        T: Into<CellCoordinates>,
    {
        self.cells.get(coordinate)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use cell()")]
    pub fn get_cell<T>(&self, coordinate: T) -> Option<&Cell>
    where
        T: Into<CellCoordinates>,
    {
        self.cell(coordinate)
    }

    /// Get cell with mutable.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(1,
    ///   1)`
    /// # Return value
    /// * `&mut Cell` - Cell with mutable.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.sheet_mut(0).unwrap();
    /// let cell = worksheet.cell_mut("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let cell = worksheet.cell_mut((1, 1));
    /// ```
    pub fn cell_mut<T>(&mut self, coordinate: T) -> &mut Cell
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.row_dimension_mut(row);
        let row_dimension = self.row_dimension_mut(row).clone();
        let col_dimension = self.column_dimension_by_number_mut(col).clone();
        self.cells
            .get_mut((col, row), &row_dimension, &col_dimension)
    }

    #[deprecated(since = "3.0.0", note = "Use cell_mut()")]
    pub fn get_cell_mut<T>(&mut self, coordinate: T) -> &mut Cell
    where
        T: Into<CellCoordinates>,
    {
        self.cell_mut(coordinate)
    }

    #[inline]
    #[must_use]
    pub fn collection_by_column(&self, column_num: u32) -> Vec<&Cell> {
        self.cells.collection_by_column(column_num)
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use collection_by_column()")]
    pub fn get_collection_by_column(&self, column_num: u32) -> Vec<&Cell> {
        self.collection_by_column(column_num)
    }

    #[inline]
    #[must_use]
    pub fn collection_by_row(&self, row_num: u32) -> Vec<&Cell> {
        self.cells.collection_by_row(row_num)
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use collection_by_row()")]
    pub fn get_collection_by_row(&self, row_num: u32) -> Vec<&Cell> {
        self.collection_by_row(row_num)
    }

    #[inline]
    #[must_use]
    pub fn collection_by_column_to_hashmap(&self, column_num: u32) -> HashMap<u32, &Cell> {
        self.cells.collection_by_column_to_hashmap(column_num)
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
        self.cells.collection_by_row_to_hashmap(row_num)
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use collection_by_row_to_hashmap()")]
    pub fn get_collection_by_row_to_hashmap(&self, row_num: u32) -> HashMap<u32, &Cell> {
        self.collection_by_row_to_hashmap(row_num)
    }

    /// Set Cell
    /// # Arguments
    /// * `cell` - Cell
    pub fn set_cell(&mut self, cell: Cell) -> &mut Self {
        let row_dimension = self
            .row_dimension_mut(cell.coordinate().get_row_num())
            .clone();
        let col_dimension = self
            .column_dimension_by_number_mut(cell.coordinate().get_col_num())
            .clone();
        self.cells.set(cell, &row_dimension, &col_dimension);
        self
    }

    /// Remove Cell
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(1,
    ///   1)`
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
        self.cells.remove(col, row)
    }

    /// Get cell value.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(1,
    ///   1)`
    /// # Return value
    /// * `&CellValue` - `CellValue`.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.sheet(0).unwrap();
    /// let cell_value = worksheet.cell_value("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let cell_value = worksheet.cell_value((1, 1));
    /// ```
    #[inline]
    pub fn cell_value<T>(&self, coordinate: T) -> &CellValue
    where
        T: Into<CellCoordinates>,
    {
        self.cells.cell_value(coordinate)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use cell_value()")]
    pub fn get_cell_value<T>(&self, coordinate: T) -> &CellValue
    where
        T: Into<CellCoordinates>,
    {
        self.cell_value(coordinate)
    }

    /// Get cell value with mutable.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(1,
    ///   1)`
    /// # Return value
    /// * `&mut CellValue` - `CellValue` with mutable.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.sheet_mut(0).unwrap();
    /// let cell_value = worksheet.cell_value_mut("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let cell_value = worksheet.cell_value_mut((1, 1));
    /// ```
    #[inline]
    pub fn cell_value_mut<T>(&mut self, coordinate: T) -> &mut CellValue
    where
        T: Into<CellCoordinates>,
    {
        self.cell_mut(coordinate).cell_value_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use cell_value_mut()")]
    pub fn get_cell_value_mut<T>(&mut self, coordinate: T) -> &mut CellValue
    where
        T: Into<CellCoordinates>,
    {
        self.cell_value_mut(coordinate)
    }

    /// Gets the cell value by specifying an range.
    /// # Arguments
    /// * `range` - range. ex) "A1:C5"
    /// # Return value
    /// *`Vec<&CellValue>` - `CellValue` List.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(0).unwrap();
    /// let mut cell_value_List = worksheet.get_cell_value_by_range("A1:C5");
    /// ```
    #[inline]
    #[must_use]
    pub fn cell_value_by_range(&self, range: &str) -> Vec<&CellValue> {
        self.cells.cell_value_by_range(range)
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use cell_value_by_range()")]
    pub fn get_cell_value_by_range(&self, range: &str) -> Vec<&CellValue> {
        self.cell_value_by_range(range)
    }

    /// Get style.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(1,
    ///   1)`
    /// # Return value
    /// * `&Style` - Style.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let style = worksheet.get_style("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let style = worksheet.get_style((1, 1));
    /// ```
    #[inline]
    pub fn style<T>(&self, coordinate: T) -> &Style
    where
        T: Into<CellCoordinates>,
    {
        self.cells.style(coordinate)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use style()")]
    pub fn get_style<T>(&self, coordinate: T) -> &Style
    where
        T: Into<CellCoordinates>,
    {
        self.style(coordinate)
    }

    /// Get style with mutable.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) `"A1"` or `(1, 1)` or `(1,
    ///   1)`
    /// # Return value
    /// * `&mut Style` - Style with mutable.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.sheet_mut(0).unwrap();
    /// let style = worksheet.style_mut("A1");
    /// // or pass in a tuple `(col, row)`, both col and row starting at `1`
    /// let style = worksheet.style_mut((1, 1));
    /// ```
    #[inline]
    pub fn style_mut<T>(&mut self, coordinate: T) -> &mut Style
    where
        T: Into<CellCoordinates>,
    {
        self.cell_mut(coordinate).style_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use style_mut()")]
    pub fn get_style_mut<T>(&mut self, coordinate: T) -> &mut Style
    where
        T: Into<CellCoordinates>,
    {
        self.style_mut(coordinate)
    }

    #[inline]
    pub fn set_style<T>(&mut self, coordinate: T, style: Style) -> &mut Self
    where
        T: Into<CellCoordinates>,
    {
        self.cell_mut(coordinate).set_style(style);
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
    /// let mut worksheet = book.get_sheet_mut(0).unwrap();
    /// let mut style = umya_spreadsheet::Style::default();
    /// style
    ///     .get_borders_mut()
    ///     .get_bottom_mut()
    ///     .set_border_style(umya_spreadsheet::Border::BORDER_MEDIUM);
    /// worksheet.set_style_by_range("A1:A3", style);
    /// ```
    pub fn set_style_by_range(&mut self, range: &str, style: &Style) -> &mut Self {
        let coordinate_list = get_coordinate_list(range);

        let (col_num_start, row_num_start) = coordinate_list[0];
        if col_num_start == 0 {
            let (_, row_num_end) = coordinate_list[1];
            for row_num in row_num_start..=row_num_end {
                self.row_dimension_mut(row_num).set_style(style.clone());
            }
            return self;
        }
        if row_num_start == 0 {
            let (col_num_end, _) = coordinate_list[1];
            for col_num in col_num_start..=col_num_end {
                self.column_dimension_by_number_mut(col_num)
                    .set_style(style.clone());
            }
            return self;
        }

        for (col_num, row_num) in coordinate_list {
            self.set_style((col_num, row_num), style.clone());
        }
        self
    }

    // ************************
    // Comment
    // ************************
    /// Get Comments
    #[inline]
    #[must_use]
    pub fn comments(&self) -> &[Comment] {
        &self.comments
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use comments()")]
    pub fn get_comments(&self) -> &[Comment] {
        self.comments()
    }

    /// Get Comments in mutable.
    #[inline]
    pub fn comments_mut(&mut self) -> &mut Vec<Comment> {
        &mut self.comments
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use comments_mut()")]
    pub fn get_comments_mut(&mut self) -> &mut Vec<Comment> {
        self.comments_mut()
    }

    /// Get Comments convert to hashmap.
    #[inline]
    #[must_use]
    pub fn comments_to_hashmap(&self) -> HashMap<String, &Comment> {
        let mut result = HashMap::default();
        for comment in &self.comments {
            let coordinate = comment.get_coordinate().to_string();
            result.insert(coordinate, comment);
        }
        result
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use comments_to_hashmap()")]
    pub fn get_comments_to_hashmap(&self) -> HashMap<String, &Comment> {
        self.comments_to_hashmap()
    }

    /// Set Comments.
    /// # Arguments
    /// * `value` - Comment List (Vec)
    #[inline]
    pub fn set_comments(&mut self, value: impl Into<Vec<Comment>>) {
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
    #[must_use]
    pub fn has_comments(&self) -> bool {
        !self.comments.is_empty()
    }

    // ************************
    // Conditional
    // ************************
    /// Get `ConditionalFormatting` list.
    #[inline]
    #[must_use]
    pub fn conditional_formatting_collection(&self) -> &[ConditionalFormatting] {
        &self.conditional_formatting_collection
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use conditional_formatting_collection()")]
    pub fn get_conditional_formatting_collection(&self) -> &[ConditionalFormatting] {
        self.conditional_formatting_collection()
    }

    /// Set `ConditionalFormatting`.
    /// # Arguments
    /// * `value` - `ConditionalSet` List (Vec)
    #[inline]
    pub fn set_conditional_formatting_collection(
        &mut self,
        value: impl Into<Vec<ConditionalFormatting>>,
    ) {
        self.conditional_formatting_collection = value.into();
    }

    /// Add `ConditionalFormatting`.
    /// # Arguments
    /// * `value` - `ConditionalFormatting`
    #[inline]
    pub fn add_conditional_formatting_collection(&mut self, value: ConditionalFormatting) {
        self.conditional_formatting_collection.push(value);
    }

    // ************************
    // Hyperlink
    // ************************
    /// (This method is crate only.)
    /// Get Hyperlink convert to hashmap.
    pub(crate) fn hyperlink_collection_to_hashmap(&self) -> HashMap<String, &Hyperlink> {
        let mut result: HashMap<String, &Hyperlink> = HashMap::new();
        for cell in self.cells.collection() {
            if let Some(hyperlink) = cell.hyperlink() {
                let coordition = coordinate_from_index(
                    cell.coordinate().get_col_num(),
                    cell.coordinate().get_row_num(),
                );
                result.insert(coordition, hyperlink);
            }
        }
        result
    }

    #[deprecated(since = "3.0.0", note = "Use hyperlink_collection_to_hashmap()")]
    pub(crate) fn get_hyperlink_collection_to_hashmap(&self) -> HashMap<String, &Hyperlink> {
        self.hyperlink_collection_to_hashmap()
    }

    /// (This method is crate only.)
    /// Has Hyperlink
    #[inline]
    pub(crate) fn has_hyperlink(&self) -> bool {
        self.cells.has_hyperlink()
    }

    // ************************
    // Merge Cells
    // ************************
    // Get Merge Cells
    #[inline]
    #[must_use]
    pub fn merge_cells(&self) -> &[Range] {
        self.merge_cells.get_range_collection()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use merge_cells()")]
    pub fn get_merge_cells(&self) -> &[Range] {
        self.merge_cells()
    }

    // Get Merge Cells in mutable.
    #[inline]
    pub fn merge_cells_mut(&mut self) -> &mut Vec<Range> {
        self.merge_cells.get_range_collection_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use merge_cells_mut()")]
    pub fn get_merge_cells_mut(&mut self) -> &mut Vec<Range> {
        self.merge_cells_mut()
    }

    // Add Merge Cells.
    /// # Arguments
    /// * `range` - Range. ex) "A1:C5"
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(0).unwrap();
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
    pub(crate) fn merge_cells_crate(&self) -> &MergeCells {
        &self.merge_cells
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use merge_cells_crate()")]
    pub(crate) fn get_merge_cells_crate(&self) -> &MergeCells {
        self.merge_cells_crate()
    }

    /// (This method is crate only.)
    // Get Merge Cells Object in mutable.
    #[inline]
    pub(crate) fn merge_cells_crate_mut(&mut self) -> &mut MergeCells {
        &mut self.merge_cells
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use merge_cells_crate_mut()")]
    pub(crate) fn get_merge_cells_crate_mut(&mut self) -> &mut MergeCells {
        self.merge_cells_crate_mut()
    }

    // ************************
    // Auto Filter
    // ************************
    // Get Auto Filter (Option).
    #[inline]
    #[must_use]
    pub fn auto_filter(&self) -> Option<&AutoFilter> {
        self.auto_filter.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use auto_filter()")]
    pub fn get_auto_filter(&self) -> Option<&AutoFilter> {
        self.auto_filter()
    }

    // Get Auto Filter (Option) in mutable.
    #[inline]
    pub fn auto_filter_mut(&mut self) -> Option<&mut AutoFilter> {
        self.auto_filter.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use auto_filter_mut()")]
    pub fn get_auto_filter_mut(&mut self) -> Option<&mut AutoFilter> {
        self.auto_filter_mut()
    }

    // Set Auto Filter.
    /// # Arguments
    /// * `range` - Range. ex) "A2:K2"
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.sheet_mut(0).unwrap();
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
    #[must_use]
    pub fn column_dimensions(&self) -> &[Column] {
        self.columns.get_column_collection()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use column_dimensions()")]
    pub fn get_column_dimensions(&self) -> &[Column] {
        self.column_dimensions()
    }

    /// Get Column Dimension List in mutable.
    #[inline]
    pub fn column_dimensions_mut(&mut self) -> &mut Vec<Column> {
        self.columns.get_column_collection_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use column_dimensions_mut()")]
    pub fn get_column_dimensions_mut(&mut self) -> &mut Vec<Column> {
        self.column_dimensions_mut()
    }

    /// Calculation Auto Width.
    #[inline]
    pub fn calculation_auto_width(&mut self) -> &mut Self {
        let cells = self.cells_crate().clone();
        let merge_cells = self.merge_cells_crate().clone();
        self.column_dimensions_crate_mut()
            .calculation_auto_width(&cells, &merge_cells);
        self
    }

    /// Get Column Dimension.
    /// # Arguments
    /// * `column` - Column Char. ex) "A"
    #[inline]
    #[must_use]
    pub fn column_dimension(&self, column: &str) -> Option<&Column> {
        self.column_dimension_by_number(column_index_from_string(column))
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use column_dimension()")]
    pub fn get_column_dimension(&self, column: &str) -> Option<&Column> {
        self.column_dimension(column)
    }

    /// Get Column Dimension in mutable.
    /// # Arguments
    /// * `column` - Column Char. ex) "A"
    #[inline]
    pub fn column_dimension_mut(&mut self, column: &str) -> &mut Column {
        self.column_dimension_by_number_mut(column_index_from_string(column))
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use column_dimension_mut()")]
    pub fn get_column_dimension_mut(&mut self, column: &str) -> &mut Column {
        self.column_dimension_mut(column)
    }

    /// Get Column Dimension.
    /// # Arguments
    /// * `col` - Column Number.
    #[inline]
    #[must_use]
    pub fn column_dimension_by_number(&self, col: u32) -> Option<&Column> {
        self.column_dimensions_crate().get_column(col)
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use column_dimension_by_number()")]
    pub fn get_column_dimension_by_number(&self, col: u32) -> Option<&Column> {
        self.column_dimension_by_number(col)
    }

    /// Get Column Dimension in mutable.
    /// # Arguments
    /// * `col` - Column Number.
    #[inline]
    pub fn column_dimension_by_number_mut(&mut self, col: u32) -> &mut Column {
        self.column_dimensions_crate_mut().get_column_mut(col)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use column_dimension_by_number_mut()")]
    pub fn get_column_dimension_by_number_mut(&mut self, col: u32) -> &mut Column {
        self.column_dimension_by_number_mut(col)
    }

    /// (This method is crate only.)
    /// Get Column Dimension.
    #[inline]
    pub(crate) fn column_dimensions_crate(&self) -> &Columns {
        &self.columns
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use column_dimensions_crate()")]
    pub(crate) fn get_column_dimensions_crate(&self) -> &Columns {
        self.column_dimensions_crate()
    }

    /// (This method is crate only.)
    /// Get Column Dimension in mutable.
    #[inline]
    pub(crate) fn column_dimensions_crate_mut(&mut self) -> &mut Columns {
        &mut self.columns
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use column_dimensions_crate_mut()")]
    pub(crate) fn get_column_dimensions_crate_mut(&mut self) -> &mut Columns {
        self.column_dimensions_crate_mut()
    }

    /// (This method is crate only.)
    /// Set Column Dimension.
    #[inline]
    pub(crate) fn set_column_dimensions_crate(&mut self, value: Columns) -> &mut Self {
        self.columns = value;
        self
    }

    // ************************
    // Row Dimensions
    // ************************
    #[inline]
    #[must_use]
    pub fn has_sheet_data(&self) -> bool {
        self.rows.has_sheet_data()
    }

    /// Get Row Dimension List.
    #[inline]
    #[must_use]
    pub fn row_dimensions(&self) -> Vec<&Row> {
        self.rows.get_row_dimensions()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use row_dimensions()")]
    pub fn get_row_dimensions(&self) -> Vec<&Row> {
        self.row_dimensions()
    }

    /// Get Row Dimension List in mutable.
    #[inline]
    pub fn row_dimensions_mut(&mut self) -> Vec<&mut Row> {
        self.rows.get_row_dimensions_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_dimensions_mut()")]
    pub fn get_row_dimensions_mut(&mut self) -> Vec<&mut Row> {
        self.row_dimensions_mut()
    }

    /// Get Row Dimension convert Hashmap.
    #[inline]
    #[must_use]
    pub fn row_dimensions_to_hashmap(&self) -> &HashMap<u32, Box<Row>> {
        self.rows.get_row_dimensions_to_hashmap()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use row_dimensions_to_hashmap()")]
    pub fn get_row_dimensions_to_hashmap(&self) -> &HashMap<u32, Box<Row>> {
        self.row_dimensions_to_hashmap()
    }

    #[inline]
    pub fn row_dimensions_to_hashmap_mut(&mut self) -> &mut HashMap<u32, Box<Row>> {
        self.rows.get_row_dimensions_to_hashmap_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_dimensions_to_hashmap_mut()")]
    pub fn get_row_dimensions_to_hashmap_mut(&mut self) -> &mut HashMap<u32, Box<Row>> {
        self.row_dimensions_to_hashmap_mut()
    }

    /// Get Row Dimension.
    #[inline]
    #[must_use]
    pub fn row_dimension(&self, row: u32) -> Option<&Row> {
        self.rows.get_row_dimension(row)
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use row_dimension()")]
    pub fn get_row_dimension(&self, row: u32) -> Option<&Row> {
        self.row_dimension(row)
    }

    /// Get Row Dimension in mutable.
    #[inline]
    pub fn row_dimension_mut(&mut self, row: u32) -> &mut Row {
        self.rows.get_row_dimension_mut(row)
    }

    /// Get Row Dimension in mutable.
    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_dimension_mut()")]
    pub fn get_row_dimension_mut(&mut self, row: u32) -> &mut Row {
        self.row_dimension_mut(row)
    }

    /// (This method is crate only.)
    /// Set Row Dimension.
    #[inline]
    pub(crate) fn set_row_dimension(&mut self, value: Row) -> &mut Self {
        self.rows.set_row_dimension(value);
        self
    }

    /// (This method is crate only.)
    /// Get Row Dimension in mutable.
    #[inline]
    pub(crate) fn row_dimensions_crate_mut(&mut self) -> &mut Rows {
        &mut self.rows
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_dimensions_crate_mut()")]
    pub(crate) fn get_row_dimensions_crate_mut(&mut self) -> &mut Rows {
        self.row_dimensions_crate_mut()
    }

    /// (This method is crate only.)
    /// Get Row Dimension.
    #[inline]
    pub(crate) fn row_dimensions_crate(&self) -> &Rows {
        &self.rows
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_dimensions_crate()")]
    pub(crate) fn get_row_dimensions_crate(&self) -> &Rows {
        self.row_dimensions_crate()
    }

    // ************************
    // WorksheetDrawing
    // ************************
    /// Get `WorksheetDrawing`.
    #[inline]
    #[must_use]
    pub fn worksheet_drawing(&self) -> &WorksheetDrawing {
        &self.worksheet_drawing
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use worksheet_drawing()")]
    pub fn get_worksheet_drawing(&self) -> &WorksheetDrawing {
        self.worksheet_drawing()
    }

    /// Get `WorksheetDrawing` in mutable.
    #[inline]
    pub fn worksheet_drawing_mut(&mut self) -> &mut WorksheetDrawing {
        &mut self.worksheet_drawing
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use worksheet_drawing_mut()")]
    pub fn get_worksheet_drawing_mut(&mut self) -> &mut WorksheetDrawing {
        self.worksheet_drawing_mut()
    }

    /// Set `WorksheetDrawing`.
    /// # Arguments
    /// * `value` - `WorksheetDrawing`
    #[inline]
    pub fn set_worksheet_drawing(&mut self, value: WorksheetDrawing) {
        self.worksheet_drawing = value;
    }

    /// Has `WorksheetDrawing`.
    #[inline]
    #[must_use]
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
    /// let mut worksheet = book.sheet_mut(0).unwrap();
    /// worksheet.insert_new_row(&2, &3);
    /// ```
    #[inline]
    pub fn insert_new_row(&mut self, row_index: u32, num_rows: u32) {
        let title = &*self.title.clone();
        self.adjustment_insert_coordinate(0, 0, row_index, num_rows);
        self.adjustment_insert_coordinate_with_sheet(title, 0, 0, row_index, num_rows);
    }

    /// Adjust for references to other sheets.
    #[inline]
    pub fn insert_new_row_from_other_sheet(
        &mut self,
        sheet_name: &str,
        row_index: u32,
        num_rows: u32,
    ) {
        self.adjustment_insert_coordinate(0, 0, row_index, num_rows);
        self.adjustment_insert_coordinate_with_sheet(sheet_name, 0, 0, row_index, num_rows);
    }

    /// Insert new columns.
    /// # Arguments
    /// * `column` - Specify point of insert. ex) "B"
    /// * `num_columns` - Specify number to insert. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.sheet_mut(0).unwrap();
    /// worksheet.insert_new_column("B", &3);
    /// ```
    #[inline]
    pub fn insert_new_column(&mut self, column: &str, num_columns: u32) {
        self.insert_new_column_by_index(column_index_from_string(column), num_columns);
    }

    /// Adjust for references to other sheets.
    #[inline]
    pub fn insert_new_column_from_other_sheet(
        &mut self,
        sheet_name: &str,
        column: &str,
        num_columns: u32,
    ) {
        self.insert_new_column_by_index_from_other_sheet(
            sheet_name,
            column_index_from_string(column),
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
    /// let mut worksheet = book.sheet_mut(0).unwrap();
    /// worksheet.insert_new_column_by_index(&2, &3);
    /// ```
    #[inline]
    pub fn insert_new_column_by_index(&mut self, column_index: u32, num_columns: u32) {
        let title = &*self.title.clone();
        self.adjustment_insert_coordinate(column_index, num_columns, 0, 0);
        self.adjustment_insert_coordinate_with_sheet(title, column_index, num_columns, 0, 0);
    }

    /// Adjust for references to other sheets.
    #[inline]
    pub fn insert_new_column_by_index_from_other_sheet(
        &mut self,
        sheet_name: &str,
        column_index: u32,
        num_columns: u32,
    ) {
        self.adjustment_insert_coordinate(column_index, num_columns, 0, 0);
        self.adjustment_insert_coordinate_with_sheet(sheet_name, column_index, num_columns, 0, 0);
    }

    /// Remove rows.
    /// # Arguments
    /// * `row_index` - Specify point of remove. ex) 1
    /// * `num_rows` - Specify number to remove. ex) 2
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.sheet_mut(0).unwrap();
    /// worksheet.remove_row(&2, &3);
    /// ```
    #[inline]
    pub fn remove_row(&mut self, row_index: u32, num_rows: u32) {
        let title = &*self.title.clone();
        self.adjustment_remove_coordinate(0, 0, row_index, num_rows);
        self.adjustment_remove_coordinate_with_sheet(title, 0, 0, row_index, num_rows);
    }

    /// Adjust for references to other sheets.
    #[inline]
    pub fn remove_row_from_other_sheet(&mut self, sheet_name: &str, row_index: u32, num_rows: u32) {
        self.adjustment_remove_coordinate(0, 0, row_index, num_rows);
        self.adjustment_remove_coordinate_with_sheet(sheet_name, 0, 0, row_index, num_rows);
    }

    /// Remove columns.
    /// # Arguments
    /// * `sheet_name` - Specify the sheet name. ex) "Sheet1"
    /// * `column` - Specify point of remove. ex) "B"
    /// * `num_columns` - Specify number to remove. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.sheet_mut(0).unwrap();
    /// worksheet.remove_column("B", &3);
    /// ```
    #[inline]
    pub fn remove_column(&mut self, column: &str, num_columns: u32) {
        self.remove_column_by_index(column_index_from_string(column), num_columns);
    }

    /// Adjust for references to other sheets.
    #[inline]
    pub fn remove_column_from_other_sheet(
        &mut self,
        sheet_name: &str,
        column: &str,
        num_columns: u32,
    ) {
        self.remove_column_by_index_from_other_sheet(
            sheet_name,
            column_index_from_string(column),
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
    /// let mut worksheet = book.sheet_mut(0).unwrap();
    /// worksheet.remove_column_by_index(&2, &3);
    /// ```
    #[inline]
    pub fn remove_column_by_index(&mut self, column_index: u32, num_columns: u32) {
        let title = &*self.title.clone();
        self.adjustment_remove_coordinate(column_index, num_columns, 0, 0);
        self.adjustment_remove_coordinate_with_sheet(title, column_index, num_columns, 0, 0);
    }

    /// Adjust for references to other sheets.
    #[inline]
    pub fn remove_column_by_index_from_other_sheet(
        &mut self,
        sheet_name: &str,
        column_index: u32,
        num_columns: u32,
    ) {
        self.adjustment_remove_coordinate(column_index, num_columns, 0, 0);
        self.adjustment_remove_coordinate_with_sheet(sheet_name, column_index, num_columns, 0, 0);
    }

    /// Get Code Name.
    #[inline]
    #[must_use]
    pub fn code_name(&self) -> Option<&str> {
        self.code_name.get_value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use code_name()")]
    pub fn get_code_name(&self) -> Option<&str> {
        self.code_name()
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
    #[must_use]
    pub fn header_footer(&self) -> &HeaderFooter {
        &self.header_footer
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use header_footer()")]
    pub fn get_header_footer(&self) -> &HeaderFooter {
        self.header_footer()
    }

    /// Get Header Footer in mutable.
    #[inline]
    pub fn header_footer_mut(&mut self) -> &mut HeaderFooter {
        &mut self.header_footer
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use header_footer_mut()")]
    pub fn get_header_footer_mut(&mut self) -> &mut HeaderFooter {
        self.header_footer_mut()
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
    #[must_use]
    pub fn active_cell(&self) -> &str {
        &self.active_cell
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use active_cell()")]
    pub fn get_active_cell(&self) -> &str {
        self.active_cell()
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
    pub(crate) fn r_id(&self) -> &str {
        &self.r_id
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use r_id()")]
    pub(crate) fn get_r_id(&self) -> &str {
        self.r_id()
    }

    /// (This method is crate only.)
    /// Set r Id.
    #[inline]
    pub(crate) fn set_r_id<S: Into<String>>(&mut self, value: S) {
        self.r_id = value.into().into_boxed_str();
    }

    /// Get Sheet Id.
    #[inline]
    #[must_use]
    pub fn sheet_id(&self) -> &str {
        &self.sheet_id
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use sheet_id()")]
    pub fn get_sheet_id(&self) -> &str {
        self.sheet_id()
    }

    /// (This method is crate only.)
    /// Set Sheet Id.
    #[inline]
    pub(crate) fn set_sheet_id<S: Into<String>>(&mut self, value: S) {
        self.sheet_id = value.into().into_boxed_str();
    }

    /// Has Code Name.
    #[inline]
    #[must_use]
    pub fn has_code_name(&self) -> bool {
        self.code_name.has_value()
    }

    /// Get Tab Color.
    #[inline]
    #[must_use]
    pub fn tab_color(&self) -> Option<&Color> {
        self.tab_color.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use tab_color()")]
    pub fn get_tab_color(&self) -> Option<&Color> {
        self.tab_color()
    }

    /// Get Tab Color in mutable.
    #[inline]
    pub fn tab_color_mut(&mut self) -> &mut Color {
        if self.tab_color.is_some() {
            return self.tab_color.as_mut().unwrap();
        }
        self.set_tab_color(Color::default());
        self.tab_color.as_mut().unwrap()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use tab_color_mut()")]
    pub fn get_tab_color_mut(&mut self) -> &mut Color {
        self.tab_color_mut()
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
    #[must_use]
    pub fn calculate_worksheet_dimension(&self) -> String {
        let (column, row) = self.cells.highest_column_and_row();
        if row == 0 {
            return "A1".to_string();
        }
        let column_str = string_from_column_index(column);
        format!("A1:{column_str}{row}")
    }

    // Get Highest Column and Row Index
    /// # Return value
    /// *`(u32, u32)` - (column, row)
    #[inline]
    #[must_use]
    pub fn highest_column_and_row(&self) -> (u32, u32) {
        self.cells.highest_column_and_row()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use highest_column_and_row()")]
    pub fn get_highest_column_and_row(&self) -> (u32, u32) {
        self.highest_column_and_row()
    }

    // Get Highest Column Index
    #[inline]
    #[must_use]
    pub fn highest_column(&self) -> u32 {
        let (column, _row) = self.cells.highest_column_and_row();
        column
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use highest_column()")]
    pub fn get_highest_column(&self) -> u32 {
        self.highest_column()
    }

    // Get Highest Row Index
    #[inline]
    #[must_use]
    pub fn highest_row(&self) -> u32 {
        let (_column, row) = self.cells.highest_column_and_row();
        row
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use highest_row()")]
    pub fn get_highest_row(&self) -> u32 {
        self.highest_row()
    }

    /// Get `SheetName`.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.title
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use name()")]
    pub fn get_name(&self) -> &str {
        self.name()
    }

    /// Set `SheetName`.
    /// # Arguments
    /// * `sheet_name` - Sheet Name. [Caution] no duplicate other worksheet.
    pub fn set_name<S: Into<String>>(&mut self, sheet_name: S) -> &mut Self {
        self.title = sheet_name.into().into_boxed_str();
        let title = self.name().to_string();
        for defined_name in self.defined_names_mut() {
            defined_name.set_sheet_name(&title);
        }
        self
    }

    #[inline]
    pub(crate) fn has_state(&self) -> bool {
        self.state.has_value()
    }

    #[inline]
    #[must_use]
    pub fn state(&self) -> &SheetStateValues {
        self.state.get_value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use state()")]
    pub fn get_state(&self) -> &SheetStateValues {
        self.state()
    }

    #[inline]
    pub(crate) fn state_str(&self) -> &str {
        self.state.get_value_string()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use state_str()")]
    pub(crate) fn get_state_str(&self) -> &str {
        self.state_str()
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
    #[must_use]
    pub fn sheet_state(&self) -> &str {
        &self.sheet_state
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use sheet_state()")]
    pub fn get_sheet_state(&self) -> &str {
        self.sheet_state()
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
    #[must_use]
    pub fn page_setup(&self) -> &PageSetup {
        &self.page_setup
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use page_setup()")]
    pub fn get_page_setup(&self) -> &PageSetup {
        self.page_setup()
    }

    // Get Page Setup in mutable.
    #[inline]
    pub fn page_setup_mut(&mut self) -> &mut PageSetup {
        &mut self.page_setup
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use page_setup_mut()")]
    pub fn get_page_setup_mut(&mut self) -> &mut PageSetup {
        self.page_setup_mut()
    }

    /// Set Page Setup.
    /// # Arguments
    /// * `value` - `PageSetup`.
    #[inline]
    pub fn set_page_setup(&mut self, value: PageSetup) -> &mut Self {
        self.page_setup = value;
        self
    }

    // Get Page Margins.
    #[inline]
    #[must_use]
    pub fn page_margins(&self) -> &PageMargins {
        &self.page_margins
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use page_margins()")]
    pub fn get_page_margins(&self) -> &PageMargins {
        self.page_margins()
    }

    // Get Page Margins in mutable.
    #[inline]
    pub fn page_margins_mut(&mut self) -> &mut PageMargins {
        &mut self.page_margins
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use page_margins_mut()")]
    pub fn get_page_margins_mut(&mut self) -> &mut PageMargins {
        self.page_margins_mut()
    }

    /// Set Page Margins.
    /// # Arguments
    /// * `value` - `PageMargins`.
    #[inline]
    pub fn set_page_margins(&mut self, value: PageMargins) -> &mut Self {
        self.page_margins = value;
        self
    }

    // Get SheetViews.
    #[inline]
    #[must_use]
    pub fn sheets_views(&self) -> &SheetViews {
        &self.sheet_views
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use sheets_views()")]
    pub fn get_sheets_views(&self) -> &SheetViews {
        self.sheets_views()
    }

    // Get SheetViews in mutable.
    #[inline]
    pub fn sheet_views_mut(&mut self) -> &mut SheetViews {
        &mut self.sheet_views
    }

    // Get SheetViews in mutable.
    #[inline]
    #[deprecated(since = "3.0.0", note = "Use sheet_views_mut()")]
    pub fn get_sheet_views_mut(&mut self) -> &mut SheetViews {
        self.sheet_views_mut()
    }

    /// Set `SheetViews`.
    /// # Arguments
    /// * `value` - `SheetViews`.
    #[inline]
    pub fn set_sheets_views(&mut self, value: SheetViews) -> &mut Self {
        self.sheet_views = value;
        self
    }

    // Get Ole Objects.
    #[inline]
    #[must_use]
    pub fn ole_objects(&self) -> &OleObjects {
        &self.ole_objects
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use get_ole_objects()")]
    pub fn get_ole_objects(&self) -> &OleObjects {
        self.ole_objects()
    }

    // Get Ole Objects in mutable.
    #[inline]
    pub fn ole_objects_mut(&mut self) -> &mut OleObjects {
        &mut self.ole_objects
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use ole_objects_mut()")]
    pub fn get_ole_objects_mut(&mut self) -> &mut OleObjects {
        self.ole_objects_mut()
    }

    /// Set Ole Objects.
    /// # Arguments
    /// * `value` - `OleObjects`.
    #[inline]
    pub fn set_ole_objects(&mut self, value: OleObjects) -> &mut Self {
        self.ole_objects = value;
        self
    }

    /// Get Defined Name (Vec).
    #[inline]
    #[must_use]
    pub fn defined_names(&self) -> &[DefinedName] {
        &self.defined_names
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use defined_names()")]
    pub fn get_defined_names(&self) -> &[DefinedName] {
        self.defined_names()
    }

    /// Get Defined Name (Vec) in mutable.
    #[inline]
    pub fn defined_names_mut(&mut self) -> &mut Vec<DefinedName> {
        &mut self.defined_names
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use defined_names_mut()")]
    pub fn get_defined_names_mut(&mut self) -> &mut Vec<DefinedName> {
        self.defined_names_mut()
    }

    /// Set Defined Name (Vec).
    /// # Arguments
    /// * `value` - Vec<DefinedName>.
    #[inline]
    pub fn set_defined_names(&mut self, value: impl Into<Vec<DefinedName>>) {
        self.defined_names = value.into();
    }

    /// Add Defined Name.
    /// # Arguments
    /// * `value` - `DefinedName`.
    #[inline]
    pub fn add_defined_names(&mut self, value: DefinedName) {
        self.defined_names.push(value);
    }

    /// Add Defined Name.
    /// # Arguments
    /// * `name` - Name. ex) "`DefinedName01`"
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
    #[must_use]
    pub fn print_options(&self) -> &PrintOptions {
        &self.print_options
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use print_options()")]
    pub fn get_print_options(&self) -> &PrintOptions {
        self.print_options()
    }

    /// Get Print Options in mutable.
    #[inline]
    pub fn print_options_mut(&mut self) -> &mut PrintOptions {
        &mut self.print_options
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use print_options_mut()")]
    pub fn get_print_options_mut(&mut self) -> &mut PrintOptions {
        self.print_options_mut()
    }

    /// Set Print Options.
    /// # Arguments
    /// * `value` - `PrintOptions`.
    #[inline]
    pub fn set_print_options(&mut self, value: PrintOptions) -> &mut Self {
        self.print_options = value;
        self
    }

    /// Get Column Breaks.
    #[inline]
    #[must_use]
    pub fn column_breaks(&self) -> &ColumnBreaks {
        &self.column_breaks
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use column_breaks()")]
    pub fn get_column_breaks(&self) -> &ColumnBreaks {
        self.column_breaks()
    }

    /// Get Column Breaks in mutable.
    #[inline]
    pub fn column_breaks_mut(&mut self) -> &mut ColumnBreaks {
        &mut self.column_breaks
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use column_breaks_mut()")]
    pub fn get_column_breaks_mut(&mut self) -> &mut ColumnBreaks {
        self.column_breaks_mut()
    }

    /// Set Column Breaks.
    /// # Arguments
    /// * `value` - `ColumnBreaks`.
    #[inline]
    pub fn set_column_breaks(&mut self, value: ColumnBreaks) -> &mut Self {
        self.column_breaks = value;
        self
    }

    /// Get Row Breaks.
    #[inline]
    #[must_use]
    pub fn row_breaks(&self) -> &RowBreaks {
        &self.row_breaks
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use row_breaks()")]
    pub fn get_row_breaks(&self) -> &RowBreaks {
        self.row_breaks()
    }

    /// Get Row Breaks in mutable.
    #[inline]
    pub fn row_breaks_mut(&mut self) -> &mut RowBreaks {
        &mut self.row_breaks
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use row_breaks_mut()")]
    pub fn get_row_breaks_mut(&mut self) -> &mut RowBreaks {
        self.row_breaks_mut()
    }

    /// Set Row Breaks.
    /// # Arguments
    /// * `value` - `RowBreaks`.
    #[inline]
    pub fn set_row_breaks(&mut self, value: RowBreaks) -> &mut Self {
        self.row_breaks = value;
        self
    }

    #[inline]
    #[must_use]
    pub fn has_table(&self) -> bool {
        !self.tables.is_empty()
    }

    #[inline]
    pub fn add_table(&mut self, table: Table) {
        self.tables.push(table);
    }

    #[inline]
    #[must_use]
    pub fn tables(&self) -> &[Table] {
        &self.tables
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use tables()")]
    pub fn get_tables(&self) -> &[Table] {
        self.tables()
    }

    #[inline]
    pub fn tables_mut(&mut self) -> &mut Vec<Table> {
        &mut self.tables
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use tables_mut()")]
    pub fn get_tables_mut(&mut self) -> &mut Vec<Table> {
        self.tables_mut()
    }

    #[inline]
    #[must_use]
    pub fn has_pivot_table(&self) -> bool {
        !self.pivot_tables.is_empty()
    }

    #[inline]
    pub fn add_pivot_table(&mut self, pivot_table: PivotTable) {
        self.pivot_tables.push(pivot_table);
    }

    #[inline]
    #[must_use]
    pub fn pivot_tables(&self) -> &[PivotTable] {
        &self.pivot_tables
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use pivot_tables()")]
    pub fn get_pivot_tables(&self) -> &[PivotTable] {
        self.pivot_tables()
    }

    #[inline]
    pub fn pivot_tables_mut(&mut self) -> &mut Vec<PivotTable> {
        &mut self.pivot_tables
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use pivot_tables_mut()")]
    pub fn get_pivot_tables_mut(&mut self) -> &mut Vec<PivotTable> {
        self.pivot_tables_mut()
    }

    #[inline]
    #[must_use]
    pub fn data_validations(&self) -> Option<&DataValidations> {
        self.data_validations.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use data_validations()")]
    pub fn get_data_validations(&self) -> Option<&DataValidations> {
        self.data_validations()
    }

    #[inline]
    pub fn data_validations_mut(&mut self) -> Option<&mut DataValidations> {
        self.data_validations.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use data_validations_mut()")]
    pub fn get_data_validations_mut(&mut self) -> Option<&mut DataValidations> {
        self.data_validations_mut()
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
    #[must_use]
    pub fn data_validations_2010(&self) -> Option<&DataValidations2010> {
        self.data_validations_2010.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use data_validations_mut()")]
    pub fn get_data_validations_2010(&self) -> Option<&DataValidations2010> {
        self.data_validations_2010()
    }

    #[inline]
    pub fn data_validations_2010_mut(&mut self) -> Option<&mut DataValidations2010> {
        self.data_validations_2010.as_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use data_validations_2010_mut()")]
    pub fn get_data_validations_2010_mut(&mut self) -> Option<&mut DataValidations2010> {
        self.data_validations_2010_mut()
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
    #[must_use]
    pub fn sheet_format_properties(&self) -> &SheetFormatProperties {
        &self.sheet_format_properties
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use sheet_format_properties()")]
    pub fn get_sheet_format_properties(&self) -> &SheetFormatProperties {
        self.sheet_format_properties()
    }

    #[inline]
    pub fn sheet_format_properties_mut(&mut self) -> &mut SheetFormatProperties {
        &mut self.sheet_format_properties
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use sheet_format_properties_mut()")]
    pub fn get_sheet_format_properties_mut(&mut self) -> &mut SheetFormatProperties {
        self.sheet_format_properties_mut()
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
    #[must_use]
    pub fn image_collection(&self) -> &[Image] {
        self.worksheet_drawing().get_image_collection()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use image_collection()")]
    pub fn get_image_collection(&self) -> &[Image] {
        self.image_collection()
    }

    /// Outputs all images contained in the worksheet.
    /// # Return value
    /// * `&mut Vec<Image>` - Image Object List.
    #[inline]
    pub fn image_collection_mut(&mut self) -> &mut Vec<Image> {
        self.worksheet_drawing_mut().get_image_collection_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use image_collection_mut()")]
    pub fn get_image_collection_mut(&mut self) -> &mut Vec<Image> {
        self.image_collection_mut()
    }

    #[inline]
    pub fn add_image(&mut self, value: Image) -> &mut Self {
        self.worksheet_drawing_mut().add_image(value);
        self
    }

    #[inline]
    pub fn image<T>(&self, coordinate: T) -> Option<&Image>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.worksheet_drawing().get_image(col, row)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use image()")]
    pub fn get_image<T>(&self, coordinate: T) -> Option<&Image>
    where
        T: Into<CellCoordinates>,
    {
        self.image(coordinate)
    }

    #[inline]
    pub fn image_mut<T>(&mut self, coordinate: T) -> Option<&mut Image>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.worksheet_drawing_mut().get_image_mut(col, row)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use image_mut()")]
    pub fn get_image_mut<T>(&mut self, coordinate: T) -> Option<&mut Image>
    where
        T: Into<CellCoordinates>,
    {
        self.image_mut(coordinate)
    }

    #[inline]
    pub fn image_by_column_and_row_mut(&mut self, col: u32, row: u32) -> Option<&mut Image> {
        self.worksheet_drawing_mut().get_image_mut(col, row)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use image_by_column_and_row_mut()")]
    pub fn get_image_by_column_and_row_mut(&mut self, col: u32, row: u32) -> Option<&mut Image> {
        self.image_by_column_and_row_mut(col, row)
    }

    #[inline]
    pub fn images<T>(&self, coordinate: T) -> Vec<&Image>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.worksheet_drawing().get_images(col, row)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use images()")]
    pub fn get_images<T>(&self, coordinate: T) -> Vec<&Image>
    where
        T: Into<CellCoordinates>,
    {
        self.images(coordinate)
    }

    #[inline]
    pub fn images_mut<T>(&mut self, coordinate: T) -> Vec<&mut Image>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.worksheet_drawing_mut().get_images_mut(col, row)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use images_mut()")]
    pub fn get_images_mut<T>(&mut self, coordinate: T) -> Vec<&mut Image>
    where
        T: Into<CellCoordinates>,
    {
        self.images_mut(coordinate)
    }

    /// Outputs all Charts contained in the worksheet.
    /// # Return value
    /// * `&Vec<Chart>` - Chart Object List.
    #[inline]
    #[must_use]
    pub fn chart_collection(&self) -> &[Chart] {
        self.worksheet_drawing().get_chart_collection()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use chart_collection()")]
    pub fn get_chart_collection(&self) -> &[Chart] {
        self.chart_collection()
    }

    /// Outputs all Charts contained in the worksheet.
    /// # Return value
    /// * `&mut Vec<Chart>` - Chart Object List.
    #[inline]
    pub fn chart_collection_mut(&mut self) -> &mut Vec<Chart> {
        self.worksheet_drawing_mut().get_chart_collection_mut()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use chart_collection_mut()")]
    pub fn get_chart_collection_mut(&mut self) -> &mut Vec<Chart> {
        self.chart_collection_mut()
    }

    #[inline]
    pub fn add_chart(&mut self, value: Chart) -> &mut Self {
        self.worksheet_drawing_mut().add_chart_collection(value);
        self
    }

    #[inline]
    pub fn chart<T>(&self, coordinate: T) -> Option<&Chart>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.worksheet_drawing().get_chart(col, row)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use chart()")]
    pub fn get_chart<T>(&self, coordinate: T) -> Option<&Chart>
    where
        T: Into<CellCoordinates>,
    {
        self.chart(coordinate)
    }

    #[inline]
    pub fn chart_mut<T>(&mut self, coordinate: T) -> Option<&mut Chart>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.worksheet_drawing_mut().get_chart_mut(col, row)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use chart_mut()")]
    pub fn get_chart_mut<T>(&mut self, coordinate: T) -> Option<&mut Chart>
    where
        T: Into<CellCoordinates>,
    {
        self.chart_mut(coordinate)
    }

    #[inline]
    pub fn charts<T>(&self, coordinate: T) -> Vec<&Chart>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.worksheet_drawing().get_charts(col, row)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use charts()")]
    pub fn get_charts<T>(&self, coordinate: T) -> Vec<&Chart>
    where
        T: Into<CellCoordinates>,
    {
        self.charts(coordinate)
    }

    #[inline]
    pub fn charts_mut<T>(&mut self, coordinate: T) -> Vec<&mut Chart>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.worksheet_drawing_mut().get_charts_mut(col, row)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use charts_mut()")]
    pub fn get_charts_mut<T>(&mut self, coordinate: T) -> Vec<&mut Chart>
    where
        T: Into<CellCoordinates>,
    {
        self.charts_mut(coordinate)
    }

    /// Outputs all media contained in the worksheet.
    /// # Return value
    /// * `Vec<&MediaObject>` - Media Object List.
    pub(crate) fn media_object_collection(&self) -> Vec<&MediaObject> {
        let mut list: Vec<&MediaObject> = Vec::new();
        for image in self.worksheet_drawing().get_image_collection() {
            for media_object in image.get_media_object() {
                let is_new = !list
                    .iter()
                    .any(|v| v.get_image_name() == media_object.get_image_name());
                if is_new {
                    list.push(media_object);
                }
            }
        }
        for ole_objects in self.ole_objects().get_ole_object() {
            let media_object = ole_objects.get_embedded_object_properties().get_image();
            let is_new = !list
                .iter()
                .any(|v| v.get_image_name() == media_object.get_image_name());
            if is_new {
                list.push(media_object);
            }
        }
        for ole_objects in self.ole_objects().get_ole_object() {
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
        for comment in self.comments() {
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

    #[deprecated(since = "3.0.0", note = "Use media_object_collection()")]
    pub(crate) fn get_media_object_collection(&self) -> Vec<&MediaObject> {
        self.media_object_collection()
    }

    pub(crate) fn pivot_cache_definition_collection(&self) -> Vec<&str> {
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
    
    #[deprecated(since = "3.0.0", note = "Use pivot_cache_definition_collection()")]
    pub(crate) fn get_pivot_cache_definition_collection(&self) -> Vec<&str> {
        self.pivot_cache_definition_collection()
    }

    /// (This method is crate only.)
    /// Has Defined Names.
    #[inline]
    pub(crate) fn has_defined_names(&self) -> bool {
        !self.defined_names().is_empty()
    }

    #[inline]
    pub(crate) fn is_deserialized(&self) -> bool {
        self.raw_data_of_worksheet.is_none()
    }

    #[inline]
    pub(crate) fn raw_data_of_worksheet(&self) -> &RawWorksheet {
        self.raw_data_of_worksheet
            .as_ref()
            .expect("Not found at raw data of worksheet.")
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use raw_data_of_worksheet()")]
    pub(crate) fn get_raw_data_of_worksheet(&self) -> &RawWorksheet {
        self.raw_data_of_worksheet()
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
    #[must_use]
    pub fn sheet_protection(&self) -> Option<&SheetProtection> {
        self.sheet_protection.as_ref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use sheet_protection()")]
    pub fn get_sheet_protection(&self) -> Option<&SheetProtection> {
        self.sheet_protection()
    }

    #[inline]
    pub fn sheet_protection_mut(&mut self) -> &mut SheetProtection {
        self.sheet_protection
            .get_or_insert(SheetProtection::default())
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use sheet_protection_mut()")]
    pub fn get_sheet_protection_mut(&mut self) -> &mut SheetProtection {
        self.sheet_protection_mut()
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
    /// 'row' - The number of rows to move by (negative numbers mean move
    /// 'left') 'column' - the number of columns to move by (negative
    /// numbers mean move 'up')
    #[inline]
    pub fn move_range(&mut self, range: &str, row: i32, column: i32) -> &mut Self {
        self.move_or_copy_range(range, row, column, true)
    }

    /// Copying a section of the sheet
    /// # Arguments
    /// 'range' - Specify like "A1:G8"
    /// 'row' - The number of rows to move by (negative numbers mean move
    /// 'left') 'column' - the number of columns to move by (negative
    /// numbers mean move 'up')
    #[inline]
    pub fn copy_range(&mut self, range: &str, row: i32, column: i32) -> &mut Self {
        self.move_or_copy_range(range, row, column, false)
    }

    // Moving or copying a section of the sheet
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    fn move_or_copy_range(
        &mut self,
        range: &str,
        row: i32,
        column: i32,
        is_move: bool,
    ) -> &mut Self {
        // Check to ensure coordinates to move are within range (eg: moving A1 cells to the left is
        // impossible)
        let (row_start, row_end, col_start, col_end) = get_start_and_end_point(range);
        if (num_traits::cast::<_, i32>(col_start).unwrap() + column) < 1
            || (num_traits::cast::<_, i32>(row_start).unwrap() + row) < 1
            || (num_traits::cast::<_, i32>(col_end).unwrap() + column) > 16384
            || (num_traits::cast::<_, i32>(row_end).unwrap() + row) > 1_048_576
        {
            panic!("Out of Range.");
        }

        // Iterate row by row, collecting cell information (do I copy)
        let cells = self.cells.cell_by_range(range);
        let mut copy_cells: Vec<Cell> = cells.into_iter().flatten().cloned().collect();

        // Delete cell information as iterating through in move mode
        if is_move {
            for (col_num, row_num) in &get_coordinate_list(range) {
                self.cells.remove(*col_num, *row_num);
                self.cells.remove(
                    *col_num + num_traits::cast::<_, u32>(column).unwrap(),
                    *row_num + num_traits::cast::<_, u32>(row).unwrap(),
                );
            }
        }

        // repaste by setting cell values
        for cell in &mut copy_cells {
            cell.coordinate_mut().offset_col_num(column);
            cell.coordinate_mut().offset_row_num(row);
            self.set_cell(cell.clone());
        }

        self
    }

    /// Remove invisible garbage data.
    /// Doing so may reduce file size.
    /// Processing may take some time.
    #[inline]
    pub fn cleanup(&mut self) {
        let (_, max_row) = self.highest_column_and_row();
        for row in (1..=max_row).rev() {
            if self.rows.get_row_dimension(row).is_some() {
                let mut indexes: Vec<(u32, u32)> = Vec::new();
                {
                    let cells: Vec<&Cell> = self.cells.collection_by_row(row);
                    for cell in cells {
                        if !cell.is_visually_empty() {
                            return;
                        }
                        indexes.push((
                            cell.coordinate().get_row_num(),
                            cell.coordinate().get_col_num(),
                        ));
                    }
                }

                self.rows.get_row_dimensions_to_hashmap_mut().remove(&row);
                for (i_row, i_col) in indexes {
                    self.cells.remove(i_col, i_row);
                }
            }
        }
    }

    #[inline]
    pub fn copy_cell_styling<T>(&mut self, source: T, target: T)
    where
        T: Into<CellCoordinates>,
    {
        let style = self.cells.style(source).clone();
        self.cell_mut(target).set_style(style);
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
        source_row_no: u32,
        target_row_no: u32,
        start_col: Option<u32>,
        end_col: Option<u32>,
    ) {
        let start_no = start_col.unwrap_or(1);
        let end_no = end_col.unwrap_or(self.highest_column());
        for col_no in start_no..=end_no {
            self.copy_cell_styling((col_no, source_row_no), (col_no, target_row_no));
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
        source_col_no: u32,
        target_col_no: u32,
        start_row: Option<u32>,
        end_row: Option<u32>,
    ) {
        let start_no = start_row.unwrap_or(1);
        let end_no = end_row.unwrap_or(self.highest_row());
        for row_no in start_no..=end_no {
            self.copy_cell_styling((source_col_no, row_no), (target_col_no, row_no));
        }
    }
}
impl AdjustmentCoordinate for Worksheet {
    fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        if offset_col_num != 0 {
            // column dimensions
            self.columns
                .adjustment_insert_value(root_col_num, offset_col_num);
        }
        if offset_row_num != 0 {
            // row dimensions
            self.row_dimensions_crate_mut()
                .adjustment_insert_value(root_row_num, offset_row_num);
        }
        if offset_col_num == 0 && offset_row_num == 0 {
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
        self.cells_crate_mut().adjustment_insert_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );

        // worksheet_drawing
        self.worksheet_drawing_mut()
            .adjustment_insert_coordinate(
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
        for merge_cell in self.merge_cells_mut() {
            merge_cell.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }

        // auto filter
        if let Some(v) = self.auto_filter_mut() {
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
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        if offset_col_num != 0 {
            // column dimensions
            self.columns
                .adjustment_remove_value(root_col_num, offset_col_num);
        }
        if offset_row_num != 0 {
            // row dimensions
            self.row_dimensions_crate_mut()
                .adjustment_remove_value(root_row_num, offset_row_num);
        }
        if offset_col_num == 0 && offset_row_num == 0 {
            return;
        }

        // defined_names
        let title = &self.title;
        self.defined_names.retain(|defined_name| {
            !defined_name.is_remove_coordinate_with_sheet(
                title,
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
        self.cells_crate_mut().adjustment_remove_coordinate(
            root_col_num,
            offset_col_num,
            root_row_num,
            offset_row_num,
        );

        // worksheet_drawing
        self.worksheet_drawing_mut()
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
        self.merge_cells_mut().retain(|x| {
            !(x.is_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num))
        });
        for merge_cell in self.merge_cells_mut() {
            merge_cell.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }

        // auto filter
        let is_remove = match self.auto_filter() {
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
        if let Some(v) = self.auto_filter_mut() {
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
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        if offset_col_num == 0 && offset_row_num == 0 {
            return;
        }

        // cell formula coordinate
        let title = self.title.clone();
        self.cells_crate_mut()
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
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        if offset_col_num == 0 && offset_row_num == 0 {
            return;
        }

        // cell formula coordinate
        let title = self.title.clone();
        self.cells_crate_mut()
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
