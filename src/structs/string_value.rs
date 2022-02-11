#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StringValue {
    value: Option<String>,
    value_default: String,
}
impl StringValue {
    pub(crate) fn get_value(&self) -> &str {
        match &self.value {
            Some(v) => v,
            None => &self.value_default,
        }
    }

    pub(crate) fn get_value_string(&self) -> &str {
        self.get_value()
    }

    pub(crate) fn set_value<S: Into<String>>(&mut self, value: S) -> &mut StringValue {
        self.value = Some(value.into());
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut StringValue {
        self.set_value(value.into())
    }

    pub(crate) fn remove_value(&mut self) -> &mut Self {
        self.value = None;
        self
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
