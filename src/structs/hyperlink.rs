#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hyperlink {
    url: String,
    tooltip: String,
    location: bool,
}
impl Hyperlink {
    pub fn get_url(&self) -> &String {
        &self.url
    }

    pub fn set_url<S: Into<String>>(&mut self, value: S) -> &mut Hyperlink {
        self.url = value.into();
        self
    }

    pub fn get_tooltip(&self) -> &String {
        &self.tooltip
    }

    pub fn set_tooltip<S: Into<String>>(&mut self, value: S) -> &mut Hyperlink {
        self.tooltip = value.into();
        self
    }

    pub fn get_location(&self) -> &bool {
        &self.location
    }

    pub fn set_location(&mut self, value: bool) -> &mut Hyperlink {
        self.location = value;
        self
    }
}
