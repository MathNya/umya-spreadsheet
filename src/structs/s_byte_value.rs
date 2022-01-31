#[derive(Clone, Default, Debug)]
pub struct SByteValue {
    value: Option<i8>,
    value_default: i8,
}
impl SByteValue {
    pub(crate) fn get_value(&self) -> &i8 {
        match &self.value {
            Some(v) => v,
            None => &self.value_default,
        }
    }

    pub(crate) fn get_value_string(&self) -> String {
        self.get_value().to_string()
    }

    pub(crate) fn set_value(&mut self, value: i8) -> &mut SByteValue {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut SByteValue {
        self.set_value(value.into().parse::<i8>().unwrap())
    }

    pub(crate) fn has_value(&self) -> bool {
        match &self.value {
            Some(_) => true,
            None => false,
        }
    }
}
