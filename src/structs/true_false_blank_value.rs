#[derive(Clone, Default, Debug)]
pub struct TrueFalseBlankValue {
    value: Option<bool>,
}
impl TrueFalseBlankValue {
    pub(crate) fn get_value(&self) -> Option<bool> {
        self.value
    }

    pub(crate) fn get_value_str(&self) -> &str {
        self.get_value().map_or("", |v| if v { "t" } else { "f" })
    }

    pub(crate) fn get_value_string2(&self) -> &str {
        self.get_value()
            .map_or("", |v| if v { "True" } else { "False" })
    }

    pub(crate) fn set_value(&mut self, value: bool) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let value = value.into();
        self.set_value(!(value.eq_ignore_ascii_case("f") || value.eq_ignore_ascii_case("false")))
    }

    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub(crate) fn get_hash_string(&self) -> &str {
        if self.has_value() {
            return self.get_value_str();
        }
        "empty!!"
    }
}
