#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hyperlink {
    url: Box<str>,
    tooltip: Box<str>,
    location: bool,
}
impl Hyperlink {
    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn set_url<S: Into<String>>(&mut self, value: S) -> &mut Hyperlink {
        self.url = value.into().into_boxed_str();
        self
    }

    pub fn get_tooltip(&self) -> &str {
        &self.tooltip
    }

    pub fn set_tooltip<S: Into<String>>(&mut self, value: S) -> &mut Hyperlink {
        self.tooltip = value.into().into_boxed_str();
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
