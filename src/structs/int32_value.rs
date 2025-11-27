#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Int32Value {
    value: Option<i32>,
}
impl Int32Value {
    #[inline]
    pub(crate) fn value(&self) -> i32 {
        self.value.unwrap_or(0)
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use value()")]
    pub(crate) fn get_value(&self) -> i32 {
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
    pub(crate) fn set_value(&mut self, value: i32) -> &mut Self {
        self.value = Some(value);
        self
    }

    #[inline]
    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.set_value(value.into().parse::<i32>().unwrap())
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
