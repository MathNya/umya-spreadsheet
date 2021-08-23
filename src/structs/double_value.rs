#[derive(Default, Debug)]
pub struct DoubleValue {
    value: Option<f64>,
    value_default: f64,
}
impl DoubleValue {
    pub(crate) fn get_value(&self)-> &f64 {
        match &self.value {
            Some(v) => v,
            None => &self.value_default
        }
    }

    pub(crate) fn get_value_string(&self)-> String {
        self.get_value().to_string()
    }

    pub(crate) fn set_value(&mut self, value:f64) -> &mut DoubleValue {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value:S) -> &mut DoubleValue {
        self.set_value(value.into().parse::<f64>().unwrap())
    }

    pub(crate) fn has_value(&self)-> bool {
        match &self.value {
            Some(_) => true,
            None => false
        }
    }
}
