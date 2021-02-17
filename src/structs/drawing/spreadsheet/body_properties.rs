#[derive(Default, Debug)]
pub struct BodyProperties {
    vert_overflow: Option<String>,
    horz_overflow: Option<String>,
    rtl_col: Option<String>,
    anchor: Option<String>,
}
impl BodyProperties {
    pub fn get_vert_overflow(&self)-> &Option<String> {
        &self.vert_overflow
    }
    pub fn set_vert_overflow<S: Into<String>>(&mut self, value:S) {
        self.vert_overflow = Some(value.into());
    }
    pub fn get_horz_overflow(&self)-> &Option<String> {
        &self.horz_overflow
    }
    pub fn set_horz_overflow<S: Into<String>>(&mut self, value:S) {
        self.horz_overflow = Some(value.into());
    }
    pub fn get_rtl_col(&self)-> &Option<String> {
        &self.rtl_col
    }
    pub fn set_rtl_col<S: Into<String>>(&mut self, value:S) {
        self.rtl_col = Some(value.into());
    }
    pub fn get_anchor(&self)-> &Option<String> {
        &self.anchor
    }
    pub fn set_anchor<S: Into<String>>(&mut self, value:S) {
        self.anchor = Some(value.into());
    }
}