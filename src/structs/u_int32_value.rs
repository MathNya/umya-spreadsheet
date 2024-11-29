#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UInt32Value {
    value: Option<u32>,
}
impl UInt32Value {
    #[inline]
    pub(crate) fn get_value(&self) -> &u32 {
        self.value.as_ref().unwrap_or(&0)
    }

    #[inline]
    pub(crate) fn get_value_string(&self) -> String {
        self.get_value().to_string()
    }

    #[inline]
    pub(crate) fn set_value(&mut self, value: u32) -> &mut Self {
        self.value = Some(value);
        self
    }

    #[inline]
    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.set_value(value.into().parse::<u32>().unwrap())
    }

    #[inline]
    pub(crate) fn remove_value(&mut self) -> &mut Self {
        self.value = None;
        self
    }

    #[inline]
    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }

    #[inline]
    pub(crate) fn get_hash_string(&self) -> String {
        if self.has_value() {
            return self.get_value_string();
        }
        String::from("empty!!")
    }
}
