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
        if self.get_value() == &false {
            "0"
        } else {
            "1"
        }
    }

    pub(crate) fn set_value(&mut self, value: bool) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let v = match value.into().as_str() {
            "true" => true,
            "1" => true,
            _ => false,
        };
        self.set_value(v)
    }

    pub(crate) fn has_value(&self) -> bool {
        match &self.value {
            Some(_) => true,
            None => false,
        }
    }

    pub(crate) fn get_hash_string(&self) -> &str {
        if self.has_value() {
            return self.get_value_string();
        }
        "empty!!"
    }
}
