use helper::address::*;
use helper::coordinate::*;
use reader::xlsx::*;
use structs::Address;
use structs::Calculation;
use structs::CellValue;
use structs::Properties;
use structs::Security;
use structs::SharedStringTable;
use structs::Stylesheet;
use structs::Theme;
use structs::WorkbookView;
use structs::Worksheet;
use std::sync::Arc;
use std::sync::RwLock;

/// A Spreadsheet Object.
/// The starting point of all struct.
#[derive(Clone, Default, Debug)]
pub struct Spreadsheet {
    properties: Properties,
    security: Security,
    work_sheet_collection: Vec<Worksheet>,
    calculation_engine: Calculation,
    named_ranges: Vec<String>,
    macros_code: Option<Vec<u8>>,
    macros_certificate: String,
    ribbon_xml_data: Option<String>,
    ribbon_bin_objects: Option<Vec<String>>,
    show_horizontal_scroll: bool,
    show_vertical_scroll: bool,
    show_sheet_tabs: bool,
    minimized: bool,
    auto_filter_date_grouping: bool,
    first_sheet_index: i32,
    visibility: String,
    tab_ratio: i32,
    theme: Theme,
    stylesheet: Stylesheet,
    shared_string_table: Arc<RwLock<SharedStringTable>>,
    workbook_view: WorkbookView,
    backup_context_types: Vec<(String, String)>,
    pivot_caches: Vec<(String, String, String)>,
}
impl Spreadsheet {
    // ************************
    // update Coordinate
    // ************************

    /// Insert new rows.
    /// # Arguments
    /// * `sheet_name` - Specify the sheet name. ex) "Sheet1"
    /// * `row_index` - Specify point of insert. ex) 1
    /// * `num_rows` - Specify number to insert. ex) 2
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// book.insert_new_row("Sheet1", 2, 3);
    /// ```
    pub fn insert_new_row<S: Into<String>>(
        &mut self,
        sheet_name: S,
        row_index: u32,
        num_rows: u32,
    ) {
        self.adjustment_insert_coordinate(&sheet_name.into(), &0, &0, &row_index, &num_rows);
    }

    /// Insert new colmuns.
    /// # Arguments
    /// * `sheet_name` - Specify the sheet name. ex) "Sheet1"
    /// * `column` - Specify point of insert. ex) "B"
    /// * `num_columns` - Specify number to insert. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// book.insert_new_colmun("Sheet1", "B", 3);
    /// ```
    pub fn insert_new_colmun<S: Into<String>>(
        &mut self,
        sheet_name: S,
        column: S,
        num_columns: u32,
    ) {
        let column_upper = column.into().to_uppercase();
        let column_index = column_index_from_string(column_upper);
        self.insert_new_colmun_by_index(&sheet_name.into(), column_index, num_columns);
    }

    /// Insert new colmuns.
    /// # Arguments
    /// * `sheet_name` - Specify the sheet name. ex) "Sheet1"
    /// * `column_index` - Specify point of insert. ex) 2
    /// * `num_columns` - Specify number to insert. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// book.insert_new_colmun_by_index("Sheet1", 2, 3);
    /// ```
    pub fn insert_new_colmun_by_index<S: Into<String>>(
        &mut self,
        sheet_name: S,
        column_index: u32,
        num_columns: u32,
    ) {
        self.adjustment_insert_coordinate(&sheet_name.into(), &column_index, &num_columns, &0, &0);
    }

    /// Remove rows.
    /// # Arguments
    /// * `sheet_name` - Specify the sheet name. ex) "Sheet1"
    /// * `row_index` - Specify point of remove. ex) 1
    /// * `num_rows` - Specify number to remove. ex) 2
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// book.remove_row("Sheet1", 2, 3);
    /// ```
    pub fn remove_row<S: Into<String>>(&mut self, sheet_name: S, row_index: u32, num_rows: u32) {
        self.adjustment_remove_coordinate(&sheet_name.into(), &0, &0, &row_index, &num_rows);
    }

    /// Remove colmuns.
    /// # Arguments
    /// * `sheet_name` - Specify the sheet name. ex) "Sheet1"
    /// * `column` - Specify point of remove. ex) "B"
    /// * `num_columns` - Specify number to remove. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// book.remove_colmun("Sheet1", "B", 3);
    /// ```
    pub fn remove_colmun<S: Into<String>>(&mut self, sheet_name: S, column: S, num_columns: u32) {
        let column_upper = column.into().to_uppercase();
        let column_index = column_index_from_string(column_upper);
        self.remove_colmun_by_index(&sheet_name.into(), column_index, num_columns);
    }

    /// Remove colmuns.
    /// # Arguments
    /// * `sheet_name` - Specify the sheet name. ex) "Sheet1"
    /// * `column_index` - Specify point of remove. ex) 2
    /// * `num_columns` - Specify number to remove. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// book.remove_colmun_by_index("Sheet1", 2, 3);
    /// ```
    pub fn remove_colmun_by_index<S: Into<String>>(
        &mut self,
        sheet_name: S,
        column_index: u32,
        num_columns: u32,
    ) {
        self.adjustment_remove_coordinate(&sheet_name.into(), &column_index, &num_columns, &0, &0);
    }

    /// (This method is crate only.)
    /// Adjustment Insert Coordinate.
    pub(crate) fn adjustment_insert_coordinate(
        &mut self,
        sheet_name: &str,
        column_index: &u32,
        num_columns: &u32,
        row_index: &u32,
        num_rows: &u32,
    ) {
        self.read_sheet_collection();
        for worksheet in &mut self.work_sheet_collection {
            worksheet.adjustment_insert_coordinate(
                sheet_name,
                column_index,
                num_columns,
                row_index,
                num_rows,
            );
        }
    }

    /// (This method is crate only.)
    /// Adjustment Remove Coordinate.
    pub(crate) fn adjustment_remove_coordinate(
        &mut self,
        sheet_name: &str,
        column_index: &u32,
        num_columns: &u32,
        row_index: &u32,
        num_rows: &u32,
    ) {
        self.read_sheet_collection();
        for worksheet in &mut self.work_sheet_collection {
            worksheet.adjustment_remove_coordinate(
                sheet_name,
                column_index,
                num_columns,
                row_index,
                num_rows,
            );
        }
    }

    /// Gets the cell value by specifying an address.
    /// # Arguments
    /// * `address` - address. ex) "Sheet1!A1:C5"
    /// # Return value
    /// *`Vec<&CellValue>` - CellValue List.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut cell_value_List = book.get_cell_value_by_address("Sheet1!A1:C5");
    /// ```
    pub fn get_cell_value_by_address<S: Into<String>>(&self, address: S) -> Vec<&CellValue> {
        let (sheet_name, range) = split_address(address);
        self.get_sheet_by_name(sheet_name)
            .unwrap()
            .get_cell_value_by_range(range)
    }

    /// (This method is crate only.)
    /// Gets the cell value by specifying an Address Object.
    /// # Arguments
    /// * `address` - Address Object
    /// # Return value
    /// *`Vec<&CellValue>` - CellValue List.
    pub(crate) fn get_cell_value_by_address_crate(&self, address: &Address) -> Vec<&CellValue> {
        self.get_sheet_by_name(address.get_sheet_name())
            .unwrap()
            .get_cell_value_by_range(address.get_range().get_range())
    }

    /// Get Theme.
    pub fn get_theme(&self) -> &Theme {
        &self.theme
    }

    /// Get Theme in mutable.
    pub fn get_theme_mut(&mut self) -> &mut Theme {
        &mut self.theme
    }

    /// Set Theme.
    /// # Arguments
    /// * `value` - Theme
    pub fn set_theme(&mut self, value: Theme) -> &mut Self {
        self.theme = value;
        self
    }

    /// Get Properties.
    pub fn get_properties(&self) -> &Properties {
        &self.properties
    }

    /// Get Properties in mutable.
    pub fn get_properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Set Properties.
    /// # Arguments
    /// * `value` - Properties
    pub fn set_properties(&mut self, value: Properties) -> &mut Self {
        self.properties = value;
        self
    }

    /// Get Security.
    pub fn get_security(&self) -> &Security {
        &self.security
    }

    /// Get Security in mutable.
    pub fn get_security_mut(&mut self) -> &mut Security {
        &mut self.security
    }

    /// Set Security.
    /// # Arguments
    /// * `value` - Security
    pub fn set_security(&mut self, value: Security) -> &mut Self {
        self.security = value;
        self
    }

    /// Get Macros Code.
    /// # Return value
    /// * `&Option<Vec<u8>>` - Macros Code Raw Data.
    pub fn get_macros_code(&self) -> &Option<Vec<u8>> {
        &self.macros_code
    }

    /// Set Macros Code.
    /// # Arguments
    /// * `value` - Macros Code Raw Data.
    pub fn set_macros_code(&mut self, value: Vec<u8>) -> &mut Self {
        self.macros_code = Some(value);
        self
    }

    /// Remove Macros Code
    pub fn remove_macros_code(&mut self) -> &mut Self {
        self.macros_code = None;
        self
    }

    /// Has Macros Code
    pub fn get_has_macros(&self) -> bool {
        self.macros_code.is_some()
    }

    /// (This method is crate only.)
    /// Get Stylesheet.
    pub(crate) fn get_stylesheet(&self) -> &Stylesheet {
        &self.stylesheet
    }

    /// (This method is crate only.)
    /// Set Stylesheet.
    /// # Arguments
    /// * `value` - Stylesheet
    pub(crate) fn set_stylesheet(&mut self, value: Stylesheet) -> &mut Self {
        self.stylesheet = value;
        self
    }

    /// (This method is crate only.)
    /// Set Default Value Stylesheet.
    pub(crate) fn set_stylesheet_defalut_value(&mut self) -> &mut Self {
        self.stylesheet.set_defalut_value();
        self
    }

    /// (This method is crate only.)
    /// Get Shared String Table.
    pub(crate) fn get_shared_string_table(&self) -> Arc<RwLock<SharedStringTable>> {
        self.shared_string_table.clone()
    }

    /// (This method is crate only.)
    /// Set Shared String Table.
    /// # Arguments
    /// * `value` - Shared String Table
    pub(crate) fn set_shared_string_table(&mut self, value: SharedStringTable) -> &mut Self {
        self.shared_string_table = Arc::new(RwLock::new(value));
        self
    }

    /// Get Work Sheet List.
    pub fn get_sheet_collection(&self) -> &Vec<Worksheet> {
        for worksheet in &self.work_sheet_collection {
            if worksheet.is_serialized() == false {
                panic!("This Worksheet is Not Serialized. Please exec to read_sheet(&mut self, index: usize);");
            }
        }
        &self.work_sheet_collection
    }

    /// Get Work Sheet List.
    /// No check serialized.
    pub fn get_sheet_collection_no_check(&self) -> &Vec<Worksheet> {
        &self.work_sheet_collection
    }

    /// Get Work Sheet List in mutable.
    pub fn get_sheet_collection_mut(&mut self) -> &mut Vec<Worksheet> {
        self.read_sheet_collection();
        &mut self.work_sheet_collection
    }

    /// Get Work Sheet Count.
    /// # Return value
    /// * `usize` - Work Sheet Count.
    pub fn get_sheet_count(&self) -> usize {
        self.work_sheet_collection.len()
    }

    /// serialize by all worksheet.
    pub fn read_sheet_collection(&mut self) -> &mut Self {
        let theme = self.get_theme().clone();
        let shared_string_table = self.get_shared_string_table().clone();
        let stylesheet = self.get_stylesheet().clone();
        for worksheet in &mut self.work_sheet_collection {
            raw_to_serialize_by_worksheet(worksheet, &theme, shared_string_table.clone(), &stylesheet);
        }
        self
    }

    /// serialize a worksheet.
    pub fn read_sheet(&mut self, index: usize) -> &mut Self {
        let theme = self.get_theme().clone();
        let shared_string_table = self.get_shared_string_table().clone();
        let stylesheet = self.get_stylesheet().clone();
        let worksheet = self.work_sheet_collection.get_mut(index).unwrap();
        raw_to_serialize_by_worksheet(worksheet, &theme, shared_string_table, &stylesheet);
        self
    }

    /// Get Work Sheet.
    /// # Arguments
    /// * `index` - sheet index
    /// # Return value
    /// * `Result<&Worksheet, &'static str>` - OK:work sheet. Err:Error.
    pub fn get_sheet(&self, index: usize) -> Result<&Worksheet, &'static str> {
        match self.work_sheet_collection.get(index) {
            Some(v) => {
                if v.is_serialized() == false {
                    panic!("This Worksheet is Not Serialized. Please exec to read_sheet(&mut self, index: usize);");
                }
                return Ok(v);
            }
            None => return Err("Not found."),
        }
    }

    /// Get Work Sheet in mutable.
    /// # Arguments
    /// * `index` - sheet index
    /// # Return value
    /// * `&mut Worksheet` - Work sheet.
    pub fn get_sheet_mut(&mut self, index: usize) -> &mut Worksheet {
        let theme = self.get_theme().clone();
        let shared_string_table = self.get_shared_string_table().clone();
        let stylesheet = self.get_stylesheet().clone();
        let worksheet = self.work_sheet_collection.get_mut(index).unwrap();
        raw_to_serialize_by_worksheet(worksheet, &theme, shared_string_table, &stylesheet);
        worksheet
    }

    /// Get Work Sheet.
    /// # Arguments
    /// * `sheet_name` - sheet name
    /// # Return value
    /// * `Result<&Worksheet, &'static str>` - OK:work sheet. Err:Error.
    pub fn get_sheet_by_name<S: Into<String>>(
        &self,
        sheet_name: S,
    ) -> Result<&Worksheet, &'static str> {
        let v = sheet_name.into();
        for sheet in &self.work_sheet_collection {
            if sheet.get_title() == &v {
                if sheet.is_serialized() == false {
                    panic!("This Worksheet is Not Serialized. Please exec to read_sheet(&mut self, index: usize);");
                }
                return Ok(sheet);
            }
        }
        Err("not found.")
    }

    /// Get Work Sheet in mutable.
    /// # Arguments
    /// * `sheet_name` - sheet name
    /// # Return value
    /// * `Result<&mut Worksheet, &'static str>` - OK:work sheet. Err:Error.
    pub fn get_sheet_by_name_mut<S: Into<String>>(
        &mut self,
        sheet_name: S,
    ) -> Result<&mut Worksheet, &'static str> {
        let theme = self.get_theme().clone();
        let shared_string_table = self.get_shared_string_table();
        let stylesheet = self.get_stylesheet().clone();
        let v = sheet_name.into();
        for worksheet in &mut self.work_sheet_collection {
            if worksheet.get_title() == &v {
                raw_to_serialize_by_worksheet(worksheet, &theme, shared_string_table, &stylesheet);
                return Ok(worksheet);
            }
        }
        Err("not found.")
    }

    /// Get Active Work Sheet.
    /// # Return value
    /// * `Result<&Worksheet, &'static str>` - OK:work sheet. Err:Error.
    pub fn get_active_sheet(&self) -> Result<&Worksheet, &'static str> {
        let index = self.get_workbook_view().get_active_tab().clone();
        match self.work_sheet_collection.get(index as usize) {
            Some(v) => {
                if v.is_serialized() == false {
                    panic!("This Worksheet is Not Serialized. Please exec to read_sheet(&mut self, index: usize);");
                }
                return Ok(v);
            }
            None => return Err("Not found."),
        }
    }

    /// Get Active Work Sheet in mutable.
    /// # Return value
    /// * `&mut Worksheet` - Work sheet.
    pub fn get_active_sheet_mut(&mut self) -> &mut Worksheet {
        let theme = self.get_theme().clone();
        let shared_string_table = self.get_shared_string_table().clone();
        let stylesheet = self.get_stylesheet().clone();
        let index = self.get_workbook_view().get_active_tab().clone();
        let worksheet = self.work_sheet_collection.get_mut(index as usize).unwrap();
        raw_to_serialize_by_worksheet(worksheet, &theme, shared_string_table, &stylesheet);
        worksheet
    }

    /// Add Work Sheet.
    /// # Arguments
    /// * `value` - Work Sheet
    /// # Return value
    /// * `Result<&mut Worksheet, &'static str>` - OK:added work sheet. Err:Error.
    pub fn add_sheet(&mut self, value: Worksheet) -> Result<&mut Worksheet, &'static str> {
        let title = value.get_title();
        match Spreadsheet::check_sheet_title(self, title) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        self.work_sheet_collection.push(value);
        Ok(self.work_sheet_collection.last_mut().unwrap())
    }

    /// Add New Work Sheet.
    /// # Arguments
    /// * `sheet_title` - sheet title
    /// # Return value
    /// * `Result<&mut Worksheet, &'static str>` - OK:added work sheet. Err:Error.
    pub fn new_sheet<S: Into<String>>(
        &mut self,
        sheet_title: S,
    ) -> Result<&mut Worksheet, &'static str> {
        let v = sheet_title.into();
        match Spreadsheet::check_sheet_title(self, &v) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        let sheet_id = (self.work_sheet_collection.len() + 1).to_string();
        Ok(Spreadsheet::add_new_sheet_crate(self, sheet_id, v))
    }

    /// (This method is crate only.)
    /// Add New Work Sheet.
    /// # Arguments
    /// * `index` - sheet index
    /// * `sheet_title` - sheet title
    /// # Return value
    /// * `&mut Worksheet` - added work sheet.
    pub(crate) fn add_new_sheet_crate<S: Into<String>>(
        &mut self,
        sheet_id: S,
        sheet_title: S,
    ) -> &mut Worksheet {
        let mut worksheet = Worksheet::default();
        worksheet.set_sheet_id(sheet_id);
        worksheet.set_title(sheet_title.into());
        self.work_sheet_collection.push(worksheet);
        self.work_sheet_collection.last_mut().unwrap()
    }

    /// Set Sheet Title.
    /// # Arguments
    /// * `index` - target sheet index
    /// * `sheet_title` - sheet title
    /// # Return value
    /// * `Result<(), &'static str>` - OK:Success  Err:Error.
    pub fn set_sheet_title<S: Into<String>>(
        &mut self,
        index: usize,
        sheet_title: S,
    ) -> Result<(), &'static str> {
        let v = sheet_title.into();
        match Spreadsheet::check_sheet_title(self, &v) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        match self.work_sheet_collection.get_mut(index) {
            Some(sheet) => {
                sheet.set_title(v);
                Ok(())
            }
            None => return Err("sheet not found."),
        }
    }

    /// (This method is crate only.)
    /// Check for duplicate sheet title.
    /// # Arguments
    /// * `value` - sheet title
    /// # Return value
    /// * `Result<(), &'static str>` - OK:Not duplicate Err:Duplicate.
    pub(crate) fn check_sheet_title<S: Into<String>>(&self, value: S) -> Result<(), &'static str> {
        let v = value.into();
        for work_sheet in &self.work_sheet_collection {
            if &v == work_sheet.get_title() {
                return Err("title duplicate.");
            }
        }
        Ok(())
    }

    /// (This method is crate only.)
    /// Has Ribbon XML Data.
    pub(crate) fn has_ribbon(&self) -> bool {
        self.ribbon_xml_data.is_some()
    }

    /// Get Workbook View.
    pub fn get_workbook_view(&self) -> &WorkbookView {
        &self.workbook_view
    }

    /// Get Workbook View in mutable.
    pub fn get_workbook_view_mut(&mut self) -> &mut WorkbookView {
        &mut self.workbook_view
    }

    /// Set Workbook View.
    /// # Arguments
    /// * `value` - WorkbookView
    pub fn set_workbook_view(&mut self, value: WorkbookView) -> &mut Self {
        self.workbook_view = value;
        self
    }

    /// (This method is crate only.)
    /// Has Defined Names.
    pub(crate) fn has_defined_names(&self) -> bool {
        for sheet in self.get_sheet_collection_no_check() {
            if sheet.has_defined_names() {
                return true;
            }
        }
        false
    }

    pub(crate) fn get_backup_context_types(&self) -> &Vec<(String, String)> {
        &self.backup_context_types
    }

    pub(crate) fn set_backup_context_types(&mut self, value: Vec<(String, String)>) -> &mut Self {
        self.backup_context_types = value;
        self
    }

    pub(crate) fn get_pivot_caches(&self) -> Vec<(String, String, String)> {
        let mut result: Vec<(String, String, String)> = Vec::new();
        for (val1, val2, val3) in &self.pivot_caches {
            for worksheet in self.get_sheet_collection_no_check() {
                for pivot_cache_definition in worksheet.get_pivot_cache_definition_collection() {
                    let val3_up = format!("xl/{}", val3);
                    if val3_up.as_str() == pivot_cache_definition {
                        let mut is_new = true;
                        for (_, _, r_val3) in &result {
                            if r_val3 == val3 {
                                is_new = false;
                            }
                        }
                        if is_new {
                            result.push((val1.clone(), val2.clone(), val3.clone()));
                        }
                    }
                }
            }
        }
        result
    }

    pub(crate) fn add_pivot_caches(&mut self, value: (String, String, String)) -> &mut Self {
        self.pivot_caches.push(value);
        self
    }

    pub(crate) fn update_pivot_caches(&mut self, key: String, value: String) -> &mut Self {
        let mut result: Vec<(String, String, String)> = Vec::new();
        for (val1, val2, val3) in &self.pivot_caches {
            let mut result_value = value.clone();
            if val1 != &key {
                result_value = val3.clone();
            }
            result.push((val1.clone(), val2.clone(), result_value));
        }
        self.pivot_caches = result;
        self
    }
}
