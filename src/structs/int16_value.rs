#[derive(Clone, Debug)]
pub struct Int16Value {
    value: Option<i16>,
    value_string: String,
    value_default: i16,
}
impl Default for Int16Value {
    fn default() -> Self {
        Self {
            value: None,
            value_string: "0".into(),
            value_default: 0,
        }
    }
}
impl Int16Value {
    pub(crate) fn get_value(&self) -> &i16 {
        match &self.value {
            Some(v) => v,
            None => &self.value_default,
        }
    }

    pub(crate) fn get_value_string(&self) -> &str {
        &self.value_string
    }

    pub(crate) fn set_value(&mut self, value: i16) -> &mut Int16Value {
        self.value = Some(value);
        self.value_string = value.to_string();
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Int16Value {
        self.set_value(value.into().parse::<i16>().unwrap())
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
