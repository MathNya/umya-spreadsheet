
use super::run_properties::RunProperties;

#[derive(Default, Debug)]
pub struct TextBodyContent {
    algn: String,
    run_properties: Vec<RunProperties>,
    end_para_run_properties: Option<RunProperties>,
}
impl TextBodyContent {
    pub fn get_algn(&self) -> &str {
        &self.algn
    }

    pub fn set_algn<S: Into<String>>(&mut self, value:S) {
        self.algn = value.into();
    }

    pub fn get_run_properties(&self) -> &Vec<RunProperties> {
        &self.run_properties
    }

    pub fn add_run_properties(&mut self, value:RunProperties) {
        self.run_properties.push(value);
    }

    pub fn get_end_para_run_properties(&self) -> &Option<RunProperties> {
        &self.end_para_run_properties
    }

    pub fn set_end_para_run_properties(&mut self, value:RunProperties) {
        self.end_para_run_properties = Some(value);
    }

    pub fn remove_end_para_run_properties(&mut self) {
        self.end_para_run_properties = None;
    }
}