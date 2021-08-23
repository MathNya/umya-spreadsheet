#[derive(Default, Debug)]
pub struct UInt32Value {
    value: Option<u32>,
    value_default: u32,
}
impl UInt32Value {
    pub(crate) fn get_value(&self)-> &u32 {
        match &self.value {
            Some(v) => v,
            None => &self.value_default
        }
    }

    pub(crate) fn get_value_string(&self)-> String {
        self.get_value().to_string()
    }

    pub(crate) fn set_value(&mut self, value:u32) -> &mut UInt32Value {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value:S) -> &mut UInt32Value {
        self.set_value(value.into().parse::<u32>().unwrap())
    }

    pub(crate) fn has_value(&self)-> bool {
        match &self.value {
            Some(_) => true,
            None => false
        }
    }
}
