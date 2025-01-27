#[derive(Clone, Default, Debug)]
pub struct Int64Value {
    value: Option<i64>,
}
impl Int64Value {
    #[inline]
    pub(crate) fn value(&self) -> i64 {
        self.value.unwrap_or(0)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use value()")]
    pub(crate) fn get_value(&self) -> i64 {
        self.value()
    }

    #[inline]
    pub(crate) fn value_string(&self) -> String {
        self.value().to_string()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use value_string()")]
    pub(crate) fn get_value_string(&self) -> String {
        self.value_string()
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
    pub(crate) fn hash_string(&self) -> String {
        if self.has_value() {
            return self.value_string();
        }
        String::from("empty!!")
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use hash_string()")]
    pub(crate) fn get_hash_string(&self) -> String {
        self.hash_string()
    }
}
