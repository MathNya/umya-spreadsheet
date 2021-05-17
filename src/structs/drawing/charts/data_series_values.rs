use std::collections::BTreeMap;
use super::super::super::address::Address;

#[derive(Default, Debug)]
pub struct DataSeriesValues {
    data_type: String,
    address: Address,
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

    pub fn set_data_type<S: Into<String>>(&mut self, value:S)-> &mut DataSeriesValues {
        self.data_type = value.into();
        self
    }

    pub fn get_address(&self)-> &Address {
        &self.address
    }

    pub fn get_address_mut(&mut self)-> &mut Address {
        &mut self.address
    }

    pub fn set_address<S: Into<String>>(&mut self, value:S)-> &mut DataSeriesValues {
        let mut address = Address::default();
        address.set_address(value);
        self.address = address;
        self
    }

    pub fn get_format_code(&self)-> &str {
        &self.format_code
    }

    pub fn set_format_code<S: Into<String>>(&mut self, value:S)-> &mut DataSeriesValues {
        self.format_code = value.into();
        self
    }

    pub fn get_point_marker(&self)-> &Option<String> {
        &self.point_marker
    }

    pub fn set_point_marker<S: Into<String>>(&mut self, value:S)-> &mut DataSeriesValues {
        self.point_marker = Some(value.into());
        self
    }

    pub fn get_point_count(&self)-> &i32 {
        &self.point_count
    }

    pub fn set_point_count(&mut self, value:i32)-> &mut DataSeriesValues {
        self.point_count = value;
        self
    }

    pub fn get_data_values(&self)-> &BTreeMap<i32, String> {
        &self.data_values
    }

    pub fn add_data_values<S: Into<String>>(&mut self, index:i32, value:S)-> &mut DataSeriesValues {
        self.data_values.insert(index, value.into());
        self
    }

    pub fn get_fill_color(&self)-> &Vec<String> {
        &self.fill_color
    }

    pub fn add_fill_color<S: Into<String>>(&mut self, value:S)-> &mut DataSeriesValues {
        self.fill_color.push(value.into());
        self
    }

    pub fn set_line_width(&mut self, value:i32)-> &mut DataSeriesValues {
        self.line_width = value;
        self
    }

    pub fn get_line_width(&self)-> &i32 {
        &self.line_width
    }
}