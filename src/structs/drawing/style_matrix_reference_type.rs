// a:lnRef
use super::scheme_color::SchemeColor;

#[derive(Default, Debug)]
pub struct StyleMatrixReferenceType {
    index: String,
    scheme_color: Option<SchemeColor>,
}
impl StyleMatrixReferenceType {
    pub fn get_index(&self)-> &str {
        &self.index
    }

    pub fn set_index<S: Into<String>>(&mut self, value:S) {
        self.index = value.into();
    }

    pub fn get_scheme_color(&self)-> &Option<SchemeColor> {
        &self.scheme_color
    }

    pub fn set_scheme_color(&mut self, value:SchemeColor) {
        self.scheme_color = Some(value.into());
    }
}