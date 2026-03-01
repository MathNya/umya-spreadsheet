#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Int32Value {
    value: Option<i32>,
}
impl Int32Value {
    #[inline]
    pub(crate) fn get_value(&self) -> &i32 {
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
    pub(crate) fn set_value(&mut self, value: i32) -> &mut Self {
        self.value = Some(value);
        self
    }

    #[inline]
    pub(crate) fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let s = value.into();
        self.set_value(s.parse::<i32>().unwrap_or_else(|_| {
            s.parse::<i64>()
                .ok()
                .and_then(|v| i32::try_from(v).ok())
                .unwrap_or(0)
        }))
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
