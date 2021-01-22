use super::cells::Cells;
use super::cell::Cell;
use super::chart::Chart;
use super::range::Range;
use super::row_dimension::RowDimension;
use super::column_dimension::ColumnDimension;
use super::drawing::Drawing;
use super::page_setup::PageSetup;
use super::page_margins::PageMargins;
use super::header_footer::HeaderFooter;
use super::sheet_view::SheetView;
use super::protection::Protection;
use super::conditional_set::ConditionalSet;
use super::style::Style;
use super::styles::Styles;
use super::auto_filter::AutoFilter;
use super::hyperlink::Hyperlink;
use super::color::Color;
use super::comment::Comment;
use super::number_format::NumberFormat;
use std::collections::BTreeMap; 
use std::collections::HashMap;
use super::super::helper::coordinate::*;
use super::super::helper::number_format::*;

#[derive(Debug)]
pub struct Worksheet {
    sheet_id: String,
    title: String,
    cell_collection: Cells,
    row_dimensions : Vec<RowDimension>,
    column_dimensions : Vec<ColumnDimension>,
    drawing_collection: Vec<Drawing>,
    chart_collection: Vec<Chart>,
    sheet_state: String,
    page_setup: PageSetup,
    page_margins: PageMargins,
    header_footer: HeaderFooter,
    sheet_view: SheetView,
    protection: Protection,
    styles : Styles,
    conditional_styles_collection: Vec<ConditionalSet>,
    breaks :Vec<String>,
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
}
impl Default for Worksheet {
    fn default() -> Self {
        Self {
            sheet_id: String::from(""),
            title: String::from(""),
            cell_collection: Cells::default(),
            row_dimensions : Vec::new(),
            column_dimensions : Vec::new(),
            drawing_collection: Vec::new(),
            chart_collection: Vec::new(),
            sheet_state: String::from(""),
            page_setup: PageSetup::default(),
            page_margins: PageMargins::default(),
            header_footer: HeaderFooter::default(),
            sheet_view: SheetView::default(),
            protection: Protection::default(),
            styles : Styles::default(),
            conditional_styles_collection: Vec::new(),
            breaks :Vec::new(),
            merge_cells: Vec::new(),
            protected_cells: Vec::new(),
            auto_filter: None,
            freeze_pane: None,
            top_left_cell: None,
            show_gridlines: false,
            print_gridlines: false,
            show_row_col_headers: false,
            show_summary_below: false,
            show_summary_right: false,
            comments: Vec::new(),
            active_cell: String::from(""),
            selected_cells: String::from(""),
            right_to_left: false,
            data_validation_collection: Vec::new(),
            tab_color: None,
            dirty: false,
            hash: String::from(""),
            code_name: None,
        }
    }
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
    pub fn get_value<S: Into<String>>(&self, coordinate:S)-> String {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0];
        let row = split[1];
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
    pub fn get_value_by_column_and_row(&self, col:usize, row:usize)-> String {
        match self.get_cell_by_column_and_row(col, row) {
            Some(v) => {v.get_value().into()},
            None => "".into()
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
    pub fn get_formatted_value<S: Into<String>>(&self, coordinate:S)-> String {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0];
        let row = split[1];
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
    pub fn get_formatted_value_by_column_and_row(&self, col:usize, row:usize)-> String {
        let value:String = match self.get_cell_by_column_and_row(col, row) {
            Some(v) => {v.get_value().into()},
            None => "".into()
        };
        
        // convert value
        let result = match self.get_style_by_column_and_row(col, row) {
            Some(style) => {
                match style.get_number_format() {
                    Some(nmuber_format) => {
                        to_formatted_string(value.as_str(), nmuber_format.get_format_code())
                    },
                    None => {
                        to_formatted_string(value.as_str(), NumberFormat::FORMAT_GENERAL)
                    }
                }
            },
            None => {
                to_formatted_string(value.as_str(), NumberFormat::FORMAT_GENERAL)
            }
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

    pub fn get_collection_by_row(&self, row_num:&usize) -> BTreeMap<usize, &Cell> {
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
    pub fn get_cell<S: Into<String>>(&self, coordinate:S)->Option<&Cell> {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0];
        let row = split[1];
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
    pub fn get_cell_by_column_and_row(&self, col:usize, row:usize)->Option<&Cell> {
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
    pub fn get_cell_mut<S: Into<String>>(&mut self, coordinate:S)->&mut Cell {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0];
        let row = split[1];
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
    pub fn get_cell_by_column_and_row_mut(&mut self, col:usize, row:usize)->&mut Cell {
        match self.get_row_dimension(&row) {
            Some(_) => {},
            None => {
                let mut row_dimension = RowDimension::default();
                row_dimension.set_row_num(&row);
                self.set_row_dimension(row_dimension);
            }
        }
        if self.cell_collection.has(&col, &row) == false {
            let mut cell = Cell::default();
            cell.get_coordinate_mut().set_col_num(&col);
            cell.get_coordinate_mut().set_row_num(&row);
            self.cell_collection.add(cell);
        }
        self.cell_collection.get_mut(&col, &row).unwrap()
    }

    // ************************    
    // Style
    // ************************
    pub fn get_style_collection(&self) -> &Vec<Style> {
        &self.styles.get_collection()
    }

    pub fn get_style_collection_mut(&mut self) -> &mut Vec<Style> {
        self.styles.get_collection_mut()
    }

    pub fn get_style_collection_to_hashmap(&self) -> HashMap<String, &Style> {
        self.styles.get_collection_to_hashmap()
    }

    pub fn get_style_collection_by_row(&self, row_num:&usize) -> BTreeMap<usize, &Style> {
        self.styles.get_collection_by_row(row_num)
    }

    /// Get style.
    /// # Arguments
    /// * `coordinate` - Specify the coordinates. ex) "A1"
    /// # Return value
    /// * `Option` - Style in the Some.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let style = worksheet.get_style("A1");
    /// ```
    pub fn get_style<S: Into<String>>(&self, coordinate:S) -> Option<&Style> {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0];
        let row = split[1];
        self.get_style_by_column_and_row(col, row)
    }

    /// Gets the style by specifying the column number and row number.
    /// # Arguments
    /// * `col` - Specify the column number. (first column number is 1)
    /// * `row` - Specify the row number. (first row number is 1)
    /// # Return value
    /// * `Option` - Style in the Some.
    /// # Examples
    /// ```
    /// let book = umya_spreadsheet::new_file();
    /// let worksheet = book.get_sheet(0).unwrap();
    /// let style = worksheet.get_style_by_column_and_row(1, 1);  // get cell from A1. 
    /// ```
    pub fn get_style_by_column_and_row(&self, col:usize, row:usize)->Option<&Style> {
        self.styles.get(&col, &row)
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
    pub fn get_style_mut<S: Into<String>>(&mut self, coordinate:S) -> &mut Style {
        let coordinate_upper = coordinate.into().to_uppercase();
        let split = index_from_coordinate(&coordinate_upper);
        let col = split[0];
        let row = split[1];
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
    pub fn get_style_by_column_and_row_mut(&mut self, col:usize, row:usize)->&mut Style {
        match self.get_row_dimension(&row) {
            Some(_) => {},
            None => {
                let mut row_dimension = RowDimension::default();
                row_dimension.set_row_num(&row);
                self.set_row_dimension(row_dimension);
            }
        }
        if self.styles.has(&col, &row) == false {
            let mut style = Style::default();
            style.get_coordinate_mut().set_col_num(&col);
            style.get_coordinate_mut().set_row_num(&row);
            self.styles.add(style);
        }
        self.styles.get_mut(&col, &row).unwrap()
    }

    pub(crate) fn add_style(&mut self, style:Style) {
        self.styles.add(style);
    }

    // ************************    
    // Comment
    // ************************
    pub fn get_comments(&self)-> &Vec<Comment> {
        &self.comments
    }

    pub fn get_comments_to_hashmap(&self)-> HashMap<String, &Comment> {
        let mut result = HashMap::default();
        for comment in &self.comments {
            let coordinate = comment.get_coordinate().get_coordinate();
            result.insert(coordinate, comment);
        }
        result
    }
   
    pub(crate) fn set_comments(&mut self, value:Vec<Comment>) {
        self.comments = value;
    }

    // ************************    
    // Conditional
    // ************************
    pub fn get_conditional_styles_collection(&self) -> &Vec<ConditionalSet> {
        &self.conditional_styles_collection

    }

    pub(crate) fn set_conditional_styles_collection(&mut self, value:Vec<ConditionalSet>) {
        self.conditional_styles_collection = value;
    }

    pub(crate) fn add_conditional_styles_collection(&mut self, value:ConditionalSet) {
        self.conditional_styles_collection.push(value);
    }

    // ************************    
    // Hyperlink
    // ************************
    pub(crate) fn get_hyperlink_collection(&self)-> HashMap<String, &Hyperlink> {
        let mut result: HashMap<String, &Hyperlink> = HashMap::new();
        for cell in self.cell_collection.get_collection() {
            match cell.get_hyperlink() {
                Some(hyperlink) => {
                    let coordition = coordinate_from_index(cell.get_coordinate().get_col_num(), cell.get_coordinate().get_row_num());
                    result.insert(coordition, hyperlink);
                },
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
    
    pub(crate) fn add_merge_cells_crate<S: Into<String>>(&mut self, value:S) {
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

    pub fn set_auto_filter<S: Into<String>>(&mut self, value:S) {
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
    pub fn get_column_dimensions(&self) -> &Vec<ColumnDimension> {
        &self.column_dimensions
    }

    pub(crate) fn set_column_dimensions(&mut self, value:ColumnDimension) {
        &self.column_dimensions.push(value);
    }

    // ************************
    // Row Dimensions
    // ************************
    pub fn get_row_dimensions(&self) -> &Vec<RowDimension> {
        &self.row_dimensions
    }

    pub fn get_row_dimensions_to_b_tree_map(&self) -> BTreeMap<usize, &RowDimension> {
        let mut result = BTreeMap::default();
        for row_dimension in &self.row_dimensions {
            result.insert(row_dimension.get_row_num().clone(), row_dimension);
        }
        result
    }

    pub fn get_row_dimension(&self, row:&usize) -> Option<&RowDimension> {
        for row_dimension in &self.row_dimensions {
            if row == row_dimension.get_row_num() {
                return Some(row_dimension);
            }
        }
        None
    }

    pub fn get_row_dimension_mut(&mut self, row:&usize) -> Option<&mut RowDimension> {
        for row_dimension in &mut self.row_dimensions {
            if row == row_dimension.get_row_num() {
                return Some(row_dimension);
            }
        }
        None
    }

    pub(crate) fn set_row_dimension(&mut self, value:RowDimension) {
        let row_num = value.get_row_num();
        match self.get_row_dimension_mut(row_num) {
            Some(v) => {
                std::mem::replace(v, value);
            },
            None => self.row_dimensions.push(value)
        }
    }

    // ************************
    // Chart
    // ************************
    pub fn get_chart_collection(&self) -> &Vec<Chart> {
        &self.chart_collection
    }

    pub fn get_chart_collection_mut(&mut self) -> &mut Vec<Chart> {
        &mut self.chart_collection
    }

    pub(crate) fn new_chart(&mut self) -> &mut Chart {
        let chart = Chart::default();
        self.add_chart(chart);
        self.chart_collection.last_mut().unwrap()
    }

    pub(crate) fn add_chart(&mut self, chart:Chart) {
        self.chart_collection.push(chart);
    }

    pub fn get_chart_count(&self) -> usize {
        self.chart_collection.len()
    }

    pub fn get_chart_by_index(&self, index:usize) -> &Chart {
        &self.chart_collection[index]
    }

    pub fn get_chart_names(&self, index:usize) -> Vec<String> {
        let mut names: Vec<String> = Vec::new();
        for v in self.get_chart_collection() {
            names.push(v.get_name().into());
        }
        names
    }

    // ************************
    // update Coordinate
    // ************************
    pub(crate) fn adjustment_insert_coordinate(&mut self, sheet_name:&str, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        if sheet_name == self.title && offset_col_num != &0 {
            // update column dimensions
            for column_dimension in &mut self.column_dimensions {
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
            // update cell
            for cell in self.get_cell_collection_mut() {
                cell.get_coordinate_mut().adjustment_insert_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
            }

            // update style
            for style in self.get_style_collection_mut() {
                style.get_coordinate_mut().adjustment_insert_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
            }

            // update comments
            for comment in &mut self.comments {
                comment.get_coordinate_mut().adjustment_insert_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
            }

            // update conditional styles
            for conditional_styles in &mut self.conditional_styles_collection {
                for range in conditional_styles.get_range_collection_mut() {
                    range.adjustment_insert_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
                }
            }

            // update merge cells
            for merge_cell in self.get_merge_cells_mut() {
                merge_cell.adjustment_insert_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
            }

            // update auto filter
            match self.get_auto_filter_mut() {
                Some(v) => {
                    v.get_range_mut().adjustment_insert_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
                },
                None => {}
            };
        }

        if offset_col_num != &0 || offset_row_num != &0 {
            // update cell formula coordinate
            let title = self.title.clone();
            for cell in self.get_cell_collection_mut() {
                cell.adjustment_insert_formula_coordinate(&title, sheet_name, root_col_num, offset_col_num, root_row_num, offset_row_num);
            }

            // update chart
            for chart in self.get_chart_collection_mut() {
                for data_serise in chart.get_plot_area_mut().get_plot_series_mut() {
                    for (_, data_serise_values) in data_serise.get_plot_label_mut() {
                        data_serise_values.get_address_mut().adjustment_insert_coordinate(sheet_name, root_col_num, offset_col_num, root_row_num, offset_row_num);
                    }
                    for (_, data_serise_values) in data_serise.get_plot_values_mut() {
                        data_serise_values.get_address_mut().adjustment_insert_coordinate(sheet_name, root_col_num, offset_col_num, root_row_num, offset_row_num);
                    }
                    for (_, data_serise_values) in data_serise.get_plot_category_mut() {
                        data_serise_values.get_address_mut().adjustment_insert_coordinate(sheet_name, root_col_num, offset_col_num, root_row_num, offset_row_num);
                    }
                }
            }
        }
    }

    pub(crate) fn adjustment_remove_coordinate(&mut self, sheet_name:&str, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        if sheet_name == self.title && offset_col_num != &0 {
            // update column dimensions
            self.column_dimensions.retain(|x| {
                !(x.get_col_num_start() > root_col_num && x.get_col_num_end() < &(root_col_num + offset_col_num))
            });
            for column_dimension in &mut self.column_dimensions {
                column_dimension.adjustment_remove_coordinate(root_col_num, offset_col_num);
            }
        }
        if sheet_name == self.title && offset_row_num != &0 {
            // update row dimensions
            self.row_dimensions.retain(|x| {
                !(x.get_row_num() > root_row_num && x.get_row_num() < &(root_row_num + offset_row_num))
            });
            for row_dimension in &mut self.row_dimensions {
                row_dimension.adjustment_remove_coordinate(root_row_num, offset_row_num);
            }
        }
        if sheet_name == self.title && (offset_col_num != &0 || offset_row_num != &0) {
            // update cell
            self.get_cell_collection_mut().retain(|x| {
                !(x.get_coordinate().is_remove(root_col_num, offset_col_num, root_row_num, offset_row_num))
            });
            for cell in self.get_cell_collection_mut() {
                cell.get_coordinate_mut().adjustment_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
            }

            // update style
            self.get_style_collection_mut().retain(|x| {
                !(x.get_coordinate().is_remove(root_col_num, offset_col_num, root_row_num, offset_row_num))
            });
            for style in self.get_style_collection_mut() {
                style.get_coordinate_mut().adjustment_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
            }

            // update comments
            self.comments.retain(|x| {
                !(x.get_coordinate().is_remove(root_col_num, offset_col_num, root_row_num, offset_row_num))
            });
            for comment in &mut self.comments {
                comment.get_coordinate_mut().adjustment_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
            }

            // update conditional styles
            for conditional_styles in &mut self.conditional_styles_collection {
                conditional_styles.get_range_collection_mut().retain(|x| {
                    !(x.is_remove(root_col_num, offset_col_num, root_row_num, offset_row_num))
                });
            }
            self.conditional_styles_collection.retain(|x| {
                !(x.get_range_collection().len() == 0)
            });
            for conditional_styles in &mut self.conditional_styles_collection {
                for range in conditional_styles.get_range_collection_mut() {
                    range.adjustment_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
                }
            }

            // update merge cells
            self.get_merge_cells_mut().retain(|x| {
                !(x.is_remove(root_col_num, offset_col_num, root_row_num, offset_row_num))
            });
            for merge_cell in self.get_merge_cells_mut() {
                merge_cell.adjustment_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
            }

            // update auto filter
            let is_remove = match self.get_auto_filter() {
                Some(v) => {
                    v.get_range().is_remove(root_col_num, offset_col_num, root_row_num, offset_row_num)
                },
                None => false
            };
            if is_remove {
                self.remove_auto_filter();
            }
            match self.get_auto_filter_mut() {
                Some(v) => {
                    v.get_range_mut().adjustment_remove_coordinate(root_col_num, offset_col_num, root_row_num, offset_row_num);
                },
                None => {}
            };
        }

        if offset_col_num != &0 || offset_row_num != &0 {
            // update cell formula coordinate
            let title = self.title.clone();
            for cell in self.get_cell_collection_mut() {
                cell.adjustment_remove_formula_coordinate(&title, sheet_name, root_col_num, offset_col_num, root_row_num, offset_row_num);
            }

            // update chart
            for chart in self.get_chart_collection_mut() {
                for data_serise in chart.get_plot_area_mut().get_plot_series_mut() {
                    for (_, data_serise_values) in data_serise.get_plot_label_mut() {
                        data_serise_values.get_address_mut().adjustment_remove_coordinate(sheet_name, root_col_num, offset_col_num, root_row_num, offset_row_num);
                    }
                    for (_, data_serise_values) in data_serise.get_plot_values_mut() {
                        data_serise_values.get_address_mut().adjustment_remove_coordinate(sheet_name, root_col_num, offset_col_num, root_row_num, offset_row_num);
                    }
                    for (_, data_serise_values) in data_serise.get_plot_category_mut() {
                        data_serise_values.get_address_mut().adjustment_remove_coordinate(sheet_name, root_col_num, offset_col_num, root_row_num, offset_row_num);
                    }
                }
            }
        }
    }

    pub(crate) fn get_coordinates(&self)-> Vec<String> {
        let mut result:Vec<String> = Vec::new();
        for cell in self.cell_collection.get_collection() {
            let coordinate = coordinate_from_index(cell.get_coordinate().get_col_num(), cell.get_coordinate().get_row_num());
            result.push(coordinate);
        }
        for style in self.styles.get_collection() {
            let coordinate = coordinate_from_index(style.get_coordinate().get_col_num(), style.get_coordinate().get_row_num());
            let mut is_match = false;
            for co in &result {
                if co == &coordinate {
                    is_match = true;
                }
            }
            if is_match == false {
                result.push(coordinate.clone());
            }
        }
        result
    }

    pub fn get_code_name(&self) -> &Option<String> {
        &self.code_name
    }
    pub(crate) fn set_code_name<S: Into<String>>(&mut self, value:S) {
        self.code_name = Some(value.into());
    }
    pub fn get_header_footer(&self) -> &HeaderFooter {
        &self.header_footer
    }
    pub(crate) fn set_header_footer(&mut self, value:HeaderFooter) {
        self.header_footer = value;
    }

    pub fn get_active_cell(&self) -> &str {
        &self.active_cell
    }
    pub(crate) fn set_active_cell<S: Into<String>>(&mut self, value:S) {
        self.active_cell = value.into();
    }
    pub fn get_sheet_id(&self) -> &String {
        &self.sheet_id
    }
    pub(crate) fn set_sheet_id<S: Into<String>>(&mut self, value:S) {
        self.sheet_id = value.into();
    }

    pub fn get_drawing_collection(&self) -> &Vec<Drawing> {
        &self.drawing_collection
    }
    pub(crate) fn get_drawing_collection_mut(&mut self) -> &mut Vec<Drawing> {
        &mut self.drawing_collection
    }
    pub(crate) fn set_drawing_collection(&mut self, value:Vec<Drawing>) {
        self.drawing_collection = value;
    }
    pub(crate) fn new_drawing(&mut self) -> &mut Drawing {
        let drawing = Drawing::default();
        self.add_drawing(drawing);
        self.drawing_collection.last_mut().unwrap()
    }
    pub(crate) fn add_drawing(&mut self, value:Drawing) {
        self.drawing_collection.push(value);
    }
    pub fn has_drawing_object(&self) -> bool {
        if self.chart_collection.len() > 0 {
            return true;
        }
        false
    }
    pub fn has_code_name(&self) -> bool {
        match self.code_name {
            Some(_) => true,
            None => false
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
    pub(crate) fn set_tab_color(&mut self, value:Color) {
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
    pub(crate) fn set_title<S: Into<String>>(&mut self, value:S) {
        self.title = value.into();
    }
    pub fn get_sheet_state(&self) -> &String {
        return &self.sheet_state;
    }
    pub(crate) fn set_sheet_state(&mut self, value:String) {
        self.sheet_state = value;
    }
    pub fn get_page_setup(&self) -> &PageSetup {
        &self.page_setup
    }
    pub(crate) fn set_page_setup(&mut self, value:PageSetup) {
        self.page_setup = value;
    }
    pub fn get_page_margins(&self) -> &PageMargins {
        &self.page_margins
    }
    pub(crate) fn set_page_margins(&mut self, value:PageMargins) {
        self.page_margins = value;
    }
}
