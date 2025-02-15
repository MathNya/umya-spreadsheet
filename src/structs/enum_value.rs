use std::str::FromStr;

use super::EnumTrait;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EnumValue<T: EnumTrait + FromStr> {
    value:         Option<T>,
    value_default: T,
}

impl<T: EnumTrait + FromStr> EnumValue<T> {
    #[inline]
    pub(crate) fn value(&self) -> &T {
        match &self.value {
            Some(v) => v,
            None => &self.value_default,
        }
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use value()")]
    pub(crate) fn get_value(&self) -> &T {
        self.value()
    }

    #[inline]
    pub(crate) fn value_string(&self) -> &str {
        self.value().value_string()
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use value_string()")]
    pub(crate) fn get_value_string(&self) -> &str {
        self.value_string()
    }

    #[inline]
    pub(crate) fn set_value(&mut self, value: T) -> &mut EnumValue<T> {
        self.value = Some(value);
        self
    }

    #[inline]
    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut EnumValue<T> {
        if let Ok(v) = T::from_str(value.into().as_str()) {
            self.set_value(v);
        }
        self
    }

    #[inline]
    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    #[inline]
    pub(crate) fn hash_string(&self) -> &str {
        if self.has_value() {
            return self.value_string();
        }
        "empty!!"
    }

    #[inline]
    #[deprecated(since = "3.0.0", note = "Use hash_string()")]
    pub(crate) fn get_hash_string(&self) -> &str {
        self.hash_string()
    }
}
