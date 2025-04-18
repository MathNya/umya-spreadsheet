#[derive(Clone, Default, Debug)]
pub struct UInt16Value {
    value: Option<u16>,
}
impl UInt16Value {
    #[inline]
    pub(crate) fn get_value(&self) -> &u16 {
        self.value.as_ref().unwrap_or(&0)
    }

    #[inline]
    pub(crate) fn get_value_string(&self) -> String {
        self.get_value().to_string()
    }

    #[inline]
    pub(crate) fn set_value(&mut self, value: u16) -> &mut UInt16Value {
        self.value = Some(value);
        self
    }

    #[inline]
    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut UInt16Value {
        self.set_value(value.into().parse::<u16>().unwrap())
    }

    #[inline]
    pub(crate) fn _has_value(&self) -> bool {
        self.value.is_some()
    }
}
