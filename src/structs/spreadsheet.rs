use crate::helper::address::*;
use crate::helper::coordinate::*;
use crate::reader::xlsx::*;
use crate::structs::drawing::Theme;
use crate::structs::Address;
use crate::structs::CellValue;
use crate::structs::Cells;
use crate::structs::DefinedName;
use crate::structs::Properties;
use crate::structs::SharedStringTable;
use crate::structs::Stylesheet;
use crate::structs::WorkbookProtection;
use crate::structs::WorkbookView;
use crate::structs::Worksheet;
use crate::traits::AdjustmentCoordinate;
use crate::traits::AdjustmentCoordinateWithSheet;
use crate::StringValue;
use std::sync::Arc;
use std::sync::RwLock;
use thin_vec::ThinVec;

/// A Spreadsheet Object.
/// The starting point of all struct.
#[derive(Clone, Default, Debug)]
pub struct Spreadsheet {
    properties: Properties,
    work_sheet_collection: ThinVec<Worksheet>,
    macros_code: Option<ThinVec<u8>>,
    code_name: StringValue,
    ribbon_xml_data: StringValue,
    theme: Theme,
    stylesheet: Stylesheet,
    shared_string_table: Arc<RwLock<SharedStringTable>>,
    workbook_view: WorkbookView,
    backup_context_types: ThinVec<(Box<str>, Box<str>)>,
    pivot_caches: ThinVec<(Box<str>, Box<str>, Box<str>)>,
    workbook_protection: Option<Box<WorkbookProtection>>,
    defined_names: ThinVec<DefinedName>,
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
    /// book.insert_new_row("Sheet1", &2, &3);
    /// ```
    #[inline]
    pub fn insert_new_row(&mut self, sheet_name: &str, row_index: u32, num_rows: u32) {
        self.adjustment_insert_coordinate_with_sheet(sheet_name, 0, 0, row_index, num_rows);
    }

    /// Insert new columns.
    /// # Arguments
    /// * `sheet_name` - Specify the sheet name. ex) "Sheet1"
    /// * `column` - Specify point of insert. ex) "B"
    /// * `num_columns` - Specify number to insert. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// book.insert_new_column("Sheet1", "B", &3);
    /// ```
    #[inline]
    pub fn insert_new_column(&mut self, sheet_name: &str, column: &str, num_columns: u32) {
        let column_upper = column.to_uppercase();
        let column_index = column_index_from_string(column_upper);
        self.insert_new_column_by_index(sheet_name, column_index, num_columns);
    }

    /// Insert new columns.
    /// # Arguments
    /// * `sheet_name` - Specify the sheet name. ex) "Sheet1"
    /// * `column_index` - Specify point of insert. ex) 2
    /// * `num_columns` - Specify number to insert. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// book.insert_new_column_by_index("Sheet1", &2, &3);
    /// ```
    #[inline]
    pub fn insert_new_column_by_index(
        &mut self,
        sheet_name: &str,
        column_index: u32,
        num_columns: u32,
    ) {
        self.adjustment_insert_coordinate_with_sheet(sheet_name, column_index, num_columns, 0, 0);
    }

    /// Remove rows.
    /// # Arguments
    /// * `sheet_name` - Specify the sheet name. ex) "Sheet1"
    /// * `row_index` - Specify point of remove. ex) 1
    /// * `num_rows` - Specify number to remove. ex) &2
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// book.remove_row("Sheet1", &2, &3);
    /// ```
    #[inline]
    pub fn remove_row(&mut self, sheet_name: &str, row_index: u32, num_rows: u32) {
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
    /// book.remove_column("Sheet1", "B", &3);
    /// ```
    #[inline]
    pub fn remove_column(&mut self, sheet_name: &str, column: &str, num_columns: u32) {
        let column_upper = column.to_uppercase();
        let column_index = column_index_from_string(column_upper);
        self.remove_column_by_index(sheet_name, column_index, num_columns);
    }

    /// Remove columns.
    /// # Arguments
    /// * `sheet_name` - Specify the sheet name. ex) "Sheet1"
    /// * `column_index` - Specify point of remove. ex) 2
    /// * `num_columns` - Specify number to remove. ex) 3
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// book.remove_column_by_index("Sheet1", &2, &3);
    /// ```
    #[inline]
    pub fn remove_column_by_index(
        &mut self,
        sheet_name: &str,
        column_index: u32,
        num_columns: u32,
    ) {
        self.adjustment_remove_coordinate_with_sheet(sheet_name, column_index, num_columns, 0, 0);
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
    #[inline]
    pub fn get_cell_value_by_address(&self, address: &str) -> Vec<&CellValue> {
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
    #[inline]
    pub(crate) fn get_cell_value_by_address_crate(&self, address: &Address) -> Vec<&CellValue> {
        self.get_sheet_by_name(address.get_sheet_name())
            .unwrap()
            .get_cell_value_by_range(&address.get_range().get_range())
    }

    /// Get Theme.
    #[inline]
    pub fn get_theme(&self) -> &Theme {
        &self.theme
    }

    /// Get Theme in mutable.
    #[inline]
    pub fn get_theme_mut(&mut self) -> &mut Theme {
        &mut self.theme
    }

    /// Set Theme.
    /// # Arguments
    /// * `value` - Theme
    #[inline]
    pub fn set_theme(&mut self, value: Theme) -> &mut Self {
        self.theme = value;
        self
    }

    /// Get Properties.
    #[inline]
    pub fn get_properties(&self) -> &Properties {
        &self.properties
    }

    /// Get Properties in mutable.
    #[inline]
    pub fn get_properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    /// Set Properties.
    /// # Arguments
    /// * `value` - Properties
    #[inline]
    pub fn set_properties(&mut self, value: Properties) -> &mut Self {
        self.properties = value;
        self
    }

    /// Get Macros Code.
    /// # Return value
    /// * `Option<&Vec<u8>>` - Macros Code Raw Data.
    #[inline]
    pub fn get_macros_code(&self) -> Option<&[u8]> {
        self.macros_code.as_deref()
    }

    /// Set Macros Code.
    /// # Arguments
    /// * `value` - Macros Code Raw Data.
    #[inline]
    pub fn set_macros_code(&mut self, value: impl Into<ThinVec<u8>>) -> &mut Self {
        self.macros_code = Some(value.into());
        self
    }

    /// Remove Macros Code
    #[inline]
    pub fn remove_macros_code(&mut self) -> &mut Self {
        self.macros_code = None;
        self
    }

    /// Has Macros Code
    #[inline]
    pub fn get_has_macros(&self) -> bool {
        self.macros_code.is_some()
    }

    /// Set codeName property of workbook
    ///
    /// May be useful when importing VBA/macros code from another workbook
    /// and only used when writing book with macros code
    ///
    /// Default one is `ThisWorkbook`.
    ///
    /// Excel often uses `Workbook________` (8 underscores).
    #[inline]
    pub fn set_code_name<S: Into<String>>(&mut self, codename: S) -> &mut Self {
        self.code_name.set_value(codename);
        self
    }

    /// Get codeName property of workbook
    ///
    /// Must to be the same in workbook with VBA/macros code from this workbook
    /// for that code in Workbook object to work out of the box without adjustments
    #[inline]
    pub fn get_code_name(&self) -> Option<&str> {
        self.code_name.get_value()
    }

    /// (This method is crate only.)
    /// Get Stylesheet.
    #[inline]
    pub(crate) fn get_stylesheet(&self) -> &Stylesheet {
        &self.stylesheet
    }

    /// (This method is crate only.)
    /// Set Stylesheet.
    /// # Arguments
    /// * `value` - Stylesheet
    #[inline]
    pub(crate) fn set_stylesheet(&mut self, value: Stylesheet) -> &mut Self {
        self.stylesheet = value;
        self
    }

    /// (This method is crate only.)
    /// Set Default Value Stylesheet.
    #[inline]
    pub(crate) fn set_stylesheet_defalut_value(&mut self) -> &mut Self {
        self.stylesheet.set_defalut_value();
        self
    }

    /// (This method is crate only.)
    /// Get Shared String Table.
    #[inline]
    pub(crate) fn get_shared_string_table(&self) -> Arc<RwLock<SharedStringTable>> {
        self.shared_string_table.clone()
    }

    /// (This method is crate only.)
    /// Set Shared String Table.
    /// # Arguments
    /// * `value` - Shared String Table
    #[inline]
    pub(crate) fn set_shared_string_table(&mut self, value: SharedStringTable) -> &mut Self {
        self.shared_string_table = Arc::new(RwLock::new(value));
        self
    }

    /// Get Work Sheet List.
    pub fn get_sheet_collection(&self) -> &[Worksheet] {
        for worksheet in &self.work_sheet_collection {
            assert!(worksheet.is_deserialized(),"This Worksheet is Not Deserialized. Please exec to read_sheet(&mut self, index: usize);");
        }
        &self.work_sheet_collection
    }

    /// Get Work Sheet List.
    /// No check deserialized.
    #[inline]
    pub fn get_sheet_collection_no_check(&self) -> &[Worksheet] {
        &self.work_sheet_collection
    }

    /// Get Work Sheet List in mutable.
    #[inline]
    pub fn get_sheet_collection_mut(&mut self) -> &mut ThinVec<Worksheet> {
        self.read_sheet_collection();
        &mut self.work_sheet_collection
    }

    /// Get Work Sheet Count.
    /// # Return value
    /// * `usize` - Work Sheet Count.
    #[inline]
    pub fn get_sheet_count(&self) -> usize {
        self.work_sheet_collection.len()
    }

    /// deserialize by all worksheet.
    #[inline]
    pub fn read_sheet_collection(&mut self) -> &mut Self {
        let shared_string_table = self.get_shared_string_table();
        let stylesheet = self.get_stylesheet().clone();
        for worksheet in &mut self.work_sheet_collection {
            raw_to_deserialize_by_worksheet(worksheet, shared_string_table.clone(), &stylesheet);
        }
        self
    }

    /// deserialize a worksheet.
    #[inline]
    pub fn read_sheet(&mut self, index: usize) -> &mut Self {
        let shared_string_table = self.get_shared_string_table();
        let stylesheet = self.get_stylesheet().clone();
        let worksheet = self.work_sheet_collection.get_mut(index).unwrap();
        raw_to_deserialize_by_worksheet(worksheet, shared_string_table, &stylesheet);
        self
    }

    /// deserialize a worksheet.
    #[inline]
    pub fn read_sheet_by_name(&mut self, sheet_name: &str) -> &mut Self {
        let index = self.find_sheet_index_by_name(sheet_name).unwrap();
        self.read_sheet(index)
    }

    #[inline]
    pub(crate) fn find_sheet_index_by_name(&self, sheet_name: &str) -> Option<usize> {
        self.work_sheet_collection
            .iter()
            .position(|sheet| sheet.get_name() == sheet_name)
    }

    /// Get Work Sheet.
    /// # Arguments
    /// * `index` - sheet index
    /// # Return value
    /// * `Option<&Worksheet>`.
    #[inline]
    pub fn get_sheet(&self, index: usize) -> Option<&Worksheet> {
        self.work_sheet_collection
            .get(index)
            .inspect(|v| {
                assert!(v.is_deserialized(),"This Worksheet is Not Deserialized. Please exec to read_sheet(&mut self, index: usize);");
            })
    }

    /// Get Work Sheet.
    /// # Arguments
    /// * `sheet_name` - sheet name
    /// # Return value
    /// * `Option<&Worksheet>.
    #[inline]
    pub fn get_sheet_by_name(&self, sheet_name: &str) -> Option<&Worksheet> {
        self.find_sheet_index_by_name(sheet_name)
            .and_then(|index| self.get_sheet(index))
    }

    pub fn get_lazy_read_sheet_cells(&self, index: usize) -> Result<Cells, &'static str> {
        let shared_string_table = self.get_shared_string_table();
        self.work_sheet_collection
            .get(index)
            .map(|v| {
                v.get_cell_collection_stream(
                    &shared_string_table.read().unwrap(),
                    self.get_stylesheet(),
                )
            })
            .ok_or("Not found.")
    }

    /// Get Work Sheet in mutable.
    /// # Arguments
    /// * `index` - sheet index
    /// # Return value
    /// * `Option<&mut Worksheet>`.
    #[allow(clippy::manual_inspect)]
    pub fn get_sheet_mut(&mut self, index: usize) -> Option<&mut Worksheet> {
        let shared_string_table = self.get_shared_string_table();
        let stylesheet = self.get_stylesheet().clone();
        self.work_sheet_collection.get_mut(index).map(|v| {
            raw_to_deserialize_by_worksheet(v, shared_string_table, &stylesheet);
            v
        })
    }

    /// Get Work Sheet in mutable.
    /// # Arguments
    /// * `sheet_name` - sheet name
    /// # Return value
    /// * `Option<&mut Worksheet>`.
    #[inline]
    pub fn get_sheet_by_name_mut(&mut self, sheet_name: &str) -> Option<&mut Worksheet> {
        self.find_sheet_index_by_name(sheet_name)
            .and_then(move |index| self.get_sheet_mut(index))
    }

    #[inline]
    pub fn set_active_sheet(&mut self, index: u32) -> &mut Self {
        self.get_workbook_view_mut().set_active_tab(index);
        self
    }

    /// Get Active Work Sheet.
    /// # Return value
    /// * `&Worksheet` - Work sheet.
    #[inline]
    pub fn get_active_sheet(&self) -> &Worksheet {
        let index = self.get_workbook_view().get_active_tab();
        self.get_sheet(index as usize).unwrap()
    }

    /// Get Active Work Sheet in mutable.
    /// # Return value
    /// * `&mut Worksheet` - Work sheet.
    #[inline]
    pub fn get_active_sheet_mut(&mut self) -> &mut Worksheet {
        let index = self.get_workbook_view().get_active_tab();
        self.get_sheet_mut(index as usize).unwrap()
    }

    /// Add Work Sheet.
    /// # Arguments
    /// * `value` - Work Sheet
    /// # Return value
    /// * `Result<&mut Worksheet, &'static str>` - OK:added work sheet. Err:Error.
    #[inline]
    pub fn add_sheet(&mut self, value: Worksheet) -> Result<&mut Worksheet, &'static str> {
        let title = value.get_name();
        Spreadsheet::check_sheet_name(self, title)?;
        self.work_sheet_collection.push(value);
        Ok(self.work_sheet_collection.last_mut().unwrap())
    }

    /// Remove Work Sheet.
    /// # Arguments
    /// * `index` - sheet index
    /// # Return value
    /// * `Result<(), &'static str>` - OK:removed worksheet. Err:Error.
    #[inline]
    pub fn remove_sheet(&mut self, index: usize) -> Result<(), &'static str> {
        if self.work_sheet_collection.len() <= index {
            return Err("out of index.");
        }
        self.work_sheet_collection.remove(index);
        Ok(())
    }

    /// Remove Work Sheet.
    /// # Arguments
    /// * `sheet_name` - sheet name
    /// # Return value
    /// * `Result<(), &'static str>` - OK:removed worksheet. Err:Error.
    pub fn remove_sheet_by_name(&mut self, sheet_name: &str) -> Result<(), &'static str> {
        let cnt_before = self.work_sheet_collection.len();
        self.work_sheet_collection
            .retain(|x| x.get_name() != sheet_name);
        let cnt_after = self.work_sheet_collection.len();
        if cnt_before == cnt_after {
            return Err("out of index.");
        }
        Ok(())
    }

    /// Add New Work Sheet.
    /// # Arguments
    /// * `sheet_title` - sheet title
    /// # Return value
    /// * `Result<&mut Worksheet, &'static str>` - OK:added work sheet. Err:Error.
    #[inline]
    pub fn new_sheet<S: Into<String>>(
        &mut self,
        sheet_title: S,
    ) -> Result<&mut Worksheet, &'static str> {
        let v = sheet_title.into();
        Spreadsheet::check_sheet_name(self, &v)?;
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
        worksheet.set_name(sheet_title.into());
        worksheet
            .get_sheet_format_properties_mut()
            .set_defalut_value();
        self.work_sheet_collection.push(worksheet);
        self.work_sheet_collection.last_mut().unwrap()
    }

    /// Set Sheet Name.
    /// # Arguments
    /// * `index` - target sheet index
    /// * `sheet_name` - sheet name
    /// # Return value
    /// * `Result<(), &'static str>` - OK:Success  Err:Error.
    pub fn set_sheet_name<S: Into<String>>(
        &mut self,
        index: usize,
        sheet_name: S,
    ) -> Result<(), &'static str> {
        let sheet_name_str = sheet_name.into();
        Spreadsheet::check_sheet_name(self, sheet_name_str.as_ref())?;
        self.work_sheet_collection
            .get_mut(index)
            .map(|sheet| {
                sheet.set_name(sheet_name_str);
            })
            .ok_or("sheet not found.")
    }

    /// (This method is crate only.)
    /// Check for duplicate sheet name.
    pub(crate) fn check_sheet_name(&self, value: &str) -> Result<(), &'static str> {
        match self
            .work_sheet_collection
            .iter()
            .any(|work_sheet| value == work_sheet.get_name())
        {
            true => Err("name duplicate."),
            false => Ok(()),
        }
    }

    /// (This method is crate only.)
    /// Has Ribbon XML Data.
    #[inline]
    pub(crate) fn has_ribbon(&self) -> bool {
        self.ribbon_xml_data.has_value()
    }

    /// Get Workbook View.
    #[inline]
    pub fn get_workbook_view(&self) -> &WorkbookView {
        &self.workbook_view
    }

    /// Get Workbook View in mutable.
    #[inline]
    pub fn get_workbook_view_mut(&mut self) -> &mut WorkbookView {
        &mut self.workbook_view
    }

    /// Set Workbook View.
    /// # Arguments
    /// * `value` - WorkbookView
    #[inline]
    pub fn set_workbook_view(&mut self, value: WorkbookView) -> &mut Self {
        self.workbook_view = value;
        self
    }

    /// (This method is crate only.)
    /// Has Defined Names.
    #[inline]
    pub(crate) fn has_defined_names(&self) -> bool {
        if !self.defined_names.is_empty() {
            return true;
        }
        self.get_sheet_collection_no_check()
            .iter()
            .any(|sheet| sheet.has_defined_names())
    }

    #[inline]
    pub(crate) fn get_backup_context_types(&self) -> &[(Box<str>, Box<str>)] {
        &self.backup_context_types
    }

    #[inline]
    pub(crate) fn set_backup_context_types(
        &mut self,
        value: impl Into<ThinVec<(String, String)>>,
    ) -> &mut Self {
        self.backup_context_types = value
            .into()
            .into_iter()
            .map(|(a, b)| (a.into_boxed_str(), b.into_boxed_str()))
            .collect();
        self
    }

    pub(crate) fn get_pivot_caches(&self) -> Vec<(String, String, String)> {
        let mut result: Vec<(String, String, String)> = Vec::new();
        for (val1, val2, val3) in &self.pivot_caches {
            let val3_up = format!("xl/{}", &val3);
            for worksheet in self.get_sheet_collection_no_check() {
                for pivot_cache_definition in worksheet.get_pivot_cache_definition_collection() {
                    if val3_up.as_str() == pivot_cache_definition
                        && !result.iter().any(|(_, _, r_val3)| r_val3 == &**val3)
                    {
                        result.push((val1.to_string(), val2.to_string(), val3.to_string()));
                    }
                }
            }
        }
        result
    }

    #[inline]
    pub(crate) fn add_pivot_caches(&mut self, value: (String, String, String)) -> &mut Self {
        self.pivot_caches.push((
            value.0.into_boxed_str(),
            value.1.into_boxed_str(),
            value.2.into_boxed_str(),
        ));
        self
    }

    #[inline]
    pub(crate) fn update_pivot_caches(&mut self, key: String, value: String) -> &mut Self {
        self.pivot_caches.iter_mut().for_each(|(val1, _, val3)| {
            if **val1 == key {
                *val3 = value.clone().into_boxed_str()
            };
        });
        self
    }

    #[inline]
    pub fn get_workbook_protection(&self) -> Option<&WorkbookProtection> {
        self.workbook_protection.as_deref()
    }

    #[inline]
    pub fn get_workbook_protection_mut(&mut self) -> &mut WorkbookProtection {
        self.workbook_protection
            .get_or_insert(Box::new(WorkbookProtection::default()))
    }

    #[inline]
    pub fn set_workbook_protection(&mut self, value: WorkbookProtection) -> &mut Self {
        self.workbook_protection = Some(Box::new(value));
        self
    }

    #[inline]
    pub fn remove_workbook_protection(&mut self) -> &mut Self {
        self.workbook_protection = None;
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
}
impl AdjustmentCoordinateWithSheet for Spreadsheet {
    fn adjustment_insert_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.read_sheet_collection();
        for worksheet in &mut self.work_sheet_collection {
            worksheet.adjustment_insert_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
            worksheet.adjustment_insert_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    fn adjustment_remove_coordinate_with_sheet(
        &mut self,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
    ) {
        self.read_sheet_collection();
        for worksheet in &mut self.work_sheet_collection {
            worksheet.adjustment_remove_coordinate(
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
            worksheet.adjustment_remove_coordinate_with_sheet(
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }
}
