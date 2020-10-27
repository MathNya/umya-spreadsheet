use super::worksheet::Worksheet;
use super::properties::Properties;
use super::security::Security;
use super::calculation::Calculation;
use super::style::Style;
use super::number_format::NumberFormat;
use super::font::Font;
use super::fill::Fill;
use super::defined_name::DefinedName;
use super::borders::Borders;
use super::theme::Theme;
use super::cell_style::CellStyle;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Spreadsheet {
    properties: Properties,
    security: Security,
    work_sheet_collection: Vec<Worksheet>,
    calculation_engine: Calculation,
    active_sheet_index: usize,
    named_ranges: Vec<String>,
    cell_style_collection: Vec<CellStyle>,
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
}
impl Spreadsheet {
    pub fn get_defined_names(&self) -> &Vec<DefinedName> {
        &self.defined_names
    }
    pub(crate) fn set_defined_names(&mut self, value:Vec<DefinedName>) {
        self.defined_names = value;
    }
    pub(crate) fn add_defined_names(&mut self, value:DefinedName) {
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
    pub(crate) fn get_properties_mut(&mut self) -> &mut Properties {
        &mut self.properties
    }
    pub(crate) fn set_properties(&mut self, value:Properties) {
        self.properties = value;
    }
    pub fn get_security(&self) -> &Security {
        &self.security
    }
    pub(crate) fn get_security_mut(&mut self) -> &mut Security {
        &mut self.security
    }
    pub(crate) fn set_security(&mut self, value:Security) {
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
    pub(crate) fn set_has_macros(&mut self, value:bool) {
        self.has_macros = value;
    }
    pub fn get_unparsed_loaded_data(&self) -> &Vec<String> {
        &self.unparsed_loaded_data
    }
    pub(crate) fn set_unparsed_loaded_data(&mut self, value:Vec<String>) {
        self.unparsed_loaded_data = value;
    }
    pub fn get_active_sheet_index(&self) -> &usize {
        &self.active_sheet_index
    }
    pub(crate) fn set_active_sheet_index(&mut self, value:usize) {
        self.active_sheet_index = value;
    }
    pub(crate) fn get_all_number_format(&self) -> Vec<(String, NumberFormat)> {
        let mut result:Vec<(String, NumberFormat)> = Vec::new();
        for (_, style) in &self.get_all_cell_style() {
            match style.get_number_format() {
                Some(v) => {
                    if v.get_built_in_format_code() != &None {
                        continue;
                    }
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
        result
    }
    pub(crate) fn get_all_font(&self) -> Vec<(String, Font)> {
        let mut result:Vec<(String, Font)> = Vec::new();
        let def = Font::get_defalut_value();
        result.push((def.get_hash_code(), def));
        for (_, style) in &self.get_all_cell_style() {
            match style.get_font() {
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
                }
                None => {}
            }
        }
        result
    }
    pub(crate) fn get_all_fill(&self) -> Vec<(String, Fill)> {
        let mut result:Vec<(String, Fill)> = Vec::new();
        let def = Fill::get_defalut_value();
        result.push((def.get_hash_code(), def));
        let def2 = Fill::get_defalut_value_2();
        result.push((def2.get_hash_code(), def2));
        for (_, style) in &self.get_all_cell_style() {
            match style.get_fill() {
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
        result
    }
    pub(crate) fn get_all_borders(&self) -> Vec<(String, Borders)> {
        let mut result:Vec<(String, Borders)> = Vec::new();
        let def = Borders::get_defalut_value();
        result.push((def.get_hash_code(), def));
        for (_, style) in &self.get_all_cell_style() {
            match style.get_borders() {
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
        result
    }
    pub(crate) fn get_all_cell_style(&self) -> Vec<(String, Style)> {
        let mut result:Vec<(String, Style)> = Vec::new();
        let def = Style::get_defalut_value();
        result.push((def.get_hash_code(), def));
        for worksheet in &self.work_sheet_collection {
            for style in worksheet.get_style_collection() {
                let mut is_match = false;
                for (hash, _) in &result {
                    if hash == &style.get_hash_code() {
                        is_match = true;
                        break;
                    }
                }
                if is_match == false {
                    result.push((style.get_hash_code(), style.clone()));
                }
            }
        }
        result
    }
    //pub fn get_cell_xf_collection(&self) -> &Vec<Style> {
    //    &self.cell_xf_collection
    //}
    //pub fn get_cell_xf_by_index(&self, index:usize) -> &Style {
    //    &self.cell_xf_collection.get(index).unwrap()
    //}
    //pub fn add_cell_xf_collection(&mut self, value:Style) {
    //    self.cell_xf_collection.push(value);
    //}
    //pub(crate) fn set_cell_xf_collection(&mut self, value:Vec<Style>) {
    //    self.cell_xf_collection = value;
    //}
    pub fn get_cell_style_collection(&self) -> &Vec<CellStyle> {
        &self.cell_style_collection
    }
    pub(crate) fn get_cell_style_collection_mut(&mut self) -> &mut Vec<CellStyle> {
        &mut self.cell_style_collection
    }
    pub(crate) fn add_cell_style_collection(&mut self, value:CellStyle) {
        self.cell_style_collection.push(value);
    }
    pub(crate) fn set_cell_style_collection(&mut self, value:Vec<CellStyle>) {
        self.cell_style_collection = value;
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
    pub fn get_sheet_by_sheet_id<S: Into<String>>(&self, value:S) -> Result<&Worksheet, &'static str> {
        let v = value.into();
        for sheet in &self.work_sheet_collection {
            if sheet.get_sheet_id() == &v {
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
    pub(crate) fn has_formula_attributes(&self) -> bool {
        for worksheet in &self.work_sheet_collection {
            for cell in worksheet.get_cell_collection() {
                if cell.get_formula_attributes() != "" {
                    return true;
                }
            }
        }
        false
    }
}
