use super::RichText;
use super::SharedStringItem;
use super::Text;
use helper::formula::*;
use std::borrow::Cow;
use structs::CellFormula;
use structs::CellRawValue;
use traits::AdjustmentCoordinateWith2Sheet;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct CellValue {
    pub(crate) raw_value: CellRawValue,
    pub(crate) formula: Option<CellFormula>,
}
impl CellValue {
    pub fn get_data_type(&self) -> &str {
        &self.raw_value.get_data_type()
    }

    pub fn get_raw_value(&self) -> &CellRawValue {
        &self.raw_value
    }

    pub(crate) fn get_data_type_crate(&self) -> &str {
        self.raw_value.get_data_type()
    }

    pub fn get_value(&self) -> Cow<'static, str> {
        self.raw_value.to_string().into()
    }

    pub fn get_value_number(&self) -> Option<f64> {
        self.raw_value.get_number()
    }

    pub fn get_value_lazy(&mut self) -> Cow<'static, str> {
        if let CellRawValue::Lazy(v) = &self.raw_value {
            self.raw_value = Self::guess_typed_data(v);
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

    pub(crate) fn set_value_str<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.raw_value = CellRawValue::Str(value.into());
        self
    }

    pub fn set_value_bool(&mut self, value: bool) -> &mut Self {
        self.raw_value = CellRawValue::Bool(value);
        self.remove_formula();
        self
    }

    pub(crate) fn set_value_bool_crate(&mut self, value: bool) -> &mut Self {
        self.raw_value = CellRawValue::Bool(value);
        self
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

    pub fn is_formula(&self) -> bool {
        self.formula.is_some()
    }

    pub fn get_formula(&self) -> &str {
        match &self.formula {
            Some(v) => v.get_text(),
            None => "",
        }
    }

    pub fn set_formula<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let mut obj = CellFormula::default();
        obj.set_text(value.into());
        self.formula = Some(obj);
        self
    }

    pub fn set_formula_obj(&mut self, value: CellFormula) -> &mut Self {
        self.formula = Some(value);
        self
    }

    pub fn remove_formula(&mut self) -> &mut Self {
        self.formula = None;
        self
    }

    pub fn set_error(&mut self) -> &mut Self {
        self.set_value_crate("#VALUE!");
        self
    }

    pub(crate) fn set_shared_string_item(&mut self, value: SharedStringItem) -> &mut Self {
        if let Some(v) = value.get_text() {
            self.set_value_string(v.get_value());
        }
        if let Some(v) = value.get_rich_text() {
            self.set_rich_text(v.clone());
        }
        self
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
        self.is_value_empty() && self.is_formula_empty()
    }

    pub(crate) fn is_value_empty(&self) -> bool {
        self.get_value() == ""
    }

    pub(crate) fn is_formula_empty(&self) -> bool {
        !self.is_formula()
    }
}
impl AdjustmentCoordinateWith2Sheet for CellValue {
    fn adjustment_insert_coordinate_with_2sheet(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if let Some(v) = &mut self.formula {
            v.adjustment_insert_coordinate_with_2sheet(
                self_sheet_name,
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
        }
    }

    fn adjustment_remove_coordinate_with_2sheet(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: &u32,
        offset_col_num: &u32,
        root_row_num: &u32,
        offset_row_num: &u32,
    ) {
        if let Some(v) = &mut self.formula {
            v.adjustment_remove_coordinate_with_2sheet(
                self_sheet_name,
                sheet_name,
                root_col_num,
                offset_col_num,
                root_row_num,
                offset_row_num,
            );
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

        obj.set_value_number(1);
        assert_eq!(obj.get_value(), "1");
    }
}
