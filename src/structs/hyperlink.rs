#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hyperlink {
    url:      Box<str>,
    tooltip:  Box<str>,
    location: bool,
}
impl Hyperlink {
    #[inline]
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use url()")]
    pub fn get_url(&self) -> &str {
        self.url()
    }

    #[inline]
    pub fn set_url<S: Into<String>>(&mut self, value: S) -> &mut Hyperlink {
        self.url = value.into().into_boxed_str();
        self
    }

    #[inline]
    #[must_use]
    pub fn tooltip(&self) -> &str {
        &self.tooltip
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use tooltip()")]
    pub fn get_tooltip(&self) -> &str {
        self.tooltip()
    }

    #[inline]
    pub fn set_tooltip<S: Into<String>>(&mut self, value: S) -> &mut Hyperlink {
        self.tooltip = value.into().into_boxed_str();
        self
    }

    #[inline]
    #[must_use]
    pub fn location(&self) -> bool {
        self.location
    }

    #[inline]
    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use location()")]
    pub fn get_location(&self) -> bool {
        self.location()
    }

    #[inline]
    pub fn set_location(&mut self, value: bool) -> &mut Hyperlink {
        self.location = value;
        self
    }
}
