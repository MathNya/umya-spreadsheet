#[derive(Clone, Debug)]
pub struct Int32Value {
    value: Option<i32>,
    value_string: String,
    value_default: i32,
}
impl Default for Int32Value {
    fn default() -> Self {
        Self {
            value: None,
            value_string: "0".into(),
            value_default: 0,
        }
    }
}
impl Int32Value {
    pub(crate) fn get_value(&self) -> &i32 {
        match &self.value {
            Some(v) => v,
            None => &self.value_default,
        }
    }

    pub(crate) fn get_value_string(&self) -> &str {
        &self.value_string
    }

    pub(crate) fn set_value(&mut self, value: i32) -> &mut Int32Value {
        self.value = Some(value);
        self.value_string = value.to_string();
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Int32Value {
        self.set_value(value.into().parse::<i32>().unwrap())
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
