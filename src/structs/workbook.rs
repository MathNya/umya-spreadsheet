use std::sync::{
    Arc,
    RwLock,
};

use crate::{
    StringValue,
    helper::{
        address::split_address,
        coordinate::column_index_from_string,
    },
    reader::xlsx::raw_to_deserialize_by_worksheet,
    structs::{
        Address,
        CellValue,
        Cells,
        DefinedName,
        Properties,
        SharedStringTable,
        Stylesheet,
        WorkbookProtection,
        WorkbookView,
        Worksheet,
        drawing::Theme,
    },
    traits::{
        AdjustmentCoordinate,
        AdjustmentCoordinateWithSheet,
    },
};

/// A Workbook Object.
/// The starting point of all struct.
#[derive(Clone, Default, Debug)]
pub struct Workbook {
    properties:            Properties,
    work_sheet_collection: Vec<Worksheet>,
    macros_code:           Option<Vec<u8>>,
    jsa_macros_code:       Option<Vec<u8>>,
    code_name:             StringValue,
    ribbon_xml_data:       StringValue,
    theme:                 Theme,
    stylesheet:            Stylesheet,
    shared_string_table:   Arc<RwLock<SharedStringTable>>,
    workbook_view:         WorkbookView,
    backup_context_types:  Vec<(Box<str>, Box<str>)>,
    pivot_caches:          Vec<(Box<str>, Box<str>, Box<str>)>,
    workbook_protection:   Option<Box<WorkbookProtection>>,
    defined_names:         Vec<DefinedName>,
}

impl Workbook {
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
        self.insert_new_column_by_index(sheet_name, column_index_from_string(column), num_columns);
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
        self.remove_column_by_index(sheet_name, column_index_from_string(column), num_columns);
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
    /// *`Vec<&CellValue>` - `CellValue` List.
    /// # Examples
    /// ```
    /// let mut book = umya_spreadsheet::new_file();
    /// let mut cell_value_List = book.get_cell_value_by_address("Sheet1!A1:C5");
    /// ```
    #[inline]
    #[must_use]
    pub fn cell_value_by_address(&self, address: &str) -> Vec<&CellValue> {
        let (sheet_name, range) = split_address(address);
        self.sheet_by_name(sheet_name)
            .unwrap()
            .cell_value_by_range(range)
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use cell_value_by_address()")]
    pub fn get_cell_value_by_address(&self, address: &str) -> Vec<&CellValue> {
        self.cell_value_by_address(address)
    }

    /// (This method is crate only.)
    /// Gets the cell value by specifying an Address Object.
    /// # Arguments
    /// * `address` - Address Object
    /// # Return value
    /// *`Vec<&CellValue>` - `CellValue` List.
    #[inline]
    pub(crate) fn cell_value_by_address_crate(&self, address: &Address) -> Vec<&CellValue> {
        self.sheet_by_name(address.sheet_name())
            .unwrap()
            .cell_value_by_range(&address.range().range())
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use cell_value_by_address_crate()")]
    pub(crate) fn get_cell_value_by_address_crate(&self, address: &Address) -> Vec<&CellValue> {
        self.cell_value_by_address_crate(address)
    }

    /// Get Theme.
    #[inline]
    #[must_use]
    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use theme()")]
    pub fn get_theme(&self) -> &Theme {
        self.theme()
    }

    /// Get Theme in mutable.
    #[inline]
    pub fn theme_mut(&mut self) -> &mut Theme {
        &mut self.theme
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use theme_mut()")]
    pub fn get_theme_mut(&mut self) -> &mut Theme {
        self.theme_mut()
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
    #[must_use]
    pub fn properties(&self) -> &Properties {
        &self.properties
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use properties()")]
    pub fn get_properties(&self) -> &Properties {
        self.properties()
    }

    /// Get Properties in mutable.
    #[inline]
    pub fn properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use properties_mut()")]
    pub fn get_properties_mut(&mut self) -> &mut Properties {
        self.properties_mut()
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
    #[must_use]
    pub fn macros_code(&self) -> Option<&[u8]> {
        self.macros_code.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use macros_code()")]
    pub fn get_macros_code(&self) -> Option<&[u8]> {
        self.macros_code()
    }

    /// Set Macros Code.
    /// # Arguments
    /// * `value` - Macros Code Raw Data.
    #[inline]
    pub fn set_macros_code(&mut self, value: impl Into<Vec<u8>>) -> &mut Self {
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
    #[must_use]
    pub fn has_macros(&self) -> bool {
        self.macros_code.is_some()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use has_macros()")]
    pub fn get_has_macros(&self) -> bool {
        self.has_macros()
    }

    /// Get Macros Code.
    /// # Return value
    /// * `Option<&Vec<u8>>` - Macros Code Raw Data.
    #[inline]
    #[must_use]
    pub fn jsa_macros_code(&self) -> Option<&[u8]> {
        self.jsa_macros_code.as_deref()
    }

    /// Get Macros Code.
    /// # Return value
    /// * `Option<&Vec<u8>>` - Macros Code Raw Data.
    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use jsa_macros_code()")]
    pub fn get_jsa_macros_code(&self) -> Option<&[u8]> {
        self.jsa_macros_code()
    }

    /// Set Macros Code.
    /// # Arguments
    /// * `value` - Macros Code Raw Data.
    #[inline]
    pub fn set_jsa_macros_code(&mut self, value: impl Into<Vec<u8>>) -> &mut Self {
        self.jsa_macros_code = Some(value.into());
        self
    }

    /// Remove Macros Code
    #[inline]
    pub fn remove_jsa_macros_code(&mut self) -> &mut Self {
        self.jsa_macros_code = None;
        self
    }

    /// Has Macros Code
    #[inline]
    #[must_use]
    pub fn has_jsa_macros(&self) -> bool {
        self.jsa_macros_code.is_some()
    }

    /// Has Macros Code
    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use has_jsa_macros()")]
    pub fn get_has_jsa_macros(&self) -> bool {
        self.has_jsa_macros()
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
    /// for that code in Workbook object to work out of the box without
    /// adjustments
    #[inline]
    #[must_use]
    pub fn code_name(&self) -> Option<&str> {
        self.code_name.value()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use code_name()")]
    pub fn get_code_name(&self) -> Option<&str> {
        self.code_name()
    }

    /// (This method is crate only.)
    /// Get Stylesheet.
    #[inline]
    pub(crate) fn stylesheet(&self) -> &Stylesheet {
        &self.stylesheet
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use stylesheet()")]
    pub(crate) fn get_stylesheet(&self) -> &Stylesheet {
        self.stylesheet()
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
    pub(crate) fn set_stylesheet_default_value(&mut self) -> &mut Self {
        self.stylesheet.set_defalut_value();
        self
    }

    /// (This method is crate only.)
    /// Get Shared String Table.
    #[inline]
    pub(crate) fn shared_string_table(&self) -> Arc<RwLock<SharedStringTable>> {
        self.shared_string_table.clone()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use shared_string_table()")]
    pub(crate) fn get_shared_string_table(&self) -> Arc<RwLock<SharedStringTable>> {
        self.shared_string_table()
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
    #[must_use]
    pub fn sheet_collection(&self) -> &[Worksheet] {
        for worksheet in &self.work_sheet_collection {
            assert!(
                worksheet.is_deserialized(),
                "This Worksheet is Not Deserialized. Please exec to read_sheet(&mut self, index: \
                 usize);"
            );
        }
        &self.work_sheet_collection
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use sheet_collection()")]
    pub fn get_sheet_collection(&self) -> &[Worksheet] {
        self.sheet_collection()
    }

    /// Get Work Sheet List.
    /// No check deserialized.
    #[inline]
    #[must_use]
    pub fn sheet_collection_no_check(&self) -> &[Worksheet] {
        &self.work_sheet_collection
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use sheet_collection_no_check()")]
    pub fn get_sheet_collection_no_check(&self) -> &[Worksheet] {
        self.sheet_collection_no_check()
    }

    /// Get Work Sheet List in mutable.
    #[inline]
    pub fn sheet_collection_mut(&mut self) -> &mut [Worksheet] {
        self.read_sheet_collection();
        &mut self.work_sheet_collection
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use sheet_collection_mut()")]
    pub fn get_sheet_collection_mut(&mut self) -> &mut [Worksheet] {
        self.sheet_collection_mut()
    }

    /// Get Work Sheet Count.
    /// # Return value
    /// * `usize` - Work Sheet Count.
    #[inline]
    #[must_use]
    pub fn sheet_count(&self) -> usize {
        self.work_sheet_collection.len()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use sheet_count()")]
    pub fn get_sheet_count(&self) -> usize {
        self.sheet_count()
    }

    /// deserialize by all worksheet.
    #[inline]
    pub fn read_sheet_collection(&mut self) -> &mut Self {
        let shared_string_table = self.shared_string_table();
        let stylesheet = self.stylesheet().clone();
        for worksheet in &mut self.work_sheet_collection {
            raw_to_deserialize_by_worksheet(worksheet, &shared_string_table, &stylesheet);
        }
        self
    }

    /// deserialize a worksheet.
    #[inline]
    pub fn read_sheet(&mut self, index: usize) -> &mut Self {
        let shared_string_table = self.shared_string_table();
        let stylesheet = self.stylesheet().clone();
        let worksheet = self.work_sheet_collection.get_mut(index).unwrap();
        raw_to_deserialize_by_worksheet(worksheet, &shared_string_table, &stylesheet);
        self
    }

    /// deserialize a worksheet.
    #[inline]
    pub fn read_sheet_by_name(&mut self, sheet_name: &str) -> &mut Self {
        let index = self.find_sheet_index_by_name(sheet_name).unwrap();
        self.read_sheet(index)
    }

    #[inline]
    pub(crate) fn find_sheet_index_by_name(&self, sheet_name: &str) -> Result<usize, &'static str> {
        self.work_sheet_collection
            .iter()
            .position(|sheet| sheet.name() == sheet_name)
            .ok_or("Not found.")
    }

    /// Get Work Sheet.
    /// # Arguments
    /// * `index` - sheet index
    /// # Return value
    /// * `Result<&Worksheet, &'static str>`.
    #[inline]
    pub fn sheet(&self, index: usize) -> Result<&Worksheet, &'static str> {
        match self.work_sheet_collection.get(index) {
            Some(v) => {
                if v.is_deserialized() {
                    Ok(v)
                 } else {
                    Err("This Worksheet is Not Deserialized. Please exec to read_sheet(&mut self, index: usize")
                 }
            },
            None => {
                Err("Not found.")
            }
        }
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use sheet()")]
    pub fn get_sheet(&self, index: &usize) -> Result<&Worksheet, &'static str> {
        self.sheet(*index)
    }

    /// Get Work Sheet.
    /// # Arguments
    /// * `sheet_name` - sheet name
    /// # Return value
    /// * `Result<&Worksheet, &'static str>`.
    #[inline]
    pub fn sheet_by_name(&self, sheet_name: &str) -> Result<&Worksheet, &'static str> {
        self.find_sheet_index_by_name(sheet_name)
            .and_then(|index| self.sheet(index))
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use sheet_by_name()")]
    pub fn get_sheet_by_name(&self, sheet_name: &str) -> Result<&Worksheet, &'static str> {
        self.sheet_by_name(sheet_name)
    }

    pub fn lazy_read_sheet_cells(&self, index: usize) -> Result<Cells, &'static str> {
        let shared_string_table = self.shared_string_table();
        self.work_sheet_collection
            .get(index)
            .map(|v| {
                v.cells_stream(&shared_string_table.read().unwrap(), self.stylesheet())
            })
            .ok_or("Not found.")
    }

    #[deprecated(since = "3.0.0", note = "Use lazy_read_sheet_cells()")]
    pub fn get_lazy_read_sheet_cells(&self, index: &usize) -> Result<Cells, &'static str> {
        self.lazy_read_sheet_cells(*index)
    }

    /// Get Work Sheet in mutable.
    /// # Arguments
    /// * `index` - sheet index
    /// # Return value
    /// * `Result<&mut Worksheet, &'static str>`.
    #[allow(clippy::manual_inspect)]
    pub fn sheet_mut(&mut self, index: usize) -> Result<&mut Worksheet, &'static str> {
        let shared_string_table = self.shared_string_table();
        let stylesheet = self.stylesheet().clone();
        self.work_sheet_collection.get_mut(index).map(|v| {
            raw_to_deserialize_by_worksheet(v, &shared_string_table, &stylesheet);
            v
        })
        .ok_or("Not found.")
    }

    #[deprecated(since = "3.0.0", note = "Use sheet_mut()")]
    pub fn get_sheet_mut(&mut self, index: &usize) -> Result<&mut Worksheet, &'static str> {
        self.sheet_mut(*index)
    }

    /// Get Work Sheet in mutable.
    /// # Arguments
    /// * `sheet_name` - sheet name
    /// # Return value
    /// * `Result<&mut Worksheet, &'static str>`.
    #[inline]
    pub fn sheet_by_name_mut(&mut self, sheet_name: &str) -> Result<&mut Worksheet, &'static str> {
        self.find_sheet_index_by_name(sheet_name)
            .and_then(move |index| self.sheet_mut(index))
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use sheet_by_name_mut()")]
    pub fn get_sheet_by_name_mut(&mut self, sheet_name: &str) -> Result<&mut Worksheet, &'static str> {
        self.sheet_by_name_mut(sheet_name)
    }

    #[inline]
    pub fn set_active_sheet(&mut self, index: u32) -> &mut Self {
        self.workbook_view_mut().set_active_tab(index);
        self
    }

    /// Get Active Work Sheet.
    /// # Return value
    /// * `&Worksheet` - Work sheet.
    #[inline]
    #[must_use]
    pub fn active_sheet(&self) -> &Worksheet {
        let index = self.workbook_view().get_active_tab();
        self.sheet(index as usize).unwrap()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use active_sheet()")]
    pub fn get_active_sheet(&self) -> &Worksheet {
        self.active_sheet()
    }

    /// Get Active Work Sheet in mutable.
    /// # Return value
    /// * `&mut Worksheet` - Work sheet.
    #[inline]
    pub fn active_sheet_mut(&mut self) -> &mut Worksheet {
        let index = self.workbook_view().get_active_tab();
        self.sheet_mut(index as usize).unwrap()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use active_sheet_mut()")]
    pub fn get_active_sheet_mut(&mut self) -> &mut Worksheet {
        self.active_sheet_mut()
    }

    /// Add Work Sheet.
    /// # Arguments
    /// * `value` - Work Sheet
    /// # Return value
    /// * `Result<&mut Worksheet, &'static str>` - OK:added work sheet.
    ///   Err:Error.
    #[inline]
    pub fn add_sheet(&mut self, value: Worksheet) -> Result<&mut Worksheet, &'static str> {
        let title = value.name();
        Workbook::check_sheet_name(self, title)?;
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
            .retain(|x| x.name() != sheet_name);
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
    /// * `Result<&mut Worksheet, &'static str>` - OK:added work sheet.
    ///   Err:Error.
    #[inline]
    pub fn new_sheet<S: Into<String>>(
        &mut self,
        sheet_title: S,
    ) -> Result<&mut Worksheet, &'static str> {
        let v = sheet_title.into();
        Workbook::check_sheet_name(self, &v)?;
        let sheet_id = (self.work_sheet_collection.len() + 1).to_string();
        Ok(Workbook::add_new_sheet_crate(self, sheet_id, v.to_string()))
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
            .sheet_format_properties_mut()
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
        Workbook::check_sheet_name(self, sheet_name_str.as_ref())?;
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
        if self
            .work_sheet_collection
            .iter()
            .any(|work_sheet| value == work_sheet.name())
        {
            Err("name duplicate.")
        } else {
            Ok(())
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
    #[must_use]
    pub fn workbook_view(&self) -> &WorkbookView {
        &self.workbook_view
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use workbook_view()")]
    pub fn get_workbook_view(&self) -> &WorkbookView {
        self.workbook_view()
    }

    /// Get Workbook View in mutable.
    #[inline]
    pub fn workbook_view_mut(&mut self) -> &mut WorkbookView {
        &mut self.workbook_view
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use workbook_view_mut()")]
    pub fn get_workbook_view_mut(&mut self) -> &mut WorkbookView {
        self.workbook_view_mut()
    }

    /// Set Workbook View.
    /// # Arguments
    /// * `value` - `WorkbookView`
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
        self.sheet_collection_no_check()
            .iter()
            .any(Worksheet::has_defined_names)
    }

    #[inline]
    pub(crate) fn backup_context_types(&self) -> &[(Box<str>, Box<str>)] {
        &self.backup_context_types
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use backup_context_types()")]
    pub(crate) fn get_backup_context_types(&self) -> &[(Box<str>, Box<str>)] {
        self.backup_context_types()
    }

    #[inline]
    pub(crate) fn set_backup_context_types(
        &mut self,
        value: impl Into<Vec<(String, String)>>,
    ) -> &mut Self {
        self.backup_context_types = value
            .into()
            .into_iter()
            .map(|(a, b)| (a.into_boxed_str(), b.into_boxed_str()))
            .collect();
        self
    }

    pub(crate) fn pivot_caches(&self) -> Vec<(String, String, String)> {
        let mut result: Vec<(String, String, String)> = Vec::new();
        for (val1, val2, val3) in &self.pivot_caches {
            let val3_up = format!("xl/{}", &val3);
            for worksheet in self.sheet_collection_no_check() {
                for pivot_cache_definition in worksheet.pivot_cache_definition_collection() {
                    if val3_up == pivot_cache_definition
                        && !result.iter().any(|(_, _, r_val3)| r_val3 == &**val3)
                    {
                        result.push((val1.to_string(), val2.to_string(), val3.to_string()));
                    }
                }
            }
        }
        result
    }

    #[deprecated(since = "3.0.0", note = "Use pivot_caches()")]
    pub(crate) fn get_pivot_caches(&self) -> Vec<(String, String, String)> {
        self.pivot_caches()
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
    pub(crate) fn update_pivot_caches(&mut self, key: &str, value: &str) -> &mut Self {
        self.pivot_caches.iter_mut().for_each(|(val1, _, val3)| {
            if &**val1 == key {
                *val3 = value.to_owned().into_boxed_str();
            }
        });
        self
    }

    #[inline]
    #[must_use]
    pub fn workbook_protection(&self) -> Option<&WorkbookProtection> {
        self.workbook_protection.as_deref()
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use workbook_protection()")]
    pub fn get_workbook_protection(&self) -> Option<&WorkbookProtection> {
        self.workbook_protection()
    }

    #[inline]
    pub fn workbook_protection_mut(&mut self) -> &mut WorkbookProtection {
        self.workbook_protection
            .get_or_insert(Box::new(WorkbookProtection::default()))
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use workbook_protection_mut()")]
    pub fn get_workbook_protection_mut(&mut self) -> &mut WorkbookProtection {
        self.workbook_protection_mut()
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
}
impl AdjustmentCoordinateWithSheet for Workbook {
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
