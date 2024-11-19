#[derive(Clone, Default, Debug)]
pub struct ByteValue {
    value: Option<u8>,
}
impl ByteValue {
    #[inline]
    pub(crate) fn get_value(&self) -> &u8 {
        match &self.value {
            Some(v) => v,
            None => &0,
        }
    }

    #[inline]
    pub(crate) fn get_value_string(&self) -> String {
        self.get_value().to_string()
    }

    #[inline]
    pub(crate) fn set_value(&mut self, value: u8) -> &mut ByteValue {
        self.value = Some(value);
        self
    }

    #[inline]
    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut ByteValue {
        self.set_value(value.into().parse::<u8>().unwrap())
    }

    #[inline]
    pub(crate) fn has_value(&self) -> bool {
        self.value.is_some()
    }
}
