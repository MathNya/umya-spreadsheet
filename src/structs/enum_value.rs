use super::EnumTrait;
use std::str::FromStr;
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EnumValue<T: EnumTrait + FromStr> {
    value: Option<T>,
    value_default: T,
}
impl<T: EnumTrait + FromStr> EnumValue<T> {
    pub(crate) fn get_value(&self) -> &T {
        match &self.value {
            Some(v) => v,
            None => &self.value_default,
        }
    }

    pub(crate) fn get_value_string(&self) -> &str {
        self.get_value().get_value_string()
    }

    pub(crate) fn set_value(&mut self, value: T) -> &mut EnumValue<T> {
        self.value = Some(value);
        self
    }

    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut EnumValue<T> {
        match T::from_str(value.into().as_str()) {
            Ok(v) => {
                self.set_value(v);
            }
            Err(_) => {}
        }
        self
    }

    pub(crate) fn has_value(&self) -> bool {
        match &self.value {
            Some(_) => true,
            None => false,
        }
    }

    pub(crate) fn get_hash_string(&self) -> &str {
        if self.has_value() {
            return self.get_value_string();
        }
        "empty!!"
    }
}
