use std::collections::HashMap; 

#[derive(Default, Debug)]
pub struct Axis {
    id: usize,
    axis_number: HashMap<String, String>,
    axis_options: HashMap<String, String>,
    fill_properties: HashMap<String, String>,
    line_properties: HashMap<String, String>,
    line_style_properties: HashMap<String, String>,
    shadow_properties: HashMap<String, String>,
    glow_properties: HashMap<String, String>,
    soft_edges: HashMap<String, String>,
}
impl Axis {
    pub fn get_id(&self)->&usize {
        &self.id
    }
    pub(crate) fn set_id(&mut self, value:usize) {
        self.id = value;
    }
    pub fn get_axis_number(&self)-> &HashMap<String, String> {
        &self.axis_number
    }
    pub(crate) fn set_axis_number<S: Into<String>>(&mut self, key:S, value:S) {
        self.axis_number.insert(key.into(), value.into());
    }
    pub fn get_axis_options(&self)-> &HashMap<String, String> {
        &self.axis_options
    }
    pub(crate) fn set_axis_options<S: Into<String>>(&mut self, key:S, value:S) {
        self.axis_options.insert(key.into(), value.into());
    }
    pub fn get_fill_properties(&self)-> &HashMap<String, String> {
        &self.fill_properties
    }
    pub(crate) fn set_fill_properties<S: Into<String>>(&mut self, key:S, value:S) {
        self.fill_properties.insert(key.into(), value.into());
    }
    pub fn get_line_properties(&self)-> &HashMap<String, String> {
        &self.line_properties
    }
    pub(crate) fn set_line_properties<S: Into<String>>(&mut self, key:S, value:S) {
        self.line_properties.insert(key.into(), value.into());
    }
    pub fn get_line_style_properties(&self)-> &HashMap<String, String> {
        &self.line_style_properties
    }
    pub(crate) fn set_line_style_properties<S: Into<String>>(&mut self, key:S, value:S) {
        self.line_style_properties.insert(key.into(), value.into());
    }
    pub fn get_shadow_properties(&self)-> &HashMap<String, String> {
        &self.shadow_properties
    }
    pub(crate) fn set_shadow_properties<S: Into<String>>(&mut self, key:S, value:S) {
        self.shadow_properties.insert(key.into(), value.into());
    }
    pub fn get_glow_properties(&self)-> &HashMap<String, String> {
        &self.glow_properties
    }
    pub(crate) fn set_glow_properties<S: Into<String>>(&mut self, key:S, value:S) {
        self.glow_properties.insert(key.into(), value.into());
    }
    pub fn get_soft_edges(&self)-> &HashMap<String, String> {
        &self.soft_edges
    }
    pub(crate) fn set_soft_edges<S: Into<String>>(&mut self, key:S, value:S) {
        self.soft_edges.insert(key.into(), value.into());
    }
}
