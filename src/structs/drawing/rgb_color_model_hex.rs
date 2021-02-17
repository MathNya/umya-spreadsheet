// a:srgbClr
#[derive(Default, Debug)]
pub struct RgbColorModelHex {
    val: Option<String>,
    r: Option<String>,
    g: Option<String>,
    b: Option<String>,
}
impl RgbColorModelHex {
    pub fn set_val<S: Into<String>>(&mut self, value:S) {
        self.val = Some(value.into());
    }

    pub fn get_val(&self) -> &Option<String> {
        &self.val
    }

    pub fn set_r<S: Into<String>>(&mut self, value:S) {
        self.r = Some(value.into());
    }

    pub fn get_r(&self) -> &Option<String> {
        &self.r
    }

    pub fn set_g<S: Into<String>>(&mut self, value:S) {
        self.g = Some(value.into());
    }

    pub fn get_g(&self) -> &Option<String> {
        &self.g
    }

    pub fn set_b<S: Into<String>>(&mut self, value:S) {
        self.b = Some(value.into());
    }

    pub fn get_b(&self) -> &Option<String> {
        &self.b
    }
}
