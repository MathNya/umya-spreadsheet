#[derive(Clone, Default, Debug)]
pub struct Int16Value {
    #[allow(dead_code)]
    value: Option<i16>,
}
impl Int16Value {
    pub(crate) fn get_value(&self) -> i16 {
        self.value.unwrap_or(0)
    }

    pub(crate) fn get_value_string(&self) -> String {
        self.get_value().to_string()
    }

    pub(crate) fn set_value(&mut self, value: i16) -> &mut Int16Value {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Int16Value {
        self.set_value(value.into().parse::<i16>().unwrap())
    }

    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub(crate) fn get_hash_string(&self) -> String {
        if self.has_value() {
            return self.get_value_string();
        }
        String::from("empty!!")
    }
}
