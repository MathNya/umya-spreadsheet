#[derive(Default, Debug)]
pub struct RowDimension {
    height: f32,
    descent: f32,
    spans: String,
    thick_bot: bool,
    custom_height: bool
}
impl RowDimension {
    pub fn get_height(&self)-> &f32 {
        &self.height
    }
    pub(crate) fn set_height(&mut self, value:f32) {
        self.height = value;
    }
    pub fn get_descent(&self)-> &f32 {
        &self.descent
    }
    pub(crate) fn set_descent(&mut self, value:f32) {
        self.descent = value;
    }
    pub fn get_spans(&self)-> &str {
        &self.spans
    }
    pub(crate) fn set_spans<S: Into<String>>(&mut self, value:S) {
        self.spans = value.into();
    }
    pub fn get_thick_bot(&self)-> &bool {
        &self.thick_bot
    }
    pub(crate) fn set_thick_bot(&mut self, value:bool) {
        self.thick_bot = value;
    }
    pub fn get_custom_height(&self)-> &bool {
        &self.custom_height
    }
    pub(crate) fn set_custom_height(&mut self, value:bool) {
        self.custom_height = value;
    }
}
