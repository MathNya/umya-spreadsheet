#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BooleanValue {
    value: Option<bool>,
}

impl BooleanValue {
    #[inline]
    pub(crate) fn get_value(&self) -> bool {
        self.value.unwrap_or(false)
    }

    #[inline]
    pub(crate) fn get_value_string(&self) -> &str {
        match self.get_value() {
            true => "1",
            false => "0",
        }
    }

    #[inline]
    pub(crate) fn set_value(&mut self, value: bool) -> &mut Self {
        self.value = Some(value);
        self
    }

    #[inline]
    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.set_value(matches!(value.into().as_str(), "true" | "1"))
    }

    #[inline]
    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    #[inline]
    pub(crate) fn get_hash_string(&self) -> &str {
        if self.has_value() {
            return self.get_value_string();
        }
        "empty!!"
    }
}
