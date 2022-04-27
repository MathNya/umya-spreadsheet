#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct DoubleValue {
    value: Option<f64>,
}
impl DoubleValue {
    pub(crate) fn get_value(&self) -> &f64 {
        match &self.value {
            Some(v) => v,
            None => &0f64,
        }
    }

    pub(crate) fn get_value_string(&self) -> String {
        self.get_value().to_string()
    }

    pub(crate) fn set_value(&mut self, value: f64) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.set_value(value.into().parse::<f64>().unwrap_or_default())
    }

    pub(crate) fn has_value(&self) -> bool {
        match &self.value {
            Some(_) => true,
            None => false,
        }
    }

    pub(crate) fn get_hash_string(&self) -> String {
        if self.has_value() {
            return self.get_value_string();
        }
        String::from("empty!!")
    }
}
