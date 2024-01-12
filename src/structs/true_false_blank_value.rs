#[derive(Clone, Default, Debug)]
pub struct TrueFalseBlankValue {
    value: Option<bool>,
}
impl TrueFalseBlankValue {
    pub(crate) fn get_value(&self) -> Option<&bool> {
        self.value.as_ref()
    }

    pub(crate) fn _get_value_string(&self) -> &str {
        match self.get_value() {
            Some(v) => {
                if !*v {
                    "f"
                } else {
                    "t"
                }
            }
            None => "",
        }
    }

    pub(crate) fn get_value_string2(&self) -> &str {
        match self.get_value() {
            Some(v) => {
                if !*v {
                    "False"
                } else {
                    "True"
                }
            }
            None => "",
        }
    }

    pub(crate) fn set_value(&mut self, value: bool) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let value_str = value.into();
        self.set_value(!(&value_str == "f" || &value_str == "False"))
    }

    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub(crate) fn _get_hash_string(&self) -> &str {
        if self.has_value() {
            return self._get_value_string();
        }
        "empty!!"
    }
}
