#[derive(Clone, Default, Debug)]
pub struct BooleanValue {
    value: Option<bool>,
    value_default: bool,
}
impl BooleanValue {
    pub(crate) fn get_value(&self)-> &bool {
        match &self.value {
            Some(v) => v,
            None => &self.value_default
        }
    }

    pub(crate) fn get_value_string(&self)-> &str {
        if self.get_value() == &false {"0"} else {"1"}
    }

    pub(crate) fn set_value(&mut self, value:bool) -> &mut BooleanValue {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value:S) -> &mut BooleanValue {
        self.set_value(if value.into() == "0"{false}else{true})
    }

    pub(crate) fn has_value(&self)-> bool {
        match &self.value {
            Some(_) => true,
            None => false
        }
    }

    pub(crate) fn get_hash_string(&self)-> &str
    {
        if self.has_value() {
            return self.get_value_string();
        }
        "empty!!"
    }
}
