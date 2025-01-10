#[derive(Clone, Default, Debug)]
pub struct SByteValue {
    value: Option<i8>,
}
impl SByteValue {
    #[inline]
    pub(crate) fn get_value(&self) -> i8 {
        self.value.unwrap_or(0)
    }

    #[inline]
    pub(crate) fn get_value_string(&self) -> String {
        self.get_value().to_string()
    }

    #[inline]
    pub(crate) fn set_value(&mut self, value: i8) -> &mut SByteValue {
        self.value = Some(value);
        self
    }

    #[inline]
    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut SByteValue {
        self.set_value(value.into().parse::<i8>().unwrap())
    }

    #[inline]
    pub(crate) fn _has_value(&self) -> bool {
        self.value.is_some()
    }
}
