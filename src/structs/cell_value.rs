use super::RichText;
use super::SharedStringItem;
use super::Text;
use helper::formula::*;
use std::borrow::Cow;
use structs::CellRawValue;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct CellValue {
    pub(crate) raw_value: CellRawValue,
    pub(crate) formula: Option<String>,
    pub(crate) formula_attributes: Vec<(String, String)>,
}
impl CellValue {
    pub fn get_data_type(&self) -> &CellRawValue {
        &self.raw_value
    }

    pub fn get_raw_value(&self) -> &CellRawValue {
        &self.raw_value
    }

    pub(crate) fn get_data_type_crate(&self) -> &str {
        match &self.formula {
            Some(_) => {
                return "f";
            }
            None => {}
        }
        self.raw_value.get_data_type()
    }

    pub fn set_formula_attributes(&mut self, formula_attributes: Vec<(String, String)>) {
        self.formula_attributes = formula_attributes;
    }

    pub fn get_formula_attributes(&self) -> Vec<(&str, &str)> {
        self.formula_attributes
            .iter()
            .map(|(a, b)| (a.as_str(), b.as_str()))
            .collect()
    }

    pub fn get_value(&self) -> Cow<'static, str> {
        self.raw_value.to_string().into()
    }

    pub fn get_value_number(&self) -> Option<f64> {
        self.raw_value.get_number()
    }

    pub fn get_value_lazy(&mut self) -> Cow<'static, str> {
        match &self.raw_value {
            CellRawValue::Lazy(v) => {
                self.raw_value = Self::guess_typed_data(v);
            }
            _ => {}
        }
        self.remove_formula();
        self.raw_value.to_string().into()
    }

    pub(crate) fn get_text(&self) -> Option<Text> {
        self.raw_value.get_text()
    }

    pub(crate) fn get_rich_text(&self) -> Option<RichText> {
        self.raw_value.get_rich_text()
    }

    /// Set the raw value after trying to convert `value` into one of the supported data types.
    /// <br />
    /// Types that `value` may be converted to:
    /// - `Null` - if the string was `"NULL"`
    /// - `Numeric` - if the string can be parsed to an `f64`
    /// - `Bool` - if the string was either `"TRUE"` or `"FALSE"`
    /// - `Error` - if the string was `"#VALUE!"`
    /// - `String` - if the string does not fulfill any of the other conditions
    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.raw_value = Self::guess_typed_data(&value.into());
        self.remove_formula();
        self
    }

    #[deprecated(note = "use `set_value` or `set_value_string` instead")]
    pub fn set_value_from_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.set_value(value);
        self
    }

    pub(crate) fn set_value_crate<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.raw_value = Self::guess_typed_data(&value.into());
        self
    }

    pub fn set_value_lazy<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.raw_value = CellRawValue::Lazy(value.into());
        self
    }

    pub fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.raw_value = CellRawValue::String(value.into());
        self.remove_formula();
        self
    }

    pub fn set_value_bool(&mut self, value: bool) -> &mut Self {
        self.raw_value = CellRawValue::Bool(value);
        self.remove_formula();
        self
    }

    #[deprecated(note = "use `set_value_bool` instead")]
    pub fn set_value_from_bool_ref(&mut self, value: &bool) -> &mut Self {
        self.set_value_bool(*value)
    }

    pub fn set_value_number<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<f64>,
    {
        self.raw_value = CellRawValue::Numeric(value.into());
        self.remove_formula();
        self
    }

    pub fn set_rich_text(&mut self, value: RichText) -> &mut Self {
        self.raw_value = CellRawValue::RichText(value);
        self.remove_formula();
        self
    }

    #[deprecated(note = "use `set_rich_text` instead")]
    pub fn set_rich_text_ref(&mut self, value: &RichText) -> &mut Self {
        self.set_rich_text(value.clone())
    }

    pub fn set_formula<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.formula = Some(value.into());
        self
    }

    pub fn remove_formula(&mut self) -> &mut Self {
        self.formula = None;
        self.formula_attributes.clear();
        self
    }

    pub fn set_error(&mut self) -> &mut Self {
        self.set_value_crate("#VALUE!");
        self
    }

    pub(crate) fn set_shared_string_item(&mut self, value: SharedStringItem) -> &mut Self {
        match value.get_text() {
            Some(v) => {
                self.set_value_string(v.get_value());
            }
            None => {}
        }
        match value.get_rich_text() {
            Some(v) => {
                self.set_rich_text_ref(v);
            }
            None => {}
        }
        self.remove_formula();
        self
    }

    pub fn is_formula(&self) -> bool {
        self.formula.is_some()
    }

    pub fn get_formula(&self) -> &str {
        match &self.formula {
            Some(v) => {
                return v;
            }
            None => {}
        }
        ""
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_u16(&mut self, value: u16) -> &mut Self {
        self.set_value_number(value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_u16_ref(&mut self, value: &u16) -> &mut Self {
        self.set_value_number(*value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_u32(&mut self, value: u32) -> &mut Self {
        self.set_value_number(value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_u32_ref(&mut self, value: &u32) -> &mut Self {
        self.set_value_number(*value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_u64(&mut self, value: u64) -> &mut Self {
        self.set_value_number(value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_u64_ref(&mut self, value: &u64) -> &mut Self {
        self.set_value_number(*value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_i16(&mut self, value: i16) -> &mut Self {
        self.set_value_number(value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_i16_ref(&mut self, value: &i16) -> &mut Self {
        self.set_value_number(*value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_i32(&mut self, value: i32) -> &mut Self {
        self.set_value_number(value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_i32_ref(&mut self, value: &i32) -> &mut Self {
        self.set_value_number(*value)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_i64(&mut self, value: i64) -> &mut Self {
        self.set_value_number(value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_i64_ref(&mut self, value: &i64) -> &mut Self {
        self.set_value_number(*value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_usize(&mut self, value: usize) -> &mut Self {
        self.set_value_number(value as f64)
    }

    #[deprecated(note = "use `set_value_number` instead")]
    pub fn set_value_from_usize_ref(&mut self, value: &usize) -> &mut Self {
        self.set_value_number(*value as f64)
    }

    pub(crate) fn guess_typed_data(value: &str) -> CellRawValue {
        let uppercase_value = value.to_uppercase();

        // Match the value against a few data types
        if uppercase_value == "NULL" {
            return CellRawValue::Null;
        }

        if let Ok(f) = value.parse::<f64>() {
            return CellRawValue::Numeric(f);
        }

        if uppercase_value == "TRUE" {
            return CellRawValue::Bool(true);
        }

        if uppercase_value == "FALSE" {
            return CellRawValue::Bool(false);
        }

        if uppercase_value == "#VALUE!" {
            return CellRawValue::Error;
        }

        CellRawValue::String(value.into())
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.is_value_empty() && self.is_formula_empty() && self.is_formula_attributes_empty()
    }

    pub(crate) fn is_value_empty(&self) -> bool {
        self.get_value() == ""
    }

    pub(crate) fn is_formula_empty(&self) -> bool {
        !self.is_formula()
    }

    pub(crate) fn is_formula_attributes_empty(&self) -> bool {
        self.get_formula_attributes().is_empty()
    }

    pub(crate) fn adjustment_insert_formula_coordinate(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        match &self.formula {
            Some(v) => {
                let formula = adjustment_insert_formula_coordinate(
                    v,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                    sheet_name,
                    self_sheet_name,
                );
                self.formula = Some(formula);
            }
            None => {}
        }
    }

    pub(crate) fn adjustment_remove_formula_coordinate(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        match &self.formula {
            Some(v) => {
                let formula = adjustment_remove_formula_coordinate(
                    v,
                    root_col_num,
                    offset_col_num,
                    root_row_num,
                    offset_row_num,
                    sheet_name,
                    self_sheet_name,
                );
                self.formula = Some(formula);
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_value() {
        let mut obj = CellValue::default();

        obj.set_value_string(String::from("TEST"));
        assert_eq!(obj.get_value(), "TEST");
        assert!(obj.get_value_number().is_none());

        obj.set_value_string("TEST");
        assert_eq!(obj.get_value(), "TEST");

        obj.set_value_bool(true);
        assert_eq!(obj.get_value(), "TRUE");

        obj.set_value_from_bool_ref(&true);
        assert_eq!(obj.get_value(), "TRUE");

        obj.set_value_number(1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_u16(1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_u16_ref(&1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_u32(1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_u32_ref(&1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_u64(1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_u64_ref(&1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_i16(1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_i16_ref(&1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_i32(1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_i32_ref(&1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_i64(1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_i64_ref(&1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_usize(1);
        assert_eq!(obj.get_value(), "1");

        obj.set_value_from_usize_ref(&1);
        assert_eq!(obj.get_value(), "1");
    }
}
