// a:schemeClr
#[derive(Default, Debug)]
pub struct SchemeColor {
    val: String,
    lum_mod: Option<String>,
    lum_off: Option<String>,
    shade: Option<String>,
    sat_mod: Option<String>,
    alpha: Option<String>,
}
impl SchemeColor {
    pub fn set_val<S: Into<String>>(&mut self, value:S) {
        self.val = value.into();
    }

    pub fn get_val(&self) -> &str {
        &self.val
    }

    pub fn set_lum_mod<S: Into<String>>(&mut self, value:S) {
        self.lum_mod = Some(value.into());
    }

    pub fn get_lum_mod(&self) -> &Option<String> {
        &self.lum_mod
    }

    pub fn set_lum_off<S: Into<String>>(&mut self, value:S) {
        self.lum_off = Some(value.into());
    }

    pub fn get_lum_off(&self) -> &Option<String> {
        &self.lum_off
    }

    pub fn set_shade<S: Into<String>>(&mut self, value:S) {
        self.shade = Some(value.into());
    }
    
    pub fn get_shade(&self) -> &Option<String> {
        &self.shade
    }

    pub fn set_sat_mod<S: Into<String>>(&mut self, value:S) {
        self.sat_mod = Some(value.into());
    }
    
    pub fn get_sat_mod(&self) -> &Option<String> {
        &self.sat_mod
    }

    pub fn set_alpha<S: Into<String>>(&mut self, value:S) {
        self.alpha = Some(value.into());
    }
    
    pub fn get_alpha(&self) -> &Option<String> {
        &self.alpha
    }

    pub(crate) fn with_inner_params(&self) -> bool {
        self.lum_mod.is_some() ||  self.lum_off.is_some() || self.shade.is_some() || self.sat_mod.is_some() || self.alpha.is_some()
    }
}
