#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Int32Value {
    value: Option<i32>,
}
impl Int32Value {
    pub(crate) fn get_value(&self) -> &i32 {
        match &self.value {
            Some(v) => v,
            None => &0,
        }
    }

    pub(crate) fn get_value_string(&self) -> String {
        self.get_value().to_string()
    }

    pub(crate) fn set_value(&mut self, value: i32) -> &mut Self {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.set_value(value.into().parse::<i32>().unwrap())
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
