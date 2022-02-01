use super::drawing::spreadsheet::WorksheetDrawing;
use super::AutoFilter;
use super::Cell;
use super::CellValue;
use super::Cells;
use super::Color;
use super::Column;
use super::Columns;
use super::Comment;
use super::ConditionalSet;
use super::DefinedName;
use super::HeaderFooter;
use super::Hyperlink;
use super::NumberingFormat;
use super::OleObjects;
use super::PageMargins;
use super::PageSetup;
use super::Protection;
use super::Range;
use super::Row;
use super::SheetView;
use super::Style;
use helper::coordinate::*;
use helper::number_format::*;
use helper::range::*;
use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Worksheet {
    sheet_id: String,
    title: String,
    cell_collection: Cells,
    row_dimensions: Vec<Row>,
    column_dimensions: Columns,
    worksheet_drawing: WorksheetDrawing,
    sheet_state: String,
    page_setup: PageSetup,
    page_margins: PageMargins,
    header_footer: HeaderFooter,
    sheet_view: SheetView,
    protection: Protection,
    conditional_styles_collection: Vec<ConditionalSet>,
    breaks: Vec<String>,
    merge_cells: Vec<Range>,
    protected_cells: Vec<String>,
    auto_filter: Option<AutoFilter>,
    freeze_pane: Option<String>,
    top_left_cell: Option<String>,
    show_gridlines: bool,
    print_gridlines: bool,
    show_row_col_headers: bool,
    show_summary_below: bool,
    show_summary_right: bool,
    comments: Vec<Comment>,
    active_cell: String,
    selected_cells: String,
    right_to_left: bool,
    data_validation_collection: Vec<String>,
    tab_color: Option<Color>,
    dirty: bool,
    hash: String,
    code_name: Option<String>,
    ole_objects: OleObjects,
    defined_names: Vec<DefinedName>,
}
impl Worksheet {
    // ************************
    // Value
    // ************************

    /// Get value.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) "A1"
    /// # Return value
    /// * `String` - Value of the specified cell.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let value = worksheet.get_value("A1");
    /// ```
    pub fn get_value<S: Into<String>>(&self, coordinate: S) -> String {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0].unwrap();
        let row = split[1].unwrap();
        self.get_value_by_column_and_row(col, row)
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
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let value = worksheet.get_value_by_column_and_row(1, 1);
    /// ```
    pub fn get_value_by_column_and_row(&self, col: u32, row: u32) -> String {
        match self.get_cell_by_column_and_row(col, row) {
            Some(v) => v.get_value().into(),
            None => "".into(),
        }
    }

    /// Get formatted value.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) "A1"
    /// # Return value
    /// * `String` - Formatted value of the specified cell.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let value = worksheet.get_formatted_value("A1");
    /// ```
    pub fn get_formatted_value<S: Into<String>>(&self, coordinate: S) -> String {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0].unwrap();
        let row = split[1].unwrap();
        self.get_formatted_value_by_column_and_row(col, row)
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
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let value = worksheet.get_formatted_value_by_column_and_row(1, 1);
    /// ```
    pub fn get_formatted_value_by_column_and_row(&self, col: u32, row: u32) -> String {
        let value: String = match self.get_cell_by_column_and_row(col, row) {
            Some(v) => v.get_value().into(),
            None => "".into(),
        };

        // convert value
        let result = match self
            .get_style_by_column_and_row(col, row)
            .get_number_format()
        {
            Some(nmuber_format) => {
                to_formatted_string(value.as_str(), nmuber_format.get_format_code())
            }
            None => to_formatted_string(value.as_str(), NumberingFormat::FORMAT_GENERAL),
        };
        result
    }

    // ************************
    // Cell
    // ************************
    pub fn get_cell_collection(&self) -> &Vec<Cell> {
        self.cell_collection.get_collection()
    }

    pub fn get_cell_collection_mut(&mut self) -> &mut Vec<Cell> {
        self.cell_collection.get_collection_mut()
    }

    pub fn get_cell_collection_to_hashmap(&self) -> HashMap<String, &Cell> {
        self.cell_collection.get_collection_to_hashmap()
    }

    pub fn get_collection_by_row(&self, row_num: &u32) -> BTreeMap<u32, &Cell> {
        self.cell_collection.get_collection_by_row(row_num)
    }

    /// Get cell.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) "A1"
    /// # Return value
    /// * `Option` - Cell in the Some.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let cell = worksheet.get_cell("A1");
    /// ```
    pub fn get_cell<S: Into<String>>(&self, coordinate: S) -> Option<&Cell> {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0].unwrap();
        let row = split[1].unwrap();
        self.get_cell_by_column_and_row(col, row)
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
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let cell = worksheet.get_cell_by_column_and_row(1, 1);  // get cell from A1.
    /// ```
    pub fn get_cell_by_column_and_row(&self, col: u32, row: u32) -> Option<&Cell> {
        self.cell_collection.get(&col, &row)
    }

    /// Get cell with mutable.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) "A1"
    /// # Return value
    /// * `&mut Cell` - Cell with mutable.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(0);
    /// let cell = worksheet.get_cell_mut("A1");
    /// ```
    pub fn get_cell_mut<S: Into<String>>(&mut self, coordinate: S) -> &mut Cell {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0].unwrap();
        let row = split[1].unwrap();
        self.get_cell_by_column_and_row_mut(col, row)
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
    /// let mut worksheet = book.get_sheet_mut(0);
    /// let cell = worksheet.get_cell_by_column_and_row_mut(1, 1);  // get cell from A1.
    /// ```
    pub fn get_cell_by_column_and_row_mut(&mut self, col: u32, row: u32) -> &mut Cell {
        self.get_row_dimension_mut(&row);
        self.cell_collection.get_mut(&col, &row)
    }

    pub fn set_cell(&mut self, cell: Cell) -> &mut Self {
        self.cell_collection.set(cell);
        self
    }

    /// Get cell value.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) "A1"
    /// # Return value
    /// * `&CellValue` - CellValue.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let cell_value = worksheet.get_cell_value("A1");
    /// ```
    pub fn get_cell_value<S: Into<String>>(&self, coordinate: S) -> &CellValue {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0].unwrap();
        let row = split[1].unwrap();
        self.get_cell_value_by_column_and_row(col, row)
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
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let cell_value = worksheet.get_style_by_column_and_row(1, 1);  // get cell from A1.
    /// ```
    pub fn get_cell_value_by_column_and_row(&self, col: u32, row: u32) -> &CellValue {
        self.cell_collection.get_cell_value(&col, &row)
    }

    /// Get cell value with mutable.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) "A1"
    /// # Return value
    /// * `&mut CellValue` - CellValue with mutable.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(0);
    /// let cell_value = worksheet.get_cell_value_mut("A1");
    /// ```
    pub fn get_cell_value_mut<S: Into<String>>(&mut self, coordinate: S) -> &mut CellValue {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0].unwrap();
        let row = split[1].unwrap();
        self.get_cell_value_by_column_and_row_mut(col, row)
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
    /// let mut worksheet = book.get_sheet_mut(0);
    /// let cell_value = worksheet.get_cell_value_by_column_and_row_mut(1, 1);  // get cell_value from A1.
    /// ```
    pub fn get_cell_value_by_column_and_row_mut(&mut self, col: u32, row: u32) -> &mut CellValue {
        self.get_row_dimension_mut(&row);
        self.cell_collection
            .get_mut(&col, &row)
            .get_cell_value_mut()
    }

    pub fn get_cell_value_by_range<S: Into<String>>(&self, range: S) -> Vec<&CellValue> {
        self.cell_collection.get_cell_value_by_range(range)
    }

    /// Get style.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) "A1"
    /// # Return value
    /// * `&Style` - Style.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let style = worksheet.get_style("A1");
    /// ```
    pub fn get_style<S: Into<String>>(&self, coordinate: S) -> &Style {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0].unwrap();
        let row = split[1].unwrap();
        self.get_style_by_column_and_row(col, row)
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
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let style = worksheet.get_style_by_column_and_row(1, 1);  // get cell from A1.
    /// ```
    pub fn get_style_by_column_and_row(&self, col: u32, row: u32) -> &Style {
        self.cell_collection.get_style(&col, &row)
    }

    /// Get style with mutable.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) "A1"
    /// # Return value
    /// * `&mut Style` - Style with mutable.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut worksheet = book.get_sheet_mut(0);
    /// let style = worksheet.get_style_mut("A1");
    /// ```
    pub fn get_style_mut<S: Into<String>>(&mut self, coordinate: S) -> &mut Style {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0].unwrap();
        let row = split[1].unwrap();
        self.get_style_by_column_and_row_mut(col, row)
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
    /// let mut worksheet = book.get_sheet_mut(0);
    /// let style = worksheet.get_style_by_column_and_row_mut(1, 1);  // get style from A1.
    /// ```
    pub fn get_style_by_column_and_row_mut(&mut self, col: u32, row: u32) -> &mut Style {
        self.get_row_dimension_mut(&row);
        self.cell_collection.get_mut(&col, &row).get_style_mut()
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
    /// let mut worksheet = book.get_sheet_mut(0);
    /// let mut style = umya_spreadsheet::Style::default();
    /// style.get_borders_mut().get_bottom_mut().set_border_style(umya_spreadsheet::Border::BORDER_MEDIUM);
    /// let style = worksheet.set_style_by_column_and_row(1, 1, style);  // set style to A1.
    /// ```
    pub fn set_style_by_column_and_row(&mut self, col: u32, row: u32, style: Style) -> &mut Self {
        self.get_row_dimension_mut(&row);
        self.cell_collection.get_mut(&col, &row).set_style(style);
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
    /// let mut worksheet = book.get_sheet_mut(0);
    /// let mut style = umya_spreadsheet::Style::default();
    /// style.get_borders_mut().get_bottom_mut().set_border_style(umya_spreadsheet::Border::BORDER_MEDIUM);
    /// worksheet.set_style_by_range("A1:A3", style);
    /// ```
    pub fn set_style_by_range<S: Into<String>>(&mut self, range: S, style: Style) -> &mut Self {
        let range_upper = range.into().to_uppercase();
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
                self.get_column_dimension_mut(&col_num)
                    .set_style(style.clone());
            }
            return self;
        }

        for (col_num, row_num) in coordinate_list {
            self.set_style_by_column_and_row(col_num, row_num, style.clone());
        }
        self
    }

    // ************************
    // Comment
    // ************************
    pub fn get_comments(&self) -> &Vec<Comment> {
        &self.comments
    }

    pub fn get_comments_mut(&mut self) -> &mut Vec<Comment> {
        &mut self.comments
    }

    pub fn get_comments_to_hashmap(&self) -> HashMap<String, &Comment> {
        let mut result = HashMap::default();
        for comment in &self.comments {
            let coordinate = comment.get_coordinate().get_coordinate();
            result.insert(coordinate, comment);
        }
        result
    }

    pub fn set_comments(&mut self, value: Vec<Comment>) {
        self.comments = value;
    }

    pub fn add_comments(&mut self, value: Comment) {
        self.comments.push(value);
    }

    pub fn has_comments(&self) -> bool {
        self.comments.len() > 0
    }

    // ************************
    // Conditional
    // ************************
    pub fn get_conditional_styles_collection(&self) -> &Vec<ConditionalSet> {
        &self.conditional_styles_collection
    }

    pub fn set_conditional_styles_collection(&mut self, value: Vec<ConditionalSet>) {
        self.conditional_styles_collection = value;
    }

    pub(crate) fn add_conditional_styles_collection(&mut self, value: ConditionalSet) {
        self.conditional_styles_collection.push(value);
    }

    // ************************
    // Hyperlink
    // ************************
    pub(crate) fn get_hyperlink_collection(&self) -> HashMap<String, &Hyperlink> {
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

    // ************************
    // Merge Cells
    // ************************
    pub fn get_merge_cells(&self) -> &Vec<Range> {
        &self.merge_cells
    }

    pub fn get_merge_cells_mut(&mut self) -> &mut Vec<Range> {
        &mut self.merge_cells
    }

    pub(crate) fn add_merge_cells_crate<S: Into<String>>(&mut self, value: S) {
        let mut range = Range::default();
        range.set_range(value.into());
        self.merge_cells.push(range);
    }

    // ************************
    // Auto Filter
    // ************************
    pub fn get_auto_filter(&self) -> &Option<AutoFilter> {
        &self.auto_filter
    }

    pub fn get_auto_filter_mut(&mut self) -> &mut Option<AutoFilter> {
        &mut self.auto_filter
    }

    pub fn set_auto_filter<S: Into<String>>(&mut self, value: S) {
        let mut auto_filter = AutoFilter::default();
        auto_filter.set_range(value);
        self.auto_filter = Some(auto_filter);
    }

    pub fn remove_auto_filter(&mut self) {
        self.auto_filter = None;
    }

    // ************************
    // Column Dimensions
    // ************************
    pub fn get_column_dimensions(&self) -> &Vec<Column> {
        &self.column_dimensions.get_column_collection()
    }

    pub fn get_column_dimensions_mut(&mut self) -> &mut Vec<Column> {
        self.column_dimensions.get_column_collection_mut()
    }

    pub fn get_column_dimension(&self, col: &u32) -> Option<&Column> {
        self.get_column_dimensions_crate().get_column(col)
    }

    pub fn get_column_dimension_mut(&mut self, col: &u32) -> &mut Column {
        self.get_column_dimensions_crate_mut().get_column_mut(col)
    }

    pub(crate) fn set_column_dimension(&mut self, value: Column) -> &mut Self {
        self.get_column_dimensions_crate_mut().set_column(value);
        self
    }

    pub(crate) fn get_column_dimensions_crate(&self) -> &Columns {
        &self.column_dimensions
    }

    pub(crate) fn get_column_dimensions_crate_mut(&mut self) -> &mut Columns {
        &mut self.column_dimensions
    }

    pub(crate) fn set_column_dimensions_crate(&mut self, value: Columns) -> &mut Self {
        self.column_dimensions = value;
        self
    }

    // ************************
    // Row Dimensions
    // ************************
    pub fn get_row_dimensions(&self) -> &Vec<Row> {
        &self.row_dimensions
    }

    pub fn get_row_dimensions_mut(&mut self) -> &mut Vec<Row> {
        &mut self.row_dimensions
    }

    pub fn get_row_dimensions_to_b_tree_map(&self) -> BTreeMap<u32, &Row> {
        let mut result = BTreeMap::default();
        for row_dimension in &self.row_dimensions {
            result.insert(row_dimension.get_row_num().clone(), row_dimension);
        }
        result
    }

    pub fn get_row_dimension(&self, row: &u32) -> Option<&Row> {
        for row_dimension in &self.row_dimensions {
            if row == row_dimension.get_row_num() {
                return Some(row_dimension);
            }
        }
        None
    }

    pub fn get_row_dimension_mut(&mut self, row: &u32) -> &mut Row {
        match self.get_row_dimension(row) {
            Some(_) => {}
            None => {
                let mut obj = Row::default();
                obj.set_row_num(row.clone());
                self.set_row_dimension(obj);
            }
        }
        for row_dimenstion in self.get_row_dimensions_mut() {
            if row == row_dimenstion.get_row_num() {
                return row_dimenstion;
            }
        }
        panic!("Row not found.");
    }

    pub(crate) fn set_row_dimension(&mut self, value: Row) -> &mut Self {
        self.row_dimensions.push(value);
        self
    }

    // ************************
    // WorksheetDrawing
    // ************************
    pub fn get_worksheet_drawing(&self) -> &WorksheetDrawing {
        &self.worksheet_drawing
    }

    pub fn get_worksheet_drawing_mut(&mut self) -> &mut WorksheetDrawing {
        &mut self.worksheet_drawing
    }

    pub fn set_worksheet_drawing(&mut self, value: WorksheetDrawing) {
        self.worksheet_drawing = value;
    }

    pub fn has_drawing_object(&self) -> bool {
        self.worksheet_drawing.has_drawing_object()
    }

    // ************************
    // update Coordinate
    // ************************
    pub(crate) fn adjustment_insert_coordinate(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if sheet_name == self.title && offset_col_num != &0 {
            // update column dimensions
            for column_dimension in self.column_dimensions.get_column_collection_mut() {
                column_dimension.adjustment_insert_coordinate(root_col_num, offset_col_num);
            }
        }
        if sheet_name == self.title && offset_row_num != &0 {
            // update row dimensions
            for row_dimension in &mut self.row_dimensions {
                row_dimension.adjustment_insert_coordinate(root_row_num, offset_row_num);
            }
        }
        if sheet_name == self.title && (offset_col_num != &0 || offset_row_num != &0) {
            // update defined_names
            for defined_name in &mut self.defined_names {
                defined_name
                    .get_address_obj_mut()
                    .adjustment_insert_coordinate(
                        sheet_name,
                        root_col_num,
                        offset_col_num,
                        root_row_num,
                        offset_row_num,
                    );
            }
            // update cell
            for cell in self.get_cell_collection_mut() {
                cell.get_coordinate_mut().adjustment_insert_coordinate(
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );
            }

            // update comments
            for comment in &mut self.comments {
                comment.adjustment_insert_coordinate(
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );
            }

            // update conditional styles
            for conditional_styles in &mut self.conditional_styles_collection {
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

            // update merge cells
            for merge_cell in self.get_merge_cells_mut() {
                merge_cell.adjustment_insert_coordinate(
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );
            }

            // update auto filter
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

        if offset_col_num != &0 || offset_row_num != &0 {
            // update cell formula coordinate
            let title = self.title.clone();
            for cell in self.get_cell_collection_mut() {
                cell.cell_value.adjustment_insert_formula_coordinate(
                    &title,
                    sheet_name,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );
            }

            // update chart
            for graphic_frame in self.worksheet_drawing.get_graphic_frame_collection_mut() {
                for formula in graphic_frame
                    .get_graphic_mut()
                    .get_graphic_data_mut()
                    .get_chart_space_mut()
                    .get_chart_mut()
                    .get_formula_mut()
                {
                    formula.get_address_mut().adjustment_insert_coordinate(
                        sheet_name,
                        root_col_num,
                        offset_col_num,
                        root_row_num,
                        offset_row_num,
                    );
                }
            }
        }
    }

    pub(crate) fn adjustment_remove_coordinate(
        &mut self,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if sheet_name == self.title && offset_col_num != &0 {
            // update column dimensions
            self.column_dimensions
                .get_column_collection_mut()
                .retain(|x| {
                    !(x.get_col_num() > root_col_num
                        && x.get_col_num() < &(root_col_num + offset_col_num))
                });
            for column_dimension in self.column_dimensions.get_column_collection_mut() {
                column_dimension.adjustment_remove_coordinate(root_col_num, offset_col_num);
            }
        }
        if sheet_name == self.title && offset_row_num != &0 {
            // update row dimensions
            self.row_dimensions.retain(|x| {
                !(x.get_row_num() > root_row_num
                    && x.get_row_num() < &(root_row_num + offset_row_num))
            });
            for row_dimension in &mut self.row_dimensions {
                row_dimension.adjustment_remove_coordinate(root_row_num, offset_row_num);
            }
        }
        if sheet_name == self.title && (offset_col_num != &0 || offset_row_num != &0) {
            // update defined_names
            self.defined_names.retain(|x| {
                !(x.get_address_obj().is_remove(
                    sheet_name,
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
                        sheet_name,
                        root_col_num,
                        offset_col_num,
                        root_row_num,
                        offset_row_num,
                    );
            }

            // update cell
            self.get_cell_collection_mut().retain(|x| {
                !(x.get_coordinate().is_remove(
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                ))
            });
            for cell in self.get_cell_collection_mut() {
                cell.get_coordinate_mut().adjustment_remove_coordinate(
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );
            }

            // update comments
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

            // update conditional styles
            for conditional_styles in &mut self.conditional_styles_collection {
                conditional_styles
                    .get_sequence_of_references_mut()
                    .get_range_collection_mut()
                    .retain(|x| {
                        !(x.is_remove(root_col_num, offset_col_num, root_row_num, offset_row_num))
                    });
            }
            self.conditional_styles_collection
                .retain(|x| !(x.get_sequence_of_references().get_range_collection().len() == 0));
            for conditional_styles in &mut self.conditional_styles_collection {
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

            // update merge cells
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

            // update auto filter
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

        if offset_col_num != &0 || offset_row_num != &0 {
            // update cell formula coordinate
            let title = self.title.clone();
            for cell in self.get_cell_collection_mut() {
                cell.cell_value.adjustment_remove_formula_coordinate(
                    &title,
                    sheet_name,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                );
            }

            // update chart
            for graphic_frame in self.worksheet_drawing.get_graphic_frame_collection_mut() {
                for formula in graphic_frame
                    .get_graphic_mut()
                    .get_graphic_data_mut()
                    .get_chart_space_mut()
                    .get_chart_mut()
                    .get_formula_mut()
                {
                    formula.get_address_mut().adjustment_remove_coordinate(
                        sheet_name,
                        root_col_num,
                        offset_col_num,
                        root_row_num,
                        offset_row_num,
                    );
                }
            }
        }
    }

    pub fn get_code_name(&self) -> &Option<String> {
        &self.code_name
    }
    pub(crate) fn set_code_name<S: Into<String>>(&mut self, value: S) {
        self.code_name = Some(value.into());
    }
    pub fn get_header_footer(&self) -> &HeaderFooter {
        &self.header_footer
    }
    pub fn set_header_footer(&mut self, value: HeaderFooter) {
        self.header_footer = value;
    }

    pub fn get_active_cell(&self) -> &str {
        &self.active_cell
    }
    pub(crate) fn set_active_cell<S: Into<String>>(&mut self, value: S) {
        self.active_cell = value.into();
    }
    pub fn get_sheet_id(&self) -> &String {
        &self.sheet_id
    }
    pub(crate) fn set_sheet_id<S: Into<String>>(&mut self, value: S) {
        self.sheet_id = value.into();
    }

    pub fn has_code_name(&self) -> bool {
        match self.code_name {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_tab_color(&self) -> &Option<Color> {
        &self.tab_color
    }

    pub(crate) fn get_tab_color_mut(&mut self) -> &mut Color {
        match &self.tab_color {
            Some(_) => return self.tab_color.as_mut().unwrap(),
            None => {}
        }
        self.set_tab_color(Color::default());
        self.tab_color.as_mut().unwrap()
    }

    pub(crate) fn set_tab_color(&mut self, value: Color) {
        self.tab_color = Some(value);
    }

    pub fn calculate_worksheet_dimension(&self) -> String {
        let highest = &self.cell_collection.get_highest_row_and_column();
        if highest["row"] == &0 {
            return "A1".to_string();
        }
        let column_str = string_from_column_index(highest["column"]);
        format!("A1:{}{}", column_str, highest["row"])
    }

    pub fn get_title(&self) -> &str {
        return &self.title;
    }

    pub fn set_title<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.title = value.into();
        let title = self.get_title().to_string();
        for defined_name in self.get_defined_names_mut() {
            defined_name.get_address_obj_mut().set_sheet_name(&title);
        }
        self
    }

    pub fn get_sheet_state(&self) -> &String {
        return &self.sheet_state;
    }
    pub fn set_sheet_state(&mut self, value: String) {
        self.sheet_state = value;
    }
    pub fn get_page_setup(&self) -> &PageSetup {
        &self.page_setup
    }
    pub fn set_page_setup(&mut self, value: PageSetup) {
        self.page_setup = value;
    }
    pub fn get_page_margins(&self) -> &PageMargins {
        &self.page_margins
    }
    pub fn set_page_margins(&mut self, value: PageMargins) {
        self.page_margins = value;
    }

    pub fn get_sheet_view(&self) -> &SheetView {
        &self.sheet_view
    }

    pub fn get_sheet_view_mut(&mut self) -> &mut SheetView {
        &mut self.sheet_view
    }

    pub fn set_sheet_view(&mut self, value: SheetView) -> &mut Self {
        self.sheet_view = value;
        self
    }

    pub fn get_ole_objects(&self) -> &OleObjects {
        &self.ole_objects
    }

    pub fn get_ole_objects_mut(&mut self) -> &mut OleObjects {
        &mut self.ole_objects
    }

    pub fn set_ole_objects(&mut self, value: OleObjects) -> &mut Self {
        self.ole_objects = value;
        self
    }

    pub fn has_ole_objects(&self) -> bool {
        self.ole_objects.get_ole_object().len() > 0
    }

    pub fn has_legacy_drawing(&self) -> bool {
        self.has_comments() || self.has_ole_objects()
    }

    pub fn get_defined_names(&self) -> &Vec<DefinedName> {
        &self.defined_names
    }

    pub fn get_defined_names_mut(&mut self) -> &mut Vec<DefinedName> {
        &mut self.defined_names
    }

    pub fn set_defined_names(&mut self, value: Vec<DefinedName>) {
        self.defined_names = value;
    }

    pub fn add_defined_names(&mut self, value: DefinedName) {
        self.defined_names.push(value);
    }

    pub fn add_defined_name<S: Into<String>>(&mut self, name: S, address: S) -> Result<(), &str> {
        let mut defined_name = DefinedName::default();
        defined_name.set_name(name.into());
        defined_name.set_address(address.into());
        self.defined_names.push(defined_name);
        Ok(())
    }
}
