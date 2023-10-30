#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BooleanValue {
    value: Option<bool>,
}

impl BooleanValue {
    pub(crate) fn get_value(&self) -> &bool {
        match &self.value {
            Some(v) => v,
            None => &false,
        }
    }

    pub(crate) fn get_value_string(&self) -> &str {
        if *self.get_value() {
            "1"
        } else {
            "0"
        }
    }

    pub(crate) fn set_value(&mut self, value: bool) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.set_value(matches!(value.into().as_str(), "true" | "1"))
    }

    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub(crate) fn get_hash_string(&self) -> &str {
        if self.has_value() {
            return self.get_value_string();
        }
        "empty!!"
    }
}
