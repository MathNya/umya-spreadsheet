use super::super::solid_fill::SolidFill;

#[derive(Default, Debug)]
pub struct RunProperties {
    text: String,
    kumimoji: Option<String>,
    lang: Option<String>,
    alt_lang: Option<String>,
    sz: Option<String>,
    solid_fill: Option<SolidFill>,
}
impl RunProperties {
    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text<S: Into<String>>(&mut self, value:S) {
        self.text = value.into();
    }

    pub fn get_kumimoji(&self) -> &Option<String> {
        &self.kumimoji
    }

    pub fn set_kumimoji<S: Into<String>>(&mut self, value:S) {
        self.kumimoji = Some(value.into());
    }

    pub fn get_lang(&self) -> &Option<String> {
        &self.lang
    }

    pub fn set_lang<S: Into<String>>(&mut self, value:S) {
        self.lang = Some(value.into());
    }

    pub fn get_alt_lang(&self) -> &Option<String> {
        &self.alt_lang
    }

    pub fn set_alt_lang<S: Into<String>>(&mut self, value:S) {
        self.alt_lang = Some(value.into());
    }

    pub fn get_sz(&self) -> &Option<String> {
        &self.sz
    }

    pub fn set_sz<S: Into<String>>(&mut self, value:S) {
        self.sz = Some(value.into());
    }

    pub fn get_solid_fill(&self) -> &Option<SolidFill> {
        &self.solid_fill
    }

    pub fn get_solid_fill_mut(&mut self) -> &mut Option<SolidFill> {
        &mut self.solid_fill
    }
    
    pub fn set_solid_fill(&mut self, value:SolidFill) {
        self.solid_fill = Some(value);
    }
}