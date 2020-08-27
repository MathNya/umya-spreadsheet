use super::spreadsheet::Spreadsheet;
use super::cells::Cells;
use super::cell::Cell;
use super::chart::Chart;
use super::row_dimension::RowDimension;
use super::column_dimension::ColumnDimension;
use super::drawing::Drawing;
use super::page_setup::PageSetup;
use super::page_margins::PageMargins;
use super::header_footer::HeaderFooter;
use super::sheet_view::SheetView;
use super::protection::Protection;
use super::conditional::Conditional;
use super::style::Style;
use super::auto_filter::AutoFilter;
use super::hyperlink::Hyperlink;
use super::color::Color;
use std::collections::BTreeMap; 
use std::collections::HashMap; 
use super::super::helper::coordinate::*;

#[derive(Debug)]
pub struct Worksheet {
    sheet_id: String,
    title: String,
    cell_collection: Cells,
    row_dimensions : BTreeMap<usize, RowDimension>,
    default_row_dimension: RowDimension,
    column_dimensions : Vec<ColumnDimension>,
    default_column_dimension: ColumnDimension,
    drawing_collection: Vec<Drawing>,
    chart_collection: Vec<Chart>,
    sheet_state: String,
    page_setup: PageSetup,
    page_margins: PageMargins,
    header_footer: HeaderFooter,
    sheet_view: SheetView,
    protection: Protection,
    styles : HashMap<String, Style>,
    conditional_styles_collection: HashMap<String, Vec<Conditional>>,
    cell_collection_is_sorted : bool,
    breaks :Vec<String>,
    merge_cells: Vec<String>,
    protected_cells: Vec<String>,
    auto_filter: AutoFilter,
    freeze_pane: Option<String>,
    top_left_cell: Option<String>,
    show_gridlines: bool,
    print_gridlines: bool,
    show_row_col_headers: bool,
    show_summary_below: bool,
    show_summary_right: bool,
    comments: Vec<String>,
    active_cell: String,
    selected_cells: String,
    cached_highest_column: String,
    cached_highest_row: String,
    right_to_left: bool,
    hyperlink_collection: Vec<Hyperlink>,
    data_validation_collection: Vec<String>,
    tab_color: Option<Color>,
    dirty: bool,
    hash: String,
    code_name: String,
}
impl Default for Worksheet {
    fn default() -> Self {
        Self {
            sheet_id: String::from(""),
            title: String::from(""),
            cell_collection: Cells::default(),
            row_dimensions : BTreeMap::new(),
            default_row_dimension: RowDimension::default(),
            column_dimensions : Vec::new(),
            default_column_dimension: ColumnDimension::default(),
            drawing_collection: Vec::new(),
            chart_collection: Vec::new(),
            sheet_state: String::from(""),
            page_setup: PageSetup::default(),
            page_margins: PageMargins::default(),
            header_footer: HeaderFooter::default(),
            sheet_view: SheetView::default(),
            protection: Protection::default(),
            styles : HashMap::new(),
            conditional_styles_collection: HashMap::new(),
            cell_collection_is_sorted : false,
            breaks :Vec::new(),
            merge_cells: Vec::new(),
            protected_cells: Vec::new(),
            auto_filter: AutoFilter::default(),
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
            cached_highest_column: String::from(""),
            cached_highest_row: String::from(""),
            right_to_left: false,
            hyperlink_collection: Vec::new(),
            data_validation_collection: Vec::new(),
            tab_color: None,
            dirty: false,
            hash: String::from(""),
            code_name: String::from(""),
        }
    }
}
impl Worksheet {
    pub(crate) fn get_coordinates(&self)-> Vec<String> {
        let mut result:Vec<String> = Vec::new();
        for coordinate in self.cell_collection.get_coordinates() {
            result.push(coordinate);
        }
        for (coordinate, _) in &self.styles {
            let mut is_match = false;
            for co in &result {
                if co == coordinate {
                    is_match = true;
                }
            }
            if is_match == false {
                result.push(coordinate.clone());
            }
        }
        result
    }
    pub fn get_style_collection(&self) -> &HashMap<String, Style> {
        &self.styles
    }
    pub fn get_style<S: Into<String>>(&self, coordinate:S) -> Result<&Style, &'static str> {
        let coordinate_upper = coordinate.into().to_uppercase();
        match self.styles.get(&coordinate_upper) {
            Some(v) => return Ok(v),
            None => Err("Not found.")
        }
    }
    pub fn get_style_mut<S: Into<String>>(&mut self, coordinate:S) -> &mut Style {
        let coordinate_upper = coordinate.into().to_uppercase();
        match self.styles.get(&coordinate_upper) {
            Some(_) => return self.styles.get_mut(&coordinate_upper).unwrap(),
            None => {}
        }
        self.add_style(&coordinate_upper, Style::default());
        self.styles.get_mut(&coordinate_upper).unwrap()
    }
    pub(crate) fn add_style<S: Into<String>>(&mut self, coordinate:S, style:Style) {
        let coordinate_upper = coordinate.into().to_uppercase();
        self.styles.insert(coordinate_upper, style);
    }
    pub fn get_conditional_styles_collection(&self) -> &HashMap<String, Vec<Conditional>> {
        &self.conditional_styles_collection
    }
    pub(crate) fn set_conditional_styles_collection(&mut self, value:HashMap<String, Vec<Conditional>>) {
        self.conditional_styles_collection = value;
    }
    pub(crate) fn add_conditional_styles_collection<S: Into<String>>(&mut self, coordinate:S, value:Vec<Conditional>) {
        self.conditional_styles_collection.insert(coordinate.into(), value);
    }
    pub fn get_merge_cells(&self) -> &Vec<String> {
        &self.merge_cells
    }
    pub(crate) fn add_merge_cells_crate<S: Into<String>>(&mut self, value:S) {
        self.merge_cells.push(value.into());
    }
    pub fn get_auto_filter(&self) -> &AutoFilter {
        &self.auto_filter
    }
    pub(crate) fn set_auto_filter(&mut self, value:AutoFilter) {
        self.auto_filter = value;
    }
    pub fn get_code_name(&self) -> &String {
        &self.code_name
    }
    pub(crate) fn set_code_name(&mut self, value:String) {
        self.code_name = value;
    }
    pub fn get_header_footer(&self) -> &HeaderFooter {
        &self.header_footer
    }
    pub(crate) fn set_header_footer(&mut self, value:HeaderFooter) {
        self.header_footer = value;
    }
    pub fn get_comments(&self) -> &Vec<String> {
        &self.comments
    }
    pub(crate) fn add_comments(&mut self, value:String) {
        self.comments.push(value);
    }
    pub fn get_hyperlink_collection(&self) -> &Vec<Hyperlink> {
        &self.hyperlink_collection
    }
    pub(crate) fn add_hyperlink_collection(&mut self, value:Hyperlink) {
        self.hyperlink_collection.push(value);
    }
    pub fn get_row_dimension(&self, row:usize) -> Option<&RowDimension> {
        self.row_dimensions.get(&row)
    }
    pub(crate) fn set_row_dimension(&mut self, row:usize, value:RowDimension) {
        &self.row_dimensions.insert(row, value);
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
    pub fn get_cell_collection(&self) -> &Cells {
        &self.cell_collection
    }
    pub fn get_row_dimensions(&self) -> &BTreeMap<usize, RowDimension> {
        &self.row_dimensions
    }
    pub fn get_default_row_dimension(&self) -> &RowDimension {
        &self.default_row_dimension
    }
    pub fn get_column_dimensions(&self) -> &Vec<ColumnDimension> {
        &self.column_dimensions
    }
    pub(crate) fn set_column_dimensions(&mut self, value:ColumnDimension) {
        &self.column_dimensions.push(value);
    }
    pub fn get_default_column_dimension(&self) -> &ColumnDimension {
        &self.default_column_dimension
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
    pub fn get_chart_collection(&self) -> &Vec<Chart> {
        &self.chart_collection
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
    pub fn has_drawing_object(&self) -> bool {
        if self.chart_collection.len() > 0 {
            return true;
        }
        false
    }
    pub fn has_code_name(&self) -> bool {
        self.code_name != ""
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
        let highest = &self.get_cell_collection().get_highest_row_and_column();
        if highest["row"] == "0" {
            return "A1".to_string();
        }
        format!("A1:{}{}", highest["column"], highest["row"])
    }
    pub fn get_highest_column(&self) -> &String {
        return &self.cached_highest_column;
    }
    pub fn get_highest_row(&self) -> &String {
        return &self.cached_highest_row;
    }
    pub fn get_title(&self) -> &String {
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
    pub fn get_cell<S: Into<String>>(&self, coordinate:S)->Option<&Cell> {
        let coordinate_upper = coordinate.into().to_uppercase();
        if self.cell_collection.has(&coordinate_upper) == false {
            return None;
        }
        Some(self.cell_collection.get(&coordinate_upper).unwrap())
    }
    pub fn get_cell_mut<S: Into<String>>(&mut self, coordinate:S)->&mut Cell {
        let coordinate_upper = coordinate.into().to_uppercase();
        let coordinate_prm = coordinate_from_string(&coordinate_upper);
        let row: usize = coordinate_prm[1].parse::<usize>().unwrap();
        match self.row_dimensions.get(&row) {
            Some(_) => {},
            None => {
                self.set_row_dimension(row, RowDimension::default())
            }
        }
        if self.cell_collection.has(&coordinate_upper) == false {
            self.create_new_cell(&coordinate_upper);
        }
        self.cell_collection.get_mut(&coordinate_upper)
    }
    pub fn set_cell_value<S: Into<String>>(&mut self, coordinate:S, value:S)-> Result<(), &str> {
        let cell = self.get_cell_mut(coordinate);
        cell.set_value(value)
    }
    pub fn set_cell_value_and_data_type<S: Into<String>>(&mut self, coordinate:S, value:S, data_type:S)-> Result<(), &str> {
        let cell = self.get_cell_mut(coordinate);
        cell.set_value_and_data_type(value, data_type)
    }
    pub(crate) fn create_new_cell(&mut self, coordinate:&String) {
        let cell = Cell::default();
        self.cell_collection.add(coordinate, cell);
        self.cell_collection_is_sorted = false;
        // Coordinates
        //$aCoordinates = Coordinate::coordinateFromString($pCoordinate);
        //if (Coordinate::columnIndexFromString($this->cachedHighestColumn) < Coordinate::columnIndexFromString($aCoordinates[0])) {
        //    $this->cachedHighestColumn = $aCoordinates[0];
        //}
        //if ($aCoordinates[1] > $this->cachedHighestRow) {
        //    $this->cachedHighestRow = $aCoordinates[1];
        //}

        // Cell needs appropriate xfIndex from dimensions records
        //    but don't create dimension records if they don't already exist
        //$rowDimension = $this->getRowDimension($aCoordinates[1], false);
        //$columnDimension = $this->getColumnDimension($aCoordinates[0], false);

        //if ($rowDimension !== null && $rowDimension->getXfIndex() > 0) {
        //   // then there is a row dimension with explicit style, assign it to the cell
        //    $cell->setXfIndex($rowDimension->getXfIndex());
        //} elseif ($columnDimension !== null && $columnDimension->getXfIndex() > 0) {
        //    // then there is a column dimension, assign it to the cell
        //    $cell->setXfIndex($columnDimension->getXfIndex());
        //}

    }
}
