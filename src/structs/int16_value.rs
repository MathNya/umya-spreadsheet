#[derive(Default, Debug)]
pub struct Int16Value {
    value: Option<i16>,
    value_default: i16,
}
impl Int16Value {
    pub(crate) fn get_value(&self)-> &i16 {
        match &self.value {
            Some(v) => v,
            None => &self.value_default
        }
    }

    pub(crate) fn get_value_string(&self)-> String {
        self.get_value().to_string()
    }

    pub(crate) fn set_value(&mut self, value:i16) -> &mut Int16Value {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value:S) -> &mut Int16Value {
        self.set_value(value.into().parse::<i16>().unwrap())
    }

    pub(crate) fn has_value(&self)-> bool {
        match &self.value {
            Some(_) => true,
            None => false
        }
    }
}
