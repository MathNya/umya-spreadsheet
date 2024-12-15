#[derive(Clone, Default, Debug)]
pub struct Int64Value {
    value: Option<i64>,
}
impl Int64Value {
    #[inline]
    pub(crate) fn get_value(&self) -> i64 {
        self.value.unwrap_or(0)
    }

    #[inline]
    pub(crate) fn get_value_string(&self) -> String {
        self.get_value().to_string()
    }

    #[inline]
    pub(crate) fn set_value(&mut self, value: i64) -> &mut Int64Value {
        self.value = Some(value);
        self
    }

    #[inline]
    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Int64Value {
        self.set_value(value.into().parse::<i64>().unwrap())
    }

    #[inline]
    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    #[inline]
    pub(crate) fn _get_hash_string(&self) -> String {
        if self.has_value() {
            return self.get_value_string();
        }
        String::from("empty!!")
    }
}
