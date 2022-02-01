#[derive(Clone, Debug)]
pub struct UInt16Value {
    value: Option<u16>,
    value_string: String,
    value_default: u16,
}
impl Default for UInt16Value {
    fn default() -> Self {
        Self {
            value: None,
            value_string: String::from("0"),
            value_default: 0,
        }
    }
}
impl UInt16Value {
    pub(crate) fn get_value(&self) -> &u16 {
        match &self.value {
            Some(v) => v,
            None => &self.value_default,
        }
    }

    pub(crate) fn get_value_string(&self) -> &str {
        &self.value_string
    }

    pub(crate) fn set_value(&mut self, value: u16) -> &mut UInt16Value {
        self.value = Some(value);
        self.value_string = value.to_string();
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut UInt16Value {
        self.set_value(value.into().parse::<u16>().unwrap())
    }

    pub(crate) fn _has_value(&self) -> bool {
        match &self.value {
            Some(_) => true,
            None => false,
        }
    }

    pub(crate) fn _get_hash_string(&self) -> &str {
        if self._has_value() {
            return self.get_value_string();
        }
        "empty!!"
    }
}
