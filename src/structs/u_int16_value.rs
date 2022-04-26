#[derive(Clone, Default, Debug)]
pub struct UInt16Value {
    value: Option<u16>,
}
impl UInt16Value {
    pub(crate) fn get_value(&self) -> &u16 {
        match &self.value {
            Some(v) => v,
            None => &0,
        }
    }

    pub(crate) fn get_value_string(&self) -> String {
        self.get_value().to_string()
    }

    pub(crate) fn set_value(&mut self, value: u16) -> &mut UInt16Value {
        self.value = Some(value);
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
}
