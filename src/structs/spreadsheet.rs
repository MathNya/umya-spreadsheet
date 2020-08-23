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
    cell_xf_collection: Vec<Style>,
    has_macros: bool,
    macros_code: String,
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
    defined_names: HashMap<String, DefinedName>,
}
impl Spreadsheet {
    pub fn get_defined_names(&self) -> &HashMap<String, DefinedName> {
        &self.defined_names
    }
    pub(crate) fn set_defined_names(&mut self, value:HashMap<String, DefinedName>) {
        self.defined_names = value;
    }
    pub(crate) fn add_defined_names(&mut self, value:DefinedName) {
        self.defined_names.insert(value.get_name().to_string(), value);
    }
    pub(crate) fn get_all_conditional_style_list(&self) -> Vec<(String, Style)> {
        let mut result:Vec<(String, Style)> = Vec::new();
        for work_sheet in &self.work_sheet_collection {
            for (_, conditional_formatting) in work_sheet.get_conditional_styles_collection() {
                for conditional in conditional_formatting {
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
        for style in &self.cell_xf_collection {
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
        for style in &self.cell_xf_collection {
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
        for style in &self.cell_xf_collection {
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
        for style in &self.cell_xf_collection {
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
    pub(crate) fn get_all_cell_style(&self, theme_color_map:&Vec<String>) -> Vec<(String, Style)> {
        let mut result:Vec<(String, Style)> = Vec::new();
        for style in &self.cell_xf_collection {
            if style.get_xf_id() != &0 {
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
    pub fn get_cell_xf_collection(&self) -> &Vec<Style> {
        &self.cell_xf_collection
    }
    pub fn get_cell_xf_by_index(&self, index:usize) -> &Style {
        &self.cell_xf_collection.get(index).unwrap()
    }
    pub fn add_cell_xf_collection(&mut self, value:Style) {
        self.cell_xf_collection.push(value);
    }
    pub(crate) fn set_cell_xf_collection(&mut self, value:Vec<Style>) {
        self.cell_xf_collection = value;
    }
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
    pub fn get_sheet(&self, index:usize) -> &Worksheet {
        &self.work_sheet_collection.get(index).unwrap()
    }
    pub fn get_sheet_mut(&mut self, index:usize) -> &mut Worksheet {
        self.work_sheet_collection.get_mut(index).unwrap()
    }
    pub(crate) fn new_sheet(&mut self) -> &mut Worksheet {
        let mut worksheet = Worksheet::default();
        self.work_sheet_collection.push(worksheet);
        self.work_sheet_collection.last_mut().unwrap()
    }
    pub fn has_ribbon(&self) -> bool {
        self.ribbon_xml_data.is_some()
    }
    pub(crate) fn add_sheet(&mut self, worksheet:Worksheet) {
       self.work_sheet_collection.push(worksheet);
    }
    pub(crate) fn has_formula_attributes(&self) -> bool {
        for worksheet in &self.work_sheet_collection {
            for (_, cell) in worksheet.get_cell_collection().get_collection() {
                if cell.get_formula_attributes() != "" {
                    return true;
                }
            }
        }
        false
    }
}
