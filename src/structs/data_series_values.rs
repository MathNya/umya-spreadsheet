use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct DataSeriesValues {
    data_type: String,
    data_source: String,
    format_code: String,
    point_marker: Option<String>,
    point_count: i32,
    data_values: BTreeMap<i32, String>,
    fill_color: Vec<String>,
    line_width: i32,
}
impl DataSeriesValues {

    pub const DATASERIES_TYPE_STRING: &'static str = "String";
    pub const DATASERIES_TYPE_NUMBER: &'static str = "Number";

    pub fn get_data_type(&self)-> &str {
        &self.data_type
    }
    pub(crate) fn set_data_type<S: Into<String>>(&mut self, value:S) {
        self.data_type = value.into();
    }
    pub fn get_data_source(&self)-> &str {
        &self.data_source
    }
    pub(crate) fn set_data_source<S: Into<String>>(&mut self, value:S) {
        self.data_source = value.into();
    }
    pub fn get_format_code(&self)-> &str {
        &self.format_code
    }
    pub(crate) fn set_format_code<S: Into<String>>(&mut self, value:S) {
        self.format_code = value.into();
    }
    pub fn get_point_marker(&self)-> &Option<String> {
        &self.point_marker
    }
    pub(crate) fn set_point_marker<S: Into<String>>(&mut self, value:S) {
        self.point_marker = Some(value.into());
    }
    pub fn get_point_count(&self)-> &i32 {
        &self.point_count
    }
    pub(crate) fn set_point_count(&mut self, value:i32) {
        self.point_count = value;
    }
    pub fn get_data_values(&self)-> &BTreeMap<i32, String> {
        &self.data_values
    }
    pub(crate) fn add_data_values<S: Into<String>>(&mut self, index:i32, value:S) {
        self.data_values.insert(index, value.into());
    }
    pub fn get_fill_color(&self)-> &Vec<String> {
        &self.fill_color
    }
    pub(crate) fn add_fill_color<S: Into<String>>(&mut self, value:S) {
        self.fill_color.push(value.into());
    }
    pub(crate) fn set_line_width(&mut self, value:i32) {
        self.line_width = value;
    }
    pub fn get_line_width(&self)-> &i32 {
        &self.line_width
    }
}