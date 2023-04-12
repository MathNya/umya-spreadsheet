use hashbrown::HashMap;
use helper::coordinate::*;
use helper::range::*;
use structs::drawing::spreadsheet::WorksheetDrawing;
use structs::raw::RawWorksheet;
use structs::AutoFilter;
use structs::Cell;
use structs::CellValue;
use structs::Cells;
use structs::Chart;
use structs::Color;
use structs::Column;
use structs::ColumnBreaks;
use structs::Columns;
use structs::Comment;
use structs::ConditionalFormatting;
use structs::DataValidations;
use structs::DefinedName;
use structs::HeaderFooter;
use structs::Hyperlink;
use structs::Image;
use structs::MediaObject;
use structs::MergeCells;
use structs::OleObjects;
use structs::PageMargins;
use structs::PageSetup;
use structs::PrintOptions;
use structs::Range;
use structs::Row;
use structs::RowBreaks;
use structs::Rows;
use structs::SharedStringTable;
use structs::SheetFormatProperties;
use structs::SheetViews;
use structs::Style;
use structs::Stylesheet;

use reader::xlsx::worksheet::*;

/// A Worksheet Object.
#[derive(Clone, Debug, Default)]
pub struct Worksheet {
    raw_data_of_worksheet: Option<RawWorksheet>,
    r_id: String,
    sheet_id: String,
    title: String,
    cell_collection: Cells,
    row_dimensions: Rows,
    column_dimensions: Columns,
    worksheet_drawing: Box<WorksheetDrawing>,
    sheet_state: String,
    page_setup: PageSetup,
    page_margins: PageMargins,
    header_footer: HeaderFooter,
    sheet_views: SheetViews,
    conditional_formatting_collection: Vec<ConditionalFormatting>,
    merge_cells: MergeCells,
    auto_filter: Option<AutoFilter>,
    comments: Vec<Comment>,
    active_cell: String,
    tab_color: Option<Color>,
    code_name: Option<String>,
    ole_objects: OleObjects,
    defined_names: Vec<DefinedName>,
    print_options: PrintOptions,
    column_breaks: ColumnBreaks,
    row_breaks: RowBreaks,
    data_validations: Option<DataValidations>,
    sheet_format_properties: SheetFormatProperties,
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
    pub fn get_value<T>(&self, coordinate: T) -> String
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_cell((col, row))
            .map(|v| v.get_value().into())
            .unwrap_or("".into())
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
    pub fn get_value_number<T>(&self, coordinate: T) -> Option<f64>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_cell((col, row)).and_then(|v| v.get_value_number())
    }

    /// Get value by specifying the column number and row number.
    /// # Arguments
    /// * `col` - Specify the column number. (first column number is 1)
    /// * `row` - Specify the row number. (first row number is 1)
    /// # Return value
    /// * `String` - Value of the specified cell.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(&0).unwrap();
    /// let value = worksheet.get_value_by_column_and_row(&1, &1);
    /// ```
    #[deprecated(note = "use `get_value` instead")]
    pub fn get_value_by_column_and_row(&self, col: &u32, row: &u32) -> String {
        match self.get_cell_by_column_and_row(col, row) {
            Some(v) => v.get_value().into(),
            None => "".into(),
        }
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
    pub fn get_formatted_value<T>(&self, coordinate: T) -> String
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.cell_collection
            .get_formatted_value_by_column_and_row(&col, &row)
    }

    /// Get formatted value by specifying the column number and row number.
    /// # Arguments
    /// * `col` - Specify the column number. (first column number is 1)
    /// * `row` - Specify the row number. (first row number is 1)
    /// # Return value
    /// * `String` - Formatted value of the specified cell.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(&0).unwrap();
    /// let value = worksheet.get_formatted_value_by_column_and_row(&1, &1);
    /// ```
    #[deprecated(note = "use `get_formated_value` instead")]
    pub fn get_formatted_value_by_column_and_row(&self, col: &u32, row: &u32) -> String {
        self.cell_collection
            .get_formatted_value_by_column_and_row(col, row)
    }

    // ************************
    // Cell
    // ************************
    /// Get Cell List.
    pub fn get_cell_collection(&self) -> Vec<&Cell> {
        self.cell_collection.get_collection()
    }

    /// Get Cell List in mutable.
    pub fn get_cell_collection_mut(&mut self) -> Vec<&mut Cell> {
        self.cell_collection.get_collection_mut()
    }

    pub fn get_collection_to_hashmap(&self) -> &HashMap<(u32, u32), Cell> {
        self.cell_collection.get_collection_to_hashmap()
    }

    pub fn get_collection_to_hashmap_mut(&mut self) -> &mut HashMap<(u32, u32), Cell> {
        self.cell_collection.get_collection_to_hashmap_mut()
    }

    pub(crate) fn get_cell_collection_stream(
        &self,
        shared_string_table: &SharedStringTable,
        stylesheet: &Stylesheet,
    ) -> Cells {
        if self.is_deserialized() {
            panic!("This Worksheet is Deserialized.");
        }
        read_lite(
            self.raw_data_of_worksheet.as_ref().unwrap(),
            shared_string_table,
            stylesheet,
        )
        .unwrap()
    }

    /// (This method is crate only.)
    /// Get Cells.
    pub(crate) fn get_cell_collection_crate(&self) -> &Cells {
        &self.cell_collection
    }

    /// (This method is crate only.)
    /// Get Cells in mutable.
    pub(crate) fn get_cell_collection_crate_mut(&mut self) -> &mut Cells {
        &mut self.cell_collection
    }

    /// Get cell.
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
    pub fn get_cell<T>(&self, coordinate: T) -> Option<&Cell>
    where
        T: Into<CellCoordinates>,
    {
        self.cell_collection.get(coordinate)
    }

    /// Gets the cell by specifying the column number and row number.
    /// # Arguments
    /// * `col` - Specify the column number. (first column number is 1)
    /// * `row` - Specify the row number. (first row number is 1)
    /// # Return value
    /// * `Option` - Cell in the Some.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(&0).unwrap();
    /// let cell = worksheet.get_cell_by_column_and_row(&1, &1);  // get cell from A1.
    /// ```
    #[deprecated(note = "use `get_cell` instead")]
    pub fn get_cell_by_column_and_row(&self, col: &u32, row: &u32) -> Option<&Cell> {
        self.cell_collection.get((col, row))
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
        self.cell_collection.get_mut((col, row))
    }

    /// Gets the cell with mutable by specifying the column number and row number.
    /// # Arguments
    /// * `col` - Specify the column number. (first column number is 1)
    /// * `row` - Specify the row number. (first row number is 1)
    /// # Return value
    /// *`&mut Cell` - Cell with mutable.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// let cell = worksheet.get_cell_mut((&1, &1));  // get cell from A1.
    /// ```
    #[deprecated(note = "please use `get_cell_mut` instead")]
    pub fn get_cell_by_column_and_row_mut(&mut self, col: &u32, row: &u32) -> &mut Cell {
        self.get_row_dimension_mut(row);
        self.cell_collection.get_mut((col, row))
    }

    pub fn get_collection_by_column(&self, column_num: &u32) -> Vec<&Cell> {
        self.cell_collection.get_collection_by_column(column_num)
    }

    pub fn get_collection_by_row(&self, row_num: &u32) -> Vec<&Cell> {
        self.cell_collection.get_collection_by_row(row_num)
    }

    pub fn get_collection_by_column_to_hashmap(&self, column_num: &u32) -> HashMap<u32, &Cell> {
        self.cell_collection
            .get_collection_by_column_to_hashmap(column_num)
    }

    pub fn get_collection_by_row_to_hashmap(&self, row_num: &u32) -> HashMap<u32, &Cell> {
        self.cell_collection
            .get_collection_by_row_to_hashmap(row_num)
    }

    /// Set Cell
    /// # Arguments
    /// * `cell` - Cell
    pub fn set_cell(&mut self, cell: Cell) -> &mut Self {
        self.cell_collection.set(cell);
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
    pub fn remove_cell<T>(&mut self, coordinate: T) -> bool
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.cell_collection.remove(&col, &row)
    }

    #[deprecated(note = "use `remove_cell` instead")]
    pub fn remove_cell_by_column_and_row_mut(&mut self, col: &u32, row: &u32) -> bool {
        self.cell_collection.remove(col, row)
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
    pub fn get_cell_value<T>(&self, coordinate: T) -> &CellValue
    where
        T: Into<CellCoordinates>,
    {
        self.cell_collection.get_cell_value(coordinate)
    }

    /// Gets the cell value by specifying the column number and row number.
    /// # Arguments
    /// * `col` - Specify the column number. (first column number is 1)
    /// * `row` - Specify the row number. (first row number is 1)
    /// # Return value
    /// * `&CellValue` - CellValue.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(&0).unwrap();
    /// let cell_value = worksheet.get_style_by_column_and_row(&1, &1);  // get cell from A1.
    /// ```
    #[deprecated(note = "use `get_cell_value` instead")]
    pub fn get_cell_value_by_column_and_row(&self, col: &u32, row: &u32) -> &CellValue {
        self.cell_collection.get_cell_value((col, row))
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
    pub fn get_cell_value_mut<T>(&mut self, coordinate: T) -> &mut CellValue
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_row_dimension_mut(&row);
        self.cell_collection
            .get_mut((col, row))
            .get_cell_value_mut()
    }

    /// Gets the cell value with mutable by specifying the column number and row number.
    /// # Arguments
    /// * `col` - Specify the column number. (first column number is 1)
    /// * `row` - Specify the row number. (first row number is 1)
    /// # Return value
    /// *`&mut CellValue` - CellValue with mutable.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// let cell_value = worksheet.get_cell_value_by_column_and_row_mut(&1, &1);  // get cell_value from A1.
    /// ```
    #[deprecated(note = "use `get_cell_value_mut` instead")]
    pub fn get_cell_value_by_column_and_row_mut(&mut self, col: &u32, row: &u32) -> &mut CellValue {
        self.get_row_dimension_mut(row);
        self.cell_collection
            .get_mut((col, row))
            .get_cell_value_mut()
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
    pub fn get_cell_value_by_range(&self, range: &str) -> Vec<&CellValue> {
        self.cell_collection.get_cell_value_by_range(range)
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
    pub fn get_style<T>(&self, coordinate: T) -> &Style
    where
        T: Into<CellCoordinates>,
    {
        self.cell_collection.get_style(coordinate)
    }

    /// Gets the style by specifying the column number and row number.
    /// # Arguments
    /// * `col` - Specify the column number. (first column number is 1)
    /// * `row` - Specify the row number. (first row number is 1)
    /// # Return value
    /// * `&Style` - Style.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(&0).unwrap();
    /// let style = worksheet.get_style_by_column_and_row(&1, &1);  // get cell from A1.
    /// ```
    #[deprecated(note = "use `get_style` instead")]
    pub fn get_style_by_column_and_row(&self, col: &u32, row: &u32) -> &Style {
        self.cell_collection.get_style((col, row))
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
    pub fn get_style_mut<T>(&mut self, coordinate: T) -> &mut Style
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_row_dimension_mut(&row);
        self.cell_collection.get_mut((col, row)).get_style_mut()
    }

    /// Gets the style with mutable by specifying the column number and row number.
    /// # Arguments
    /// * `col` - Specify the column number. (first column number is 1)
    /// * `row` - Specify the row number. (first row number is 1)
    /// # Return value
    /// *`&mut Style` - Style with mutable.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// let style = worksheet.get_style_by_column_and_row_mut(&1, &1);  // get style from A1.
    /// ```
    #[deprecated(note = "use `get_style_mut` instead")]
    pub fn get_style_by_column_and_row_mut(&mut self, col: &u32, row: &u32) -> &mut Style {
        self.get_row_dimension_mut(row);
        self.cell_collection.get_mut((col, row)).get_style_mut()
    }

    pub fn set_style<T>(&mut self, coordinate: T, style: Style) -> &mut Self
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_row_dimension_mut(&row);
        self.cell_collection.get_mut((&col, &row)).set_style(style);
        self
    }

    /// Set the style by specifying the column number and row number.
    /// # Arguments
    /// * `col` - Specify the column number. (first column number is 1)
    /// * `row` - Specify the row number. (first row number is 1)
    /// * `style` - Style.
    /// # Return value
    /// *`&mut Self` - self.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(&0).unwrap();
    /// let mut style = umya_spreadsheet::Style::default();
    /// style.get_borders_mut().get_bottom_mut().set_border_style(umya_spreadsheet::Border::BORDER_MEDIUM);
    /// let style = worksheet.set_style_by_column_and_row(&1, &1, style);  // set style to A1.
    /// ```
    #[deprecated(note = "use `set_style` instead")]
    pub fn set_style_by_column_and_row(&mut self, col: &u32, row: &u32, style: Style) -> &mut Self {
        self.get_row_dimension_mut(row);
        self.cell_collection.get_mut((col, row)).set_style(style);
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
        let range_upper = range.to_uppercase();
        let coordinate_list = get_coordinate_list(&range_upper);

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

    // ************************
    // Comment
    // ************************
    /// Get Comments
    pub fn get_comments(&self) -> &Vec<Comment> {
        &self.comments
    }

    /// Get Comments in mutable.
    pub fn get_comments_mut(&mut self) -> &mut Vec<Comment> {
        &mut self.comments
    }

    /// Get Comments convert to hashmap.
    pub fn get_comments_to_hashmap(&self) -> HashMap<String, &Comment> {
        let mut result = HashMap::default();
        for comment in &self.comments {
            let coordinate = comment.get_coordinate().get_coordinate();
            result.insert(coordinate, comment);
        }
        result
    }

    /// Set Comments.
    /// # Arguments
    /// * `value` - Comment List (Vec)
    pub fn set_comments(&mut self, value: Vec<Comment>) {
        self.comments = value;
    }

    /// Add Comments.
    /// # Arguments
    /// * `value` - Comment
    pub fn add_comments(&mut self, value: Comment) {
        self.comments.push(value);
    }

    /// Has Comments.
    pub fn has_comments(&self) -> bool {
        !self.comments.is_empty()
    }

    // ************************
    // Conditional
    // ************************
    /// Get ConditionalFormatting list.
    pub fn get_conditional_formatting_collection(&self) -> &Vec<ConditionalFormatting> {
        &self.conditional_formatting_collection
    }

    /// Set ConditionalFormatting.
    /// # Arguments
    /// * `value` - ConditionalSet List (Vec)
    pub fn set_conditional_formatting_collection(&mut self, value: Vec<ConditionalFormatting>) {
        self.conditional_formatting_collection = value;
    }

    /// Add ConditionalFormatting.
    /// # Arguments
    /// * `value` - ConditionalFormatting
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
        for cell in self.cell_collection.get_collection() {
            match cell.get_hyperlink() {
                Some(hyperlink) => {
                    let coordition = coordinate_from_index(
                        cell.get_coordinate().get_col_num(),
                        cell.get_coordinate().get_row_num(),
                    );
                    result.insert(coordition, hyperlink);
                }
                None => {}
            }
        }
        result
    }

    /// (This method is crate only.)
    /// Has Hyperlink
    pub(crate) fn has_hyperlink(&self) -> bool {
        self.cell_collection.has_hyperlink()
    }

    // ************************
    // Merge Cells
    // ************************
    // Get Merge Cells
    pub fn get_merge_cells(&self) -> &Vec<Range> {
        self.merge_cells.get_range_collection()
    }

    // Get Merge Cells in mutable.
    pub fn get_merge_cells_mut(&mut self) -> &mut Vec<Range> {
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
    pub fn add_merge_cells<S: Into<String>>(&mut self, range: S) -> &mut Self {
        self.merge_cells.add_range(range);
        self
    }

    /// (This method is crate only.)
    // Get Merge Cells Object
    pub(crate) fn get_merge_cells_crate(&self) -> &MergeCells {
        &self.merge_cells
    }

    /// (This method is crate only.)
    // Get Merge Cells Object in mutable.
    pub(crate) fn get_merge_cells_crate_mut(&mut self) -> &mut MergeCells {
        &mut self.merge_cells
    }

    // ************************
    // Auto Filter
    // ************************
    // Get Auto Filter (Option).
    pub fn get_auto_filter(&self) -> &Option<AutoFilter> {
        &self.auto_filter
    }

    // Get Auto Filter (Option) in mutable.
    pub fn get_auto_filter_mut(&mut self) -> &mut Option<AutoFilter> {
        &mut self.auto_filter
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
    pub fn set_auto_filter<S: Into<String>>(&mut self, range: S) {
        let mut auto_filter = AutoFilter::default();
        auto_filter.set_range(range);
        self.auto_filter = Some(auto_filter);
    }

    // Remove Auto Filter.
    pub fn remove_auto_filter(&mut self) {
        self.auto_filter = None;
    }

    // ************************
    // Column Dimensions
    // ************************
    /// Get Column Dimension List.
    pub fn get_column_dimensions(&self) -> &Vec<Column> {
        self.column_dimensions.get_column_collection()
    }

    /// Get Column Dimension List in mutable.
    pub fn get_column_dimensions_mut(&mut self) -> &mut Vec<Column> {
        self.column_dimensions.get_column_collection_mut()
    }

    /// Calculation Auto Width.
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
    pub fn get_column_dimension(&self, column: &str) -> Option<&Column> {
        let column_upper = column.to_uppercase();
        let col = column_index_from_string(column_upper);
        self.get_column_dimension_by_number(&col)
    }

    /// Get Column Dimension in mutable.
    /// # Arguments
    /// * `column` - Column Char. ex) "A"
    pub fn get_column_dimension_mut(&mut self, column: &str) -> &mut Column {
        let column_upper = column.to_uppercase();
        let col = column_index_from_string(column_upper);
        self.get_column_dimension_by_number_mut(&col)
    }

    /// Get Column Dimension.
    /// # Arguments
    /// * `col` - Column Number.
    pub fn get_column_dimension_by_number(&self, col: &u32) -> Option<&Column> {
        self.get_column_dimensions_crate().get_column(col)
    }

    /// Get Column Dimension in mutable.
    /// # Arguments
    /// * `col` - Column Number.
    pub fn get_column_dimension_by_number_mut(&mut self, col: &u32) -> &mut Column {
        self.get_column_dimensions_crate_mut().get_column_mut(col)
    }

    /// (This method is crate only.)
    /// Get Column Dimension.
    pub(crate) fn get_column_dimensions_crate(&self) -> &Columns {
        &self.column_dimensions
    }

    /// (This method is crate only.)
    /// Get Column Dimension in mutable.
    pub(crate) fn get_column_dimensions_crate_mut(&mut self) -> &mut Columns {
        &mut self.column_dimensions
    }

    /// (This method is crate only.)
    /// Set Column Dimension.
    pub(crate) fn set_column_dimensions_crate(&mut self, value: Columns) -> &mut Self {
        self.column_dimensions = value;
        self
    }

    // ************************
    // Row Dimensions
    // ************************
    pub fn has_sheet_data(&self) -> bool {
        self.row_dimensions.has_sheet_data()
    }

    /// Get Row Dimension List.
    pub fn get_row_dimensions(&self) -> Vec<&Row> {
        self.row_dimensions.get_row_dimensions()
    }

    /// Get Row Dimension List in mutable.
    pub fn get_row_dimensions_mut(&mut self) -> Vec<&mut Row> {
        self.row_dimensions.get_row_dimensions_mut()
    }

    /// Get Row Dimension convert Hashmap.
    pub fn get_row_dimensions_to_hashmap(&self) -> &HashMap<u32, Row> {
        self.row_dimensions.get_row_dimensions_to_hashmap()
    }

    pub fn get_row_dimensions_to_hashmap_mut(&mut self) -> &mut HashMap<u32, Row> {
        self.row_dimensions.get_row_dimensions_to_hashmap_mut()
    }

    /// Get Row Dimension.
    pub fn get_row_dimension(&self, row: &u32) -> Option<&Row> {
        self.row_dimensions.get_row_dimension(row)
    }

    /// Get Row Dimension in mutable.
    pub fn get_row_dimension_mut(&mut self, row: &u32) -> &mut Row {
        self.row_dimensions.get_row_dimension_mut(row)
    }

    /// (This method is crate only.)
    /// Set Row Dimension.
    pub(crate) fn set_row_dimension(&mut self, value: Row) -> &mut Self {
        self.row_dimensions.set_row_dimension(value);
        self
    }

    /// (This method is crate only.)
    /// Get Row Dimension in mutable.
    pub(crate) fn get_row_dimensions_crate_mut(&mut self) -> &mut Rows {
        &mut self.row_dimensions
    }

    /// (This method is crate only.)
    /// Get Row Dimension.
    pub(crate) fn _get_row_dimensions_crate(&self) -> &Rows {
        &self.row_dimensions
    }

    // ************************
    // WorksheetDrawing
    // ************************
    /// Get WorksheetDrawing.
    pub fn get_worksheet_drawing(&self) -> &WorksheetDrawing {
        &self.worksheet_drawing
    }

    /// Get WorksheetDrawing in mutable.
    pub fn get_worksheet_drawing_mut(&mut self) -> &mut WorksheetDrawing {
        &mut self.worksheet_drawing
    }

    /// Set WorksheetDrawing.
    /// # Arguments
    /// * `value` - WorksheetDrawing
    pub fn set_worksheet_drawing(&mut self, value: WorksheetDrawing) {
        self.worksheet_drawing = Box::new(value);
    }

    /// Has WorksheetDrawing.
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
    pub fn insert_new_row(&mut self, row_index: &u32, num_rows: &u32) {
        self.adjustment_insert_coordinate(&0, &0, row_index, num_rows);
    }

    /// Adjust for references to other sheets.
    pub fn insert_new_row_from_other_sheet(
        &mut self,
        sheet_name: &str,
        row_index: &u32,
        num_rows: &u32,
    ) {
        self.adjustment_insert_coordinate_from_other_sheet(sheet_name, &0, &0, row_index, num_rows);
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
    pub fn insert_new_column(&mut self, column: &str, num_columns: &u32) {
        let column_upper = column.to_uppercase();
        let column_index = column_index_from_string(column_upper);
        self.insert_new_column_by_index(&column_index, num_columns);
    }

    /// Adjust for references to other sheets.
    pub fn insert_new_column_from_other_sheet(
        &mut self,
        sheet_name: &str,
        column: &str,
        num_columns: &u32,
    ) {
        let column_upper = column.to_uppercase();
        let column_index = column_index_from_string(column_upper);
        self.insert_new_column_by_index_from_other_sheet(sheet_name, &column_index, num_columns);
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
    pub fn insert_new_column_by_index(&mut self, column_index: &u32, num_columns: &u32) {
        self.adjustment_insert_coordinate(column_index, num_columns, &0, &0);
    }

    /// Adjust for references to other sheets.
    pub fn insert_new_column_by_index_from_other_sheet(
        &mut self,
        sheet_name: &str,
        column_index: &u32,
        num_columns: &u32,
    ) {
        self.adjustment_insert_coordinate_from_other_sheet(
            sheet_name,
            column_index,
            num_columns,
            &0,
            &0,
        );
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
    pub fn remove_row(&mut self, row_index: &u32, num_rows: &u32) {
        self.adjustment_remove_coordinate(&0, &0, row_index, num_rows);
    }

    /// Adjust for references to other sheets.
    pub fn remove_row_from_other_sheet(
        &mut self,
        sheet_name: &str,
        row_index: &u32,
        num_rows: &u32,
    ) {
        self.adjustment_remove_coordinate_from_other_sheet(sheet_name, &0, &0, row_index, num_rows);
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
    pub fn remove_column(&mut self, column: &str, num_columns: &u32) {
        let column_upper = column.to_uppercase();
        let column_index = column_index_from_string(column_upper);
        self.remove_column_by_index(&column_index, num_columns);
    }

    /// Adjust for references to other sheets.
    pub fn remove_column_from_other_sheet(
        &mut self,
        sheet_name: &str,
        column: &str,
        num_columns: &u32,
    ) {
        let column_upper = column.to_uppercase();
        let column_index = column_index_from_string(column_upper);
        self.remove_column_by_index_from_other_sheet(sheet_name, &column_index, num_columns);
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
    pub fn remove_column_by_index(&mut self, column_index: &u32, num_columns: &u32) {
        self.adjustment_remove_coordinate(column_index, num_columns, &0, &0);
    }

    /// Adjust for references to other sheets.
    pub fn remove_column_by_index_from_other_sheet(
        &mut self,
        sheet_name: &str,
        column_index: &u32,
        num_columns: &u32,
    ) {
        self.adjustment_remove_coordinate_from_other_sheet(
            sheet_name,
            column_index,
            num_columns,
            &0,
            &0,
        );
    }

    /// (This method is crate only.)
    /// Adjustment Insert Coordinate
    pub(crate) fn adjustment_insert_coordinate(
        &mut self,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if offset_col_num != &0 {
            // column dimensions
            self.column_dimensions
                .adjustment_insert_coordinate(root_col_num, offset_col_num);
        }
        if offset_row_num != &0 {
            // row dimensions
            self.get_row_dimensions_crate_mut()
                .adjustment_insert_coordinate(root_row_num, offset_row_num);
        }
        if offset_col_num != &0 || offset_row_num != &0 {
            // defined_names
            let title = self.title.clone();
            for defined_name in &mut self.defined_names {
                defined_name
                    .get_address_obj_mut()
                    .adjustment_insert_coordinate(
                        &title,
                        root_col_num,
                        offset_col_num,
                        root_row_num,
                        offset_row_num,
                    );
            }

            // cell
            self.get_cell_collection_crate_mut()
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
                for range in conditional_styles
                    .get_sequence_of_references_mut()
                    .get_range_collection_mut()
                {
                    range.adjustment_insert_coordinate(
                        root_col_num,
                        offset_col_num,
                        root_row_num,
                        offset_row_num,
                    );
                }
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
            match self.get_auto_filter_mut() {
                Some(v) => {
                    v.get_range_mut().adjustment_insert_coordinate(
                        root_col_num,
                        offset_col_num,
                        root_row_num,
                        offset_row_num,
                    );
                }
                None => {}
            };
        }
    }

    pub(crate) fn adjustment_insert_coordinate_from_other_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if offset_col_num != &0 || offset_row_num != &0 {
            // cell formula coordinate
            let title = self.title.clone();
            self.get_cell_collection_crate_mut()
                .adjustment_insert_formula_coordinate(
                    &title,
                    sheet_name,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );

            // chart
            self.worksheet_drawing.adjustment_insert_coordinate(
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
        if offset_col_num != &0 {
            // column dimensions
            self.column_dimensions
                .adjustment_remove_coordinate(root_col_num, offset_col_num);
        }
        if offset_row_num != &0 {
            // row dimensions
            self.get_row_dimensions_crate_mut()
                .adjustment_remove_coordinate(root_row_num, offset_row_num);
        }
        if offset_col_num != &0 || offset_row_num != &0 {
            // defined_names
            let title = self.title.clone();
            self.defined_names.retain(|x| {
                !(x.get_address_obj().is_remove(
                    &title,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                ))
            });
            for defined_name in &mut self.defined_names {
                defined_name
                    .get_address_obj_mut()
                    .adjustment_remove_coordinate(
                        &title,
                        root_col_num,
                        offset_col_num,
                        root_row_num,
                        offset_row_num,
                    );
            }

            // cell
            self.get_cell_collection_crate_mut()
                .adjustment_remove_coordinate(
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );

            // comments
            self.comments.retain(|x| {
                !(x.get_coordinate().is_remove(
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                ))
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
            for conditional_styles in &mut self.conditional_formatting_collection {
                conditional_styles
                    .get_sequence_of_references_mut()
                    .get_range_collection_mut()
                    .retain(|x| {
                        !(x.is_remove(root_col_num, offset_col_num, root_row_num, offset_row_num))
                    });
            }
            self.conditional_formatting_collection.retain(|x| {
                !x.get_sequence_of_references()
                    .get_range_collection()
                    .is_empty()
            });
            for conditional_styles in &mut self.conditional_formatting_collection {
                for range in conditional_styles
                    .get_sequence_of_references_mut()
                    .get_range_collection_mut()
                {
                    range.adjustment_remove_coordinate(
                        root_col_num,
                        offset_col_num,
                        root_row_num,
                        offset_row_num,
                    );
                }
            }

            // merge cells
            self.get_merge_cells_mut().retain(|x| {
                !(x.is_remove(root_col_num, offset_col_num, root_row_num, offset_row_num))
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
                Some(v) => v.get_range().is_remove(
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
            match self.get_auto_filter_mut() {
                Some(v) => {
                    v.get_range_mut().adjustment_remove_coordinate(
                        root_col_num,
                        offset_col_num,
                        root_row_num,
                        offset_row_num,
                    );
                }
                None => {}
            };
        }
    }

    /// (This method is crate only.)
    /// Adjustment Remove Coordinate
    pub(crate) fn adjustment_remove_coordinate_from_other_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if offset_col_num != &0 || offset_row_num != &0 {
            // cell formula coordinate
            let title = self.title.clone();
            self.get_cell_collection_crate_mut()
                .adjustment_remove_formula_coordinate(
                    &title,
                    sheet_name,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );

            // chart
            self.worksheet_drawing.adjustment_remove_coordinate(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    /// Get Code Name.
    pub fn get_code_name(&self) -> &Option<String> {
        &self.code_name
    }

    /// Set Code Name.
    /// # Arguments
    /// * `value` - Code Name
    pub fn set_code_name<S: Into<String>>(&mut self, value: S) {
        self.code_name = Some(value.into());
    }

    /// Get Header Footer.
    pub fn get_header_footer(&self) -> &HeaderFooter {
        &self.header_footer
    }

    /// Get Header Footer in mutable.
    pub fn get_header_footer_mut(&mut self) -> &mut HeaderFooter {
        &mut self.header_footer
    }

    /// Set Header Footer.
    /// # Arguments
    /// * `value` - Header Footer
    pub fn set_header_footer(&mut self, value: HeaderFooter) -> &mut Self {
        self.header_footer = value;
        self
    }

    /// Get Active Cell.
    pub fn get_active_cell(&self) -> &str {
        &self.active_cell
    }

    /// Set Active Cell.
    /// # Arguments
    /// * `cell` - Cell ex) "A1"
    pub fn set_active_cell<S: Into<String>>(&mut self, cell: S) {
        self.active_cell = cell.into();
    }

    /// Get R Id.
    pub(crate) fn get_r_id(&self) -> &String {
        &self.r_id
    }

    /// (This method is crate only.)
    /// Set r Id.
    pub(crate) fn set_r_id<S: Into<String>>(&mut self, value: S) {
        self.r_id = value.into();
    }

    /// Get Sheet Id.
    pub fn get_sheet_id(&self) -> &String {
        &self.sheet_id
    }

    /// (This method is crate only.)
    /// Set Sheet Id.
    pub(crate) fn set_sheet_id<S: Into<String>>(&mut self, value: S) {
        self.sheet_id = value.into();
    }

    /// Has Code Name.
    pub fn has_code_name(&self) -> bool {
        self.code_name.is_some()
    }

    /// Get Tab Color.
    pub fn get_tab_color(&self) -> &Option<Color> {
        &self.tab_color
    }

    /// Get Tab Color in mutable.
    pub fn get_tab_color_mut(&mut self) -> &mut Color {
        match &self.tab_color {
            Some(_) => return self.tab_color.as_mut().unwrap(),
            None => {}
        }
        self.set_tab_color(Color::default());
        self.tab_color.as_mut().unwrap()
    }

    /// Set Tab Color.
    /// # Arguments
    /// * `value` - Color
    pub fn set_tab_color(&mut self, value: Color) -> &mut Self {
        self.tab_color = Some(value);
        self
    }

    /// Remove Tab Color.
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
    pub fn get_highest_column_and_row(&self) -> (u32, u32) {
        self.cell_collection.get_highest_column_and_row()
    }

    // Get Highest Column Index
    pub fn get_highest_column(&self) -> u32 {
        let (column, _row) = self.cell_collection.get_highest_column_and_row();
        column
    }

    // Get Highest Row Index
    pub fn get_highest_row(&self) -> u32 {
        let (_column, row) = self.cell_collection.get_highest_column_and_row();
        row
    }

    /// Get Title.
    #[deprecated(since = "1.0.0", note = "please use `get_name` instead")]
    pub fn get_title(&self) -> &str {
        self.get_name()
    }

    /// Get SheetName.
    pub fn get_name(&self) -> &str {
        &self.title
    }

    /// Set Title.
    /// # Arguments
    /// * `sheet_title` - Sheet Title. [Caution] no duplicate other worksheet.
    #[deprecated(since = "1.0.0", note = "please use `set_name` instead")]
    pub fn set_title<S: Into<String>>(&mut self, sheet_title: S) -> &mut Self {
        self.set_name(sheet_title)
    }

    /// Set SheetName.
    /// # Arguments
    /// * `sheet_name` - Sheet Name. [Caution] no duplicate other worksheet.
    pub fn set_name<S: Into<String>>(&mut self, sheet_name: S) -> &mut Self {
        self.title = sheet_name.into();
        let title = self.get_name().to_string();
        for defined_name in self.get_defined_names_mut() {
            defined_name.get_address_obj_mut().set_sheet_name(&title);
        }
        self
    }

    // Get Sheet State
    pub fn get_sheet_state(&self) -> &str {
        &self.sheet_state
    }

    /// Set Sheet State.
    /// # Arguments
    /// * `value` - Sheet State.
    pub fn set_sheet_state(&mut self, value: String) -> &mut Self {
        self.sheet_state = value;
        self
    }

    // Get Page Setup.
    pub fn get_page_setup(&self) -> &PageSetup {
        &self.page_setup
    }

    // Get Page Setup in mutable.
    pub fn get_page_setup_mut(&mut self) -> &mut PageSetup {
        &mut self.page_setup
    }

    /// Set Page Setup.
    /// # Arguments
    /// * `value` - PageSetup.
    pub fn set_page_setup(&mut self, value: PageSetup) -> &mut Self {
        self.page_setup = value;
        self
    }

    // Get Page Margins.
    pub fn get_page_margins(&self) -> &PageMargins {
        &self.page_margins
    }

    // Get Page Margins in mutable.
    pub fn get_page_margins_mut(&mut self) -> &mut PageMargins {
        &mut self.page_margins
    }

    /// Set Page Margins.
    /// # Arguments
    /// * `value` - PageMargins.
    pub fn set_page_margins(&mut self, value: PageMargins) -> &mut Self {
        self.page_margins = value;
        self
    }

    // Get SheetViews.
    pub fn get_sheets_views(&self) -> &SheetViews {
        &self.sheet_views
    }

    // Get SheetViews in mutable.
    pub fn get_sheet_views_mut(&mut self) -> &mut SheetViews {
        &mut self.sheet_views
    }

    /// Set SheetViews.
    /// # Arguments
    /// * `value` - SheetViews.
    pub fn set_sheets_views(&mut self, value: SheetViews) -> &mut Self {
        self.sheet_views = value;
        self
    }

    // Get Ole Objects.
    pub fn get_ole_objects(&self) -> &OleObjects {
        &self.ole_objects
    }

    // Get Ole Objects in mutable.
    pub fn get_ole_objects_mut(&mut self) -> &mut OleObjects {
        &mut self.ole_objects
    }

    /// Set Ole Objects.
    /// # Arguments
    /// * `value` - OleObjects.
    pub fn set_ole_objects(&mut self, value: OleObjects) -> &mut Self {
        self.ole_objects = value;
        self
    }

    /// Get Defined Name (Vec).
    pub fn get_defined_names(&self) -> &Vec<DefinedName> {
        &self.defined_names
    }

    /// Get Defined Name (Vec) in mutable.
    pub fn get_defined_names_mut(&mut self) -> &mut Vec<DefinedName> {
        &mut self.defined_names
    }

    /// Set Defined Name (Vec).
    /// # Arguments
    /// * `value` - Vec<DefinedName>.
    pub fn set_defined_names(&mut self, value: Vec<DefinedName>) {
        self.defined_names = value;
    }

    /// Add Defined Name.
    /// # Arguments
    /// * `value` - DefinedName.
    pub fn add_defined_names(&mut self, value: DefinedName) {
        self.defined_names.push(value);
    }

    /// Add Defined Name.
    /// # Arguments
    /// * `name` - Name. ex) "DefinedName01"
    /// * `address` - Address. ex) "A1:A2"
    pub fn add_defined_name<S: Into<String>>(&mut self, name: S, address: S) -> Result<(), &str> {
        let mut defined_name = DefinedName::default();
        defined_name.set_name(name.into());
        defined_name.set_address(address.into());
        self.add_defined_names(defined_name);
        Ok(())
    }

    /// Get Print Options.
    pub fn get_print_options(&self) -> &PrintOptions {
        &self.print_options
    }

    /// Get Print Options in mutable.
    pub fn get_print_options_mut(&mut self) -> &mut PrintOptions {
        &mut self.print_options
    }

    /// Set Print Options.
    /// # Arguments
    /// * `value` - PrintOptions.
    pub fn set_print_options(&mut self, value: PrintOptions) -> &mut Self {
        self.print_options = value;
        self
    }

    /// Get Column Breaks.
    pub fn get_column_breaks(&self) -> &ColumnBreaks {
        &self.column_breaks
    }

    /// Get Column Breaks in mutable.
    pub fn get_column_breaks_mut(&mut self) -> &mut ColumnBreaks {
        &mut self.column_breaks
    }

    /// Set Column Breaks.
    /// # Arguments
    /// * `value` - ColumnBreaks.
    pub fn set_column_breaks(&mut self, value: ColumnBreaks) -> &mut Self {
        self.column_breaks = value;
        self
    }

    /// Get Row Breaks.
    pub fn get_row_breaks(&self) -> &RowBreaks {
        &self.row_breaks
    }

    /// Get Row Breaks in mutable.
    pub fn get_row_breaks_mut(&mut self) -> &mut RowBreaks {
        &mut self.row_breaks
    }

    /// Set Row Breaks.
    /// # Arguments
    /// * `value` - RowBreaks.
    pub fn set_row_breaks(&mut self, value: RowBreaks) -> &mut Self {
        self.row_breaks = value;
        self
    }

    pub fn get_data_validations(&self) -> &Option<DataValidations> {
        &self.data_validations
    }

    pub fn get_data_validations_mut(&mut self) -> &mut Option<DataValidations> {
        &mut self.data_validations
    }

    pub fn set_data_validations(&mut self, value: DataValidations) -> &mut Self {
        self.data_validations = Some(value);
        self
    }

    pub fn remove_data_validations(&mut self) -> &mut Self {
        self.data_validations = None;
        self
    }

    pub fn get_sheet_format_properties(&self) -> &SheetFormatProperties {
        &self.sheet_format_properties
    }

    pub fn get_sheet_format_properties_mut(&mut self) -> &mut SheetFormatProperties {
        &mut self.sheet_format_properties
    }

    pub fn set_sheet_format_properties(&mut self, value: SheetFormatProperties) -> &mut Self {
        self.sheet_format_properties = value;
        self
    }

    /// Outputs all images contained in the worksheet.
    /// # Return value
    /// * `&Vec<Image>` - Image Object List.
    pub fn get_image_collection(&self) -> &Vec<Image> {
        self.get_worksheet_drawing().get_image_collection()
    }

    /// Outputs all images contained in the worksheet.
    /// # Return value
    /// * `&mut Vec<Image>` - Image Object List.
    pub fn get_image_collection_mut(&mut self) -> &mut Vec<Image> {
        self.get_worksheet_drawing_mut().get_image_collection_mut()
    }

    pub fn add_image(&mut self, value: Image) -> &mut Self {
        self.get_worksheet_drawing_mut().add_image(value);
        self
    }

    pub fn get_image<T>(&self, coordinate: T) -> Option<&Image>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing().get_image(&col, &row)
    }

    #[deprecated(note = "use `get_image` instead")]
    pub fn get_image_by_column_and_row(&self, col: &u32, row: &u32) -> Option<&Image> {
        self.get_worksheet_drawing().get_image(col, row)
    }

    pub fn get_image_mut<T>(&mut self, coordinate: T) -> Option<&mut Image>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing_mut().get_image_mut(&col, &row)
    }

    pub fn get_image_by_column_and_row_mut(&mut self, col: &u32, row: &u32) -> Option<&mut Image> {
        self.get_worksheet_drawing_mut().get_image_mut(col, row)
    }

    pub fn get_images<T>(&self, coordinate: T) -> Vec<&Image>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing().get_images(&col, &row)
    }

    #[deprecated(note = "use `get_images` instead")]
    pub fn get_images_by_column_and_row(&self, col: &u32, row: &u32) -> Vec<&Image> {
        self.get_worksheet_drawing().get_images(col, row)
    }

    pub fn get_images_mut<T>(&mut self, coordinate: T) -> Vec<&mut Image>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing_mut().get_images_mut(&col, &row)
    }

    #[deprecated(note = "use `get_images_mut` instead")]
    pub fn get_images_by_column_and_row_mut(&mut self, col: &u32, row: &u32) -> Vec<&mut Image> {
        self.get_worksheet_drawing_mut().get_images_mut(col, row)
    }

    /// Outputs all Charts contained in the worksheet.
    /// # Return value
    /// * `&Vec<Chart>` - Chart Object List.
    pub fn get_chart_collection(&self) -> &Vec<Chart> {
        self.get_worksheet_drawing().get_chart_collection()
    }

    /// Outputs all Charts contained in the worksheet.
    /// # Return value
    /// * `&mut Vec<Chart>` - Chart Object List.
    pub fn get_chart_collection_mut(&mut self) -> &mut Vec<Chart> {
        self.get_worksheet_drawing_mut().get_chart_collection_mut()
    }

    pub fn add_chart(&mut self, value: Chart) -> &mut Self {
        self.get_worksheet_drawing_mut().add_chart_collection(value);
        self
    }

    pub fn get_chart<T>(&self, coordinate: T) -> Option<&Chart>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing().get_chart(&col, &row)
    }

    #[deprecated(note = "use `get_chart` instead")]
    pub fn get_chart_by_column_and_row(&self, col: &u32, row: &u32) -> Option<&Chart> {
        self.get_worksheet_drawing().get_chart(col, row)
    }

    pub fn get_chart_mut<T>(&mut self, coordinate: T) -> Option<&mut Chart>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing_mut().get_chart_mut(&col, &row)
    }

    #[deprecated(note = "use `get_chart_mut` instead")]
    pub fn get_chart_by_column_and_row_mut(&mut self, col: &u32, row: &u32) -> Option<&mut Chart> {
        self.get_worksheet_drawing_mut().get_chart_mut(col, row)
    }

    pub fn get_charts<T>(&self, coordinate: T) -> Vec<&Chart>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing().get_charts(&col, &row)
    }

    #[deprecated(note = "use `get_charts` instead")]
    pub fn get_charts_by_column_and_row(&self, col: &u32, row: &u32) -> Vec<&Chart> {
        self.get_worksheet_drawing().get_charts(col, row)
    }

    pub fn get_charts_mut<T>(&mut self, coordinate: T) -> Vec<&mut Chart>
    where
        T: Into<CellCoordinates>,
    {
        let CellCoordinates { col, row } = coordinate.into();
        self.get_worksheet_drawing_mut().get_charts_mut(&col, &row)
    }

    #[deprecated(note = "use `get_charts_mut` instead")]
    pub fn get_charts_by_column_and_row_mut(&mut self, col: &u32, row: &u32) -> Vec<&mut Chart> {
        self.get_worksheet_drawing_mut().get_charts_mut(col, row)
    }

    /// Outputs all media contained in the worksheet.
    /// # Return value
    /// * `Vec<&MediaObject>` - Media Object List.
    pub(crate) fn get_media_object_collection(&self) -> Vec<&MediaObject> {
        let mut result: Vec<&MediaObject> = Vec::new();
        for image in self.get_worksheet_drawing().get_image_collection() {
            let media_object = image.get_media_object();
            let mut is_new = true;
            for v in &result {
                if v.get_image_name() == media_object.get_image_name() {
                    is_new = false;
                }
            }
            if is_new {
                result.push(media_object);
            }
        }
        for ole_objects in self.get_ole_objects().get_ole_object() {
            let media_object = ole_objects.get_embedded_object_properties().get_image();
            let mut is_new = true;
            for v in &result {
                if v.get_image_name() == media_object.get_image_name() {
                    is_new = false;
                }
            }
            if is_new {
                result.push(media_object);
            }
        }
        result
    }

    pub(crate) fn get_pivot_cache_definition_collection(&self) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        match &self.raw_data_of_worksheet {
            Some(raw_data) => {
                for relationships in raw_data.get_relationships_list() {
                    for row in relationships.get_relationship_list() {
                        if row.get_type() == "http://schemas.openxmlformats.org/officeDocument/2006/relationships/pivotCacheDefinition" {
                            result.push(row.get_raw_file().get_file_target());
                        }
                    }
                }
            }
            None => {}
        }
        result
    }

    /// (This method is crate only.)
    /// Has Defined Names.
    pub(crate) fn has_defined_names(&self) -> bool {
        if !self.get_defined_names().is_empty() {
            return true;
        }
        false
    }

    pub(crate) fn is_deserialized(&self) -> bool {
        self.raw_data_of_worksheet.is_none()
    }

    pub(crate) fn get_raw_data_of_worksheet(&self) -> &RawWorksheet {
        match &self.raw_data_of_worksheet {
            Some(v) => {
                return v;
            }
            None => {}
        }
        panic!("Not found at raw data of worksheet.");
    }

    pub(crate) fn set_raw_data_of_worksheet(&mut self, value: RawWorksheet) -> &mut Self {
        self.raw_data_of_worksheet = Some(value);
        self
    }

    pub(crate) fn remove_raw_data_of_worksheet(&mut self) -> &mut Self {
        self.raw_data_of_worksheet = None;
        self
    }

    /// (This method is crate only.)
    /// Has Ole Objects.
    pub(crate) fn has_ole_objects(&self) -> bool {
        !self.ole_objects.get_ole_object().is_empty()
    }

    /// (This method is crate only.)
    /// Has Legacy Drawing.
    pub(crate) fn has_legacy_drawing(&self) -> bool {
        self.has_comments() || self.has_ole_objects()
    }
}
