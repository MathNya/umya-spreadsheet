#[derive(Clone, Default, Debug)]
pub struct TrueFalseBlankValue {
    value: Option<bool>,
}
impl TrueFalseBlankValue {
    #[inline]
    pub(crate) fn value(&self) -> Option<bool> {
        self.value
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use value_string()")]
    pub(crate) fn get_value(&self) -> Option<bool> {
        self.value()
    }

    #[inline]
    pub(crate) fn value_str(&self) -> &str {
        self.value().map_or("", |v| if v { "t" } else { "f" })
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use value_string()")]
    pub(crate) fn get_value_str(&self) -> &str {
        self.value_str()
    }

    #[inline]
    pub(crate) fn value_string2(&self) -> &str {
        self.value()
            .map_or("", |v| if v { "True" } else { "False" })
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use value_string()")]
    pub(crate) fn get_value_string2(&self) -> &str {
        self.value_string2()
    }

    #[inline]
    pub(crate) fn set_value(&mut self, value: bool) -> &mut Self {
        self.value = Some(value);
        self
    }

    #[inline]
    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let value = value.into();
        self.set_value(!(value.eq_ignore_ascii_case("f") || value.eq_ignore_ascii_case("false")))
    }

    #[inline]
    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    #[inline]
    pub(crate) fn hash_string(&self) -> &str {
        if self.has_value() {
            return self.value_str();
        }
        "empty!!"
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use hash_string()")]
    pub(crate) fn get_hash_string(&self) -> &str {
        self.hash_string()
    }
}
