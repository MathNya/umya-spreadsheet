#[derive(Clone, Default, Debug)]
pub struct TrueFalseValue {
    value: Option<bool>,
    value_default: bool,
}
impl TrueFalseValue {
    pub(crate) fn get_value(&self) -> &bool {
        self.value.as_ref().unwrap_or(&self.value_default)
    }

    pub(crate) fn get_value_string(&self) -> &str {
        match self.get_value() {
            true => "t",
            false => "f",
        }
    }

    pub(crate) fn set_value(&mut self, value: bool) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let value: String = value.into();
        self.set_value(!(value.eq_ignore_ascii_case("f") || value.eq_ignore_ascii_case("false")))
    }

    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub(crate) fn _get_hash_string(&self) -> &str {
        if self.has_value() {
            return self.get_value_string();
        }
        "empty!!"
    }
}
