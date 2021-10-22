use super::Worksheet;
use super::Properties;
use super::Security;
use super::Calculation;
use super::Style;
use super::DefinedName;
use super::Theme;
use super::Stylesheet;
use super::SharedStringTable;
use super::WorkbookView;
use helper::coordinate::*;

#[derive(Default, Debug)]
pub struct Spreadsheet {
    properties: Properties,
    security: Security,
    work_sheet_collection: Vec<Worksheet>,
    calculation_engine: Calculation,
    named_ranges: Vec<String>,
    has_macros: bool,
    macros_code: Option<Vec<u8>>,
    macros_certificate: String,
    ribbon_xml_data: Option<String>,
    ribbon_bin_objects: Option<Vec<String>>,
    unparsed_loaded_data: Vec<String>,
    show_horizontal_scroll: bool,
    show_vertical_scroll: bool,
    show_sheet_tabs: bool,
    minimized: bool,
    auto_filter_date_grouping: bool,
    first_sheet_index: i32,
    visibility: String,
    tab_ratio: i32,
    theme: Theme,
    defined_names: Vec<DefinedName>,
    stylesheet: Stylesheet,
    shared_string_table: SharedStringTable,
    workbook_view: WorkbookView,
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
    pub fn insert_new_row<S: Into<String>>(&mut self, sheet_name:S, row_index:u32, num_rows:u32) {
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
    pub fn insert_new_colmun<S: Into<String>>(&mut self, sheet_name:S, column:S, num_columns:u32) {
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
    pub fn insert_new_colmun_by_index<S: Into<String>>(&mut self, sheet_name:S, column_index:u32, num_columns:u32) {
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
    pub fn remove_row<S: Into<String>>(&mut self, sheet_name:S, row_index:u32, num_rows:u32) {
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
    pub fn remove_colmun<S: Into<String>>(&mut self, sheet_name:S, column:S, num_columns:u32) {
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
    pub fn remove_colmun_by_index<S: Into<String>>(&mut self, sheet_name:S, column_index:u32, num_columns:u32) {
        self.adjustment_remove_coordinate(&sheet_name.into(), &column_index, &num_columns, &0, &0);
    }

    pub(crate) fn adjustment_insert_coordinate(&mut self, sheet_name:&str, column_index:&u32, num_columns:&u32, row_index:&u32, num_rows:&u32) {
        for defined_name in &mut self.defined_names {
            defined_name.get_address_obj_mut().adjustment_insert_coordinate(sheet_name, column_index, num_columns, row_index, num_rows);
        }
        for worksheet in &mut self.work_sheet_collection {
            worksheet.adjustment_insert_coordinate(sheet_name, column_index, num_columns, row_index, num_rows);
        }
    }

    pub(crate) fn adjustment_remove_coordinate(&mut self, sheet_name:&str, column_index:&u32, num_columns:&u32, row_index:&u32, num_rows:&u32) {
        self.defined_names.retain(|x| {
            !(x.get_address_obj().is_remove(sheet_name, column_index, num_columns, row_index, num_rows))
        });
        for defined_name in &mut self.defined_names {
            defined_name.get_address_obj_mut().adjustment_remove_coordinate(sheet_name, column_index, num_columns, row_index, num_rows);
        }
        for worksheet in &mut self.work_sheet_collection {
            worksheet.adjustment_remove_coordinate(sheet_name, column_index, num_columns, row_index, num_rows);
        }
    }

    pub fn get_defined_names(&self) -> &Vec<DefinedName> {
        &self.defined_names
    }

    pub fn get_defined_names_mut(&mut self) -> &mut Vec<DefinedName> {
        &mut self.defined_names
    }

    pub fn set_defined_names(&mut self, value:Vec<DefinedName>) {
        self.defined_names = value;
    }

    pub fn add_defined_names(&mut self, value:DefinedName) {
        self.defined_names.push(value);
    }

    pub fn add_defined_name<S: Into<String>>(&mut self, name:S, address:S)->Result<(), &str> {
        let mut defined_name = DefinedName::default();
        defined_name.set_name(name.into());
        defined_name.set_address(address.into());
        self.defined_names.push(defined_name);
        Ok(())
    }

    pub(crate) fn get_all_conditional_style_list(&self) -> Vec<(String, Style)> {
        let mut result:Vec<(String, Style)> = Vec::new();
        for work_sheet in &self.work_sheet_collection {
            for conditional_formatting in work_sheet.get_conditional_styles_collection() {
                for conditional in conditional_formatting.get_conditional_collection() {
                    match conditional.get_style() {
                        Some(v) => {
                            let mut is_match = false;
                            for (hash, _) in &result {
                                if hash == &v.get_hash_code() {
                                    is_match = true;
                                    break;
                                }
                            }
                            if is_match == false {
                                result.push((v.get_hash_code(), v.clone()));
                            }
                        },
                        None => {}
                    }
                }
            }
        }
        result
    }

    pub fn get_theme(&self) -> &Theme {
        &self.theme
    }

    pub fn get_theme_mut(&mut self) -> &mut Theme {
        &mut self.theme
    }

    pub(crate) fn set_theme(&mut self, value:Theme) {
        self.theme = value;
    }

    pub(crate) fn has_comment(&self) -> bool {
        for worksheet in &self.work_sheet_collection {
            if worksheet.get_comments().len() > 0 {
                return true;
            }
        }
        false
    }

    pub fn get_properties(&self) -> &Properties {
        &self.properties
    }

    pub fn get_properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }
    
    pub fn set_properties(&mut self, value:Properties) {
        self.properties = value;
    }

    pub fn get_security(&self) -> &Security {
        &self.security
    }

    pub fn get_security_mut(&mut self) -> &mut Security {
        &mut self.security
    }

    pub fn set_security(&mut self, value:Security) {
        self.security = value;
    }

    pub fn get_macros_code(&self) -> &Option<Vec<u8>> {
        &self.macros_code
    }

    pub(crate) fn set_macros_code(&mut self, value:Vec<u8>) {
        self.macros_code = Some(value);
    }
    
    pub fn get_has_macros(&self) -> &bool {
        &self.has_macros
    }

    pub(crate) fn get_has_macros_mut(&mut self) -> &mut bool {
        &mut self.has_macros
    }

    pub(crate) fn set_has_macros(&mut self, value:bool) {
        self.has_macros = value;
    }

    pub fn get_unparsed_loaded_data(&self) -> &Vec<String> {
        &self.unparsed_loaded_data
    }

    pub fn set_unparsed_loaded_data(&mut self, value:Vec<String>) {
        self.unparsed_loaded_data = value;
    }

    pub(crate) fn get_stylesheet(&self) -> &Stylesheet {
        &self.stylesheet
    }

    pub(crate) fn get_stylesheet_mut(&mut self) -> &mut Stylesheet {
        &mut self.stylesheet
    }

    pub(crate) fn set_stylesheet(&mut self, value:Stylesheet) -> &mut Self {
        self.stylesheet = value;
        self
    }

    pub(crate) fn remove_stylesheet(&mut self) -> &mut Self {
        self.stylesheet = Stylesheet::default();
        self
    }

    pub(crate) fn get_shared_string_table(&self) -> &SharedStringTable {
        &self.shared_string_table
    }

    pub(crate) fn get_shared_string_table_mut(&mut self) -> &mut SharedStringTable {
        &mut self.shared_string_table
    }

    pub(crate) fn set_shared_string_table(&mut self, value:SharedStringTable) -> &mut Self {
        self.shared_string_table = value;
        self
    }

    pub(crate) fn remove_shared_string_table(&mut self) -> &mut Self {
        self.shared_string_table = SharedStringTable::default();
        self
    }

    pub fn get_sheet_collection(&self) -> &Vec<Worksheet> {
        &self.work_sheet_collection
    }

    pub fn get_sheet_count(&self) -> usize {
        self.work_sheet_collection.len()
    }

    pub fn get_sheet(&self, index:usize) -> Result<&Worksheet, &'static str> {
        match &self.work_sheet_collection.get(index) {
            Some(v) => return Ok(v),
            None => return Err("Not found.")
        }
    }

    pub fn get_sheet_mut(&mut self, index:usize) -> &mut Worksheet {
        self.work_sheet_collection.get_mut(index).unwrap()
    }

    pub fn get_sheet_by_name<S: Into<String>>(&self, value:S) -> Result<&Worksheet, &'static str> {
        let v = value.into();
        for sheet in &self.work_sheet_collection {
            if sheet.get_title() == &v {
                return Ok(sheet);
            }
        }
        Err("not found.")
    }

    pub fn get_sheet_by_name_mut<S: Into<String>>(&mut self, value:S) -> Result<&mut Worksheet, &'static str> {
        let v = value.into();
        for sheet in &mut self.work_sheet_collection {
            if sheet.get_title() == &v {
                return Ok(sheet);
            }
        }
        Err("not found.")
    }

    pub fn get_sheet_by_sheet_id<S: Into<String>>(&self, value:S) -> Result<&Worksheet, &'static str> {
        let v = value.into();
        for sheet in &self.work_sheet_collection {
            if sheet.get_sheet_id() == &v {
                return Ok(sheet);
            }
        }
        Err("not found.")
    }

    pub fn get_sheet_by_sheet_id_mut<S: Into<String>>(&mut self, value:S) -> Result<&mut Worksheet, &'static str> {
        let v = value.into();
        for sheet in &mut self.work_sheet_collection {
            if sheet.get_sheet_id() == &v {
                return Ok(sheet);
            }
        }
        Err("not found.")
    }

    pub fn new_sheet<S: Into<String>>(&mut self, value:S) -> Result<&mut Worksheet, &'static str> {
        let v = value.into();
        match Spreadsheet::check_sheet_title(self, &v) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }
        let sheet_id = (self.work_sheet_collection.len() + 1).to_string();
        Ok(Spreadsheet::new_sheet_crate(self, sheet_id, v))
    }

    pub(crate) fn new_sheet_crate<S: Into<String>>(&mut self, sheet_id:S, value:S) -> &mut Worksheet {
        let mut worksheet = Worksheet::default();
        worksheet.set_sheet_id(sheet_id.into());
        worksheet.set_title(value.into());
        self.work_sheet_collection.push(worksheet);
        self.work_sheet_collection.last_mut().unwrap()
    }

    pub fn set_sheet_title<S: Into<String>>(&mut self, index:usize, value:S) -> Result<(), &'static str>{
        let v = value.into();
        match Spreadsheet::check_sheet_title(self, &v) {
            Ok(_) => {},
            Err(e) => return Err(e)
        }
        match self.work_sheet_collection.get_mut(index) {
            Some(sheet) => {
                sheet.set_title(v);
                Ok(())
            },
            None => return Err("sheet not found.")
        }
    }

    pub(crate) fn check_sheet_title<S: Into<String>>(&self, value:S) -> Result<(), &'static str> {
        let v = value.into();
        for work_sheet in &self.work_sheet_collection {
            if &v == work_sheet.get_title() {
                return Err("title duplicate.");
            }
        }
        Ok(())
    }

    pub fn has_ribbon(&self) -> bool {
        self.ribbon_xml_data.is_some()
    }

    pub fn has_formula(&self) -> bool {
        for worksheet in &self.work_sheet_collection {
            for cell in worksheet.get_cell_collection() {
                if cell.get_formula() != "" {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_workbook_view(&self) -> &WorkbookView {
        &self.workbook_view
    }

    pub fn get_workbook_view_mut(&mut self) -> &mut WorkbookView {
        &mut self.workbook_view
    }

    pub fn set_workbook_view(&mut self, value:WorkbookView) -> &mut Self {
        self.workbook_view = value;
        self
    }
}
