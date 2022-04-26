#[derive(Clone, Default, Debug)]
pub struct Int64Value {
    value: Option<i64>,
}
impl Int64Value {
    pub(crate) fn get_value(&self) -> &i64 {
        match &self.value {
            Some(v) => v,
            None => &0,
        }
    }

    pub(crate) fn get_value_string(&self) -> String {
        self.get_value().to_string()
    }

    pub(crate) fn set_value(&mut self, value: i64) -> &mut Int64Value {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Int64Value {
        self.set_value(value.into().parse::<i64>().unwrap())
    }

    pub(crate) fn has_value(&self) -> bool {
        match &self.value {
            Some(_) => true,
            None => false,
        }
    }

    pub(crate) fn _get_hash_string(&self) -> String {
        if self.has_value() {
            return self.get_value_string();
        }
        String::from("empty!!")
    }
}
