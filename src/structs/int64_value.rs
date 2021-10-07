#[derive(Clone, Debug)]
pub struct Int64Value {
    value: Option<i64>,
    value_string: String,
    value_default: i64,
}
impl Default for Int64Value {
    fn default() -> Self {
        Self {
            value: None,
            value_string: "0".into(),
            value_default: 0,
        }
    }
}
impl Int64Value {
    pub(crate) fn get_value(&self)-> &i64 {
        match &self.value {
            Some(v) => v,
            None => &self.value_default
        }
    }

    pub(crate) fn get_value_string(&self)-> &str {
        &self.value_string
    }

    pub(crate) fn set_value(&mut self, value:i64) -> &mut Int64Value {
        self.value = Some(value);
        self.value_string = value.to_string();
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value:S) -> &mut Int64Value {
        self.set_value(value.into().parse::<i64>().unwrap())
    }

    pub(crate) fn has_value(&self)-> bool {
        match &self.value {
            Some(_) => true,
            None => false
        }
    }

    pub(crate) fn get_hash_string(&self)-> &str
    {
        if self.has_value() {
            return self.get_value_string();
        }
        "empty!!"
    }
}
