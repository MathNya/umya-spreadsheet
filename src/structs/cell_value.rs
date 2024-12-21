use super::RichText;
use super::SharedStringItem;
use super::Text;
use crate::structs::CellFormula;
use crate::structs::CellRawValue;
use crate::traits::AdjustmentCoordinateWith2Sheet;
use crate::CellErrorType;
use std::borrow::Cow;
use std::str::FromStr;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct CellValue {
    pub(crate) raw_value: CellRawValue,
    pub(crate) formula: Option<Box<CellFormula>>,
}
impl CellValue {
    #[inline]
    #[must_use]
    pub fn get_data_type(&self) -> &str {
        self.raw_value.get_data_type()
    }

    #[inline]
    #[must_use]
    pub fn get_raw_value(&self) -> &CellRawValue {
        &self.raw_value
    }

    #[inline]
    pub(crate) fn get_data_type_crate(&self) -> &str {
        match &self.formula {
            Some(_) => "str",
            None => self.raw_value.get_data_type(),
        }
    }

    #[inline]
    #[must_use]
    pub fn get_value(&self) -> Cow<'static, str> {
        self.raw_value.to_string().into()
    }

    #[inline]
    #[must_use]
    pub fn get_value_number(&self) -> Option<f64> {
        self.raw_value.get_number()
    }

    #[inline]
    pub fn get_value_lazy(&mut self) -> Cow<'static, str> {
        if let CellRawValue::Lazy(v) = &self.raw_value {
            self.raw_value = Self::guess_typed_data(v);
        }
        self.remove_formula();
        self.raw_value.to_string().into()
    }

    #[inline]
    pub(crate) fn get_text(&self) -> Option<Text> {
        self.raw_value.get_text()
    }

    #[inline]
    pub(crate) fn get_rich_text(&self) -> Option<RichText> {
        self.raw_value.get_rich_text()
    }

    /// Set the raw value after trying to convert `value` into one of the supported data types.
    /// <br />
    /// Types that `value` may be converted to:
    /// - `Empty` - if the string was `""`
    /// - `Numeric` - if the string can be parsed to an `f64`
    /// - `Bool` - if the string was either `"TRUE"` or `"FALSE"`
    /// - `Error` - if the string was either `"#VALUE!"`,`"#REF!"`,`"#NUM!"`,`"#NULL!"`,`"#NAME?"`,`"#N/A"`,`"#DATA!"` or `"#DIV/0!"`
    /// - `String` - if the string does not fulfill any of the other conditions
    #[inline]
    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.raw_value = Self::guess_typed_data(&value.into());
        self.remove_formula();
        self
    }

    #[inline]
    pub(crate) fn set_value_crate<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.raw_value = Self::guess_typed_data(&value.into());
        self
    }

    #[inline]
    pub fn set_value_lazy<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.raw_value = CellRawValue::Lazy(value.into().into_boxed_str());
        self
    }

    #[inline]
    pub fn set_value_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.raw_value = CellRawValue::String(value.into().into_boxed_str());
        self.remove_formula();
        self
    }

    #[inline]
    pub(crate) fn set_value_string_crate<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.raw_value = CellRawValue::String(value.into().into_boxed_str());
        self
    }

    #[inline]
    pub fn set_value_bool(&mut self, value: bool) -> &mut Self {
        self.raw_value = CellRawValue::Bool(value);
        self.remove_formula();
        self
    }

    #[inline]
    pub(crate) fn set_value_bool_crate(&mut self, value: bool) -> &mut Self {
        self.raw_value = CellRawValue::Bool(value);
        self
    }

    #[inline]
    pub fn set_value_number<T>(&mut self, value: T) -> &mut Self
    where
        T: Into<f64>,
    {
        self.raw_value = CellRawValue::Numeric(value.into());
        self.remove_formula();
        self
    }

    #[inline]
    pub fn set_rich_text(&mut self, value: RichText) -> &mut Self {
        self.raw_value = CellRawValue::RichText(value);
        self.remove_formula();
        self
    }

    #[inline]
    pub fn set_blank(&mut self) -> &mut Self {
        self.raw_value = CellRawValue::Empty;
        self.remove_formula();
        self
    }

    #[inline]
    #[must_use]
    pub fn is_formula(&self) -> bool {
        self.formula.is_some()
    }

    #[inline]
    #[must_use]
    pub fn get_formula(&self) -> &str {
        match &self.formula {
            Some(v) => v.get_text(),
            None => "",
        }
    }

    #[inline]
    #[must_use]
    pub fn get_formula_obj(&self) -> Option<&CellFormula> {
        self.formula.as_deref()
    }

    #[inline]
    pub fn set_formula<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let mut obj = CellFormula::default();
        obj.set_text(value.into());
        self.formula = Some(Box::new(obj));
        self
    }

    #[inline]
    pub fn set_formula_obj(&mut self, value: CellFormula) -> &mut Self {
        self.formula = Some(Box::new(value));
        self
    }

    #[inline]
    pub fn remove_formula(&mut self) -> &mut Self {
        self.formula = None;
        self
    }

    #[inline]
    pub fn set_formula_result_default<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.set_value_crate(value);
        self
    }

    #[inline]
    pub fn set_error<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.set_value_crate(value);
        self
    }

    #[inline]
    #[must_use]
    pub fn is_error(&self) -> bool {
        self.raw_value.is_error()
    }

    #[inline]
    pub(crate) fn set_shared_string_item(&mut self, value: SharedStringItem) -> &mut Self {
        if let Some(v) = value.get_text() {
            self.set_value_string(v.get_value());
        }
        if let Some(v) = value.get_rich_text() {
            self.set_rich_text(v.clone());
        }
        self
    }

    #[inline]
    pub(crate) fn guess_typed_data(value: &str) -> CellRawValue {
        let uppercase_value = value.to_uppercase();

        match uppercase_value.as_str() {
            "" => CellRawValue::Empty,
            "TRUE" => CellRawValue::Bool(true),
            "FALSE" => CellRawValue::Bool(false),
            _ => {
                if let Ok(error_type) = CellErrorType::from_str(&uppercase_value) {
                    CellRawValue::Error(error_type)
                } else if let Ok(f) = value.parse::<f64>() {
                    CellRawValue::Numeric(f)
                } else {
                    CellRawValue::String(value.into())
                }
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.is_value_empty() && self.is_formula_empty()
    }

    #[inline]
    pub(crate) fn is_value_empty(&self) -> bool {
        self.raw_value.is_empty()
    }

    #[inline]
    pub(crate) fn is_formula_empty(&self) -> bool {
        !self.is_formula()
    }

    // When opened in software such as Excel, it is visually blank.
    #[inline]
    pub(crate) fn is_visually_empty(&self) -> bool {
        self.get_value() == "" && self.is_formula_empty()
    }
}
impl AdjustmentCoordinateWith2Sheet for CellValue {
    #[inline]
    fn adjustment_insert_coordinate_with_2sheet(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
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

    #[inline]
    fn adjustment_remove_coordinate_with_2sheet(
        &mut self,
        self_sheet_name: &str,
        sheet_name: &str,
        root_col_num: u32,
        offset_col_num: u32,
        root_row_num: u32,
        offset_row_num: u32,
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

        obj.set_blank();
        assert_eq!(obj.get_value(), "");

        obj.set_error("#NUM!");
        assert_eq!(obj.get_value(), "#NUM!");
    }

    #[test]
    fn error_checking() {
        let path = std::path::Path::new("./tests/test_files/pr_204.xlsx");
        let book = crate::reader::xlsx::read(path).unwrap();
        let sheet = book.get_sheet(0).unwrap();

        let cell = sheet.get_cell_value("A1");
        assert!(cell.raw_value.is_error());
        assert_eq!(cell.raw_value, CellRawValue::Error(CellErrorType::Div0));

        let cell = sheet.get_cell_value("A2");
        assert!(cell.raw_value.is_error());
        assert_eq!(cell.raw_value, CellRawValue::Error(CellErrorType::Name));

        let cell = sheet.get_cell_value("A3");
        assert!(cell.raw_value.is_error());
        assert_eq!(cell.raw_value, CellRawValue::Error(CellErrorType::Ref));

        let cell = sheet.get_cell_value("A4");
        assert!(cell.raw_value.is_error());
        assert_eq!(cell.raw_value, CellRawValue::Error(CellErrorType::Value));

        let cell = sheet.get_cell_value("A5");
        assert!(cell.raw_value.is_error());
        assert_eq!(cell.raw_value, CellRawValue::Error(CellErrorType::NA));

        let cell = sheet.get_cell_value("A6");
        assert!(cell.raw_value.is_error());
        assert_eq!(cell.raw_value, CellRawValue::Error(CellErrorType::Num));

        let cell = sheet.get_cell_value("A7");
        assert!(cell.raw_value.is_error());
        assert_eq!(cell.raw_value, CellRawValue::Error(CellErrorType::Null));
    }
}
