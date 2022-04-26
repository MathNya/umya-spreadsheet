#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UInt32Value {
    value: Option<u32>,
}
impl UInt32Value {
    pub(crate) fn get_value(&self) -> &u32 {
        match &self.value {
            Some(v) => v,
            None => &0,
        }
    }

    pub(crate) fn get_value_string(&self) -> String {
        self.get_value().to_string()
    }

    pub(crate) fn set_value(&mut self, value: u32) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.set_value(value.into().parse::<u32>().unwrap())
    }

    pub(crate) fn remove_value(&mut self) -> &mut Self {
        self.value = None;
        self
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
