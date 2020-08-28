#[derive(Default, Debug)]
pub struct Hyperlink {
    url: String,
    tooltip: String,
    location: bool,
}
impl Hyperlink {
    pub fn get_url(&self) -> &String {
        &self.url
    }
    pub fn set_url<S: Into<String>>(&mut self, value:S) -> Result<(), &'static str> {
        self.url = value.into();
        Ok(())
    }
    pub fn get_tooltip(&self) -> &String {
        &self.tooltip
    }
    pub fn set_tooltip<S: Into<String>>(&mut self, value:S) -> Result<(), &'static str> {
        self.tooltip = value.into();
        Ok(())
    }
    pub fn get_location(&self) -> &bool {
        &self.location
    }
    pub fn set_location(&mut self, value:bool) -> Result<(), &'static str> {
        self.location = value;
        Ok(())
    }
}
