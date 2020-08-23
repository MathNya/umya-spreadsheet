use super::style::Style;
use super::color::Color;

#[derive(Clone, Debug)]
pub struct Conditional {
    condition_type: String,
    operator_type : String,
    data_type: String,
    text: String,
    priority: usize,
    percent: usize,
    bottom: usize,
    rank: usize,
    stop_if_true : bool,
    condition: Vec<String>,
    cfvo_collection: Vec<(String, Option<String>, Option<Color>)>,
    style: Option<Style>,
}
impl Default for Conditional {
    fn default() -> Self {
        Self {
            condition_type: Conditional::CONDITION_NONE.to_string(),
            operator_type : Conditional::OPERATOR_NONE.to_string(),
            data_type: "".into(),
            text: "".into(),
            priority: 0,
            percent: 0,
            bottom: 0,
            rank: 0,
            stop_if_true : false,
            condition: Vec::new(),
            cfvo_collection: Vec::new(),
            style: None
        }
    }
}
impl Conditional {
    // Condition types
    pub const CONDITION_NONE: &'static str = "none";
    pub const CONDITION_CELLIS: &'static str = "cellIs";
    pub const CONDITION_CONTAINSTEXT: &'static str = "containsText";
    pub const CONDITION_EXPRESSION: &'static str = "expression";
    pub const CONDITION_CONTAINSBLANKS: &'static str = "containsBlanks";
    pub const CONDITION_NOTCONTAINSBLANKS: &'static str = "notContainsBlanks";
    
    // Operator types
    pub const OPERATOR_NONE: &'static str = "";
    pub const OPERATOR_BEGINSWITH: &'static str = "beginsWith";
    pub const OPERATOR_ENDSWITH: &'static str = "endsWith";
    pub const OPERATOR_EQUAL: &'static str = "equal";
    pub const OPERATOR_GREATERTHAN: &'static str = "greaterThan";
    pub const OPERATOR_GREATERTHANOREQUAL: &'static str = "greaterThanOrEqual";
    pub const OPERATOR_LESSTHAN: &'static str = "lessThan";
    pub const OPERATOR_LESSTHANOREQUAL: &'static str = "lessThanOrEqual";
    pub const OPERATOR_NOTEQUAL: &'static str = "notEqual";
    pub const OPERATOR_CONTAINSTEXT: &'static str = "containsText";
    pub const OPERATOR_NOTCONTAINS: &'static str = "notContains";
    pub const OPERATOR_BETWEEN: &'static str = "between";
    pub const OPERATOR_NOTBETWEEN: &'static str = "notBetween";

    pub fn get_condition_type(&self)-> &str {
        &self.condition_type
    }
    pub(crate) fn set_condition_type<S: Into<String>>(&mut self, value:S) {
        self.condition_type = value.into();
    }
    pub fn get_operator_type(&self)-> &str {
        &self.operator_type
    }
    pub(crate) fn set_operator_type<S: Into<String>>(&mut self, value:S) {
        self.operator_type = value.into();
    }
    pub fn get_data_type(&self)-> &str {
        &self.data_type
    }
    pub(crate) fn set_data_type<S: Into<String>>(&mut self, value:S) {
        self.data_type = value.into();
    }
    pub fn get_text(&self)-> &str {
        &self.text
    }
    pub(crate) fn set_text<S: Into<String>>(&mut self, value:S) {
        self.text = value.into();
    }
    pub fn get_priority(&self)-> &usize {
        &self.priority
    }
    pub(crate) fn set_priority(&mut self, value:usize) {
        self.priority = value;
    }
    pub fn get_percent(&self)-> &usize {
        &self.percent
    }
    pub(crate) fn set_percent(&mut self, value:usize) {
        self.percent = value;
    }
    pub fn get_bottom(&self)-> &usize {
        &self.bottom
    }
    pub(crate) fn set_bottom(&mut self, value:usize) {
        self.bottom = value;
    }
    pub fn get_rank(&self)-> &usize {
        &self.rank
    }
    pub(crate) fn set_rank(&mut self, value:usize) {
        self.rank = value;
    }
    pub fn get_stop_if_true(&self)-> &bool {
        &self.stop_if_true
    }
    pub(crate) fn set_stop_if_true(&mut self, value:bool) {
        self.stop_if_true = value;
    }
    pub fn get_condition(&self)-> &Vec<String> {
        &self.condition
    }
    pub(crate) fn set_condition(&mut self, value:Vec<String>) {
        self.condition = value;
    }
    pub(crate) fn add_condition<S: Into<String>>(&mut self, value:S) {
        self.condition.push(value.into());
    }
    pub fn get_cfvo_collection(&self)-> &Vec<(String, Option<String>, Option<Color>)> {
        &self.cfvo_collection
    }
    pub(crate) fn set_cfvo_collection(&mut self, value:Vec<(String, Option<String>, Option<Color>)>) {
        self.cfvo_collection = value;
    }
    pub(crate) fn add_cfvo_collection<S: Into<String>>(&mut self, r#type:S, value:Option<String>, color:Option<Color>) {
        self.cfvo_collection.push((r#type.into(), value, color));
    }
    pub fn get_style(&self)-> &Option<Style> {
        &self.style
    }
    pub(crate) fn set_style(&mut self, value:Style) {
        self.style = Some(value);
    }
}
