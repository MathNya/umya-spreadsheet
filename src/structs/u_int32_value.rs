#[derive(Clone, Debug)]
pub struct UInt32Value {
    value: Option<u32>,
    value_string: String,
    value_default: u32,
}
impl Default for UInt32Value {
    fn default() -> Self {
        Self {
            value: None,
            value_string: String::from("0"),
            value_default: 0,
        }
    }
}
impl UInt32Value {
    pub(crate) fn get_value(&self)-> &u32 {
        match &self.value {
            Some(v) => v,
            None => &self.value_default
        }
    }

    pub(crate) fn get_value_string(&self)-> &str {
        &self.value_string
    }

    pub(crate) fn set_value(&mut self, value:u32) -> &mut Self {
        self.value = Some(value);
        self.value_string = value.to_string();
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value:S) -> &mut Self {
        self.set_value(value.into().parse::<u32>().unwrap())
    }

    pub(crate) fn remove_value(&mut self) -> &mut Self {
        self.value = None;
        self.value_string = String::from("");
        self
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
