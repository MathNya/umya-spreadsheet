#[derive(Clone, Default, Debug)]
pub struct Int16Value {
    value: Option<i16>,
}
impl Int16Value {
    pub(crate) fn _get_value(&self) -> &i16 {
        match &self.value {
            Some(v) => v,
            None => &0,
        }
    }

    pub(crate) fn _get_value_string(&self) -> String {
        self._get_value().to_string()
    }

    pub(crate) fn _set_value(&mut self, value: i16) -> &mut Int16Value {
        self.value = Some(value);
        self
    }

    pub(crate) fn _set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Int16Value {
        self._set_value(value.into().parse::<i16>().unwrap())
    }

    pub(crate) fn _has_value(&self) -> bool {
        match &self.value {
            Some(_) => true,
            None => false,
        }
    }

    pub(crate) fn _get_hash_string(&self) -> String {
        if self._has_value() {
            return self._get_value_string();
        }
        String::from("empty!!")
    }
}
