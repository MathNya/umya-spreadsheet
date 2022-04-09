use std::borrow::Cow;

use super::RichText;
use super::SharedStringItem;
use helper::formula::*;
use md5::Digest;

#[derive(Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct CellValue {
    pub(crate) value: Option<Value>,
    pub(crate) raw_value: Option<String>,
    pub(crate) rich_text: Option<RichText>,
    pub(crate) formula: Option<String>,
    pub(crate) formula_attributes: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum  Value {
    String(String),
    Formula(String),
    Numeric(f64),
    Bool(bool),
    Inline,
    Error,
    Null,
}

impl CellValue {
    // Data types
    pub const TYPE_STRING2: &'static str = "str";
    pub const TYPE_STRING: &'static str = "s";
    pub const TYPE_FORMULA: &'static str = "f";
    pub const TYPE_NUMERIC: &'static str = "n";
    pub const TYPE_BOOL: &'static str = "b";
    pub const TYPE_NULL: &'static str = "null";
    pub const TYPE_INLINE: &'static str = "inlineStr";
    pub const TYPE_ERROR: &'static str = "e";

    pub fn set_formula_attributes(&mut self, formula_attributes: Vec<(String, String)>) {
        self.formula_attributes = formula_attributes;
    }
    pub fn get_formula_attributes(&self) -> Vec<(&str, &str)> {
        self.formula_attributes
            .iter()
            .map(|(a, b)| (a.as_str(), b.as_str()))
            .collect()
    }

    // need to keep raw value in case set data type is called afterwards.
    pub fn get_typed_value(&mut self) -> &Option<Value> {
        &self.value.or_else(|| {
            self.raw_value.and_then(|r| {
                let v = Some(Self::guess_typed_data(r.as_ref()));
                self.value = v;
                v
            })
        })
    }

    pub fn get_value(&self) -> Cow<'static, str> {
        match self.get_typed_value() {
            Some(v) => {
                return v.to_string().into();
            }
            None => {}
        }
        match &self.rich_text {
            Some(v) => {
                return v.get_text().into();
            }
            None => {}
        }
        "".into()
    }

    pub(crate) fn get_value_crate(&self) -> &Option<String> {
        &self.value
    }

    pub fn get_rich_text(&self) -> &Option<RichText> {
        &self.rich_text
    }

    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        //set value lazily. parse the value if need.
        //self.value = Some(Self::guess_typed_data(value.as_ref()));
        self.raw_value = Some(value.into());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.value = Some(Value::String(value.into()));
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_bool(&mut self, value: bool) -> &mut Self {
        self.value = Some(Value::Bool(value));
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_bool_ref(&mut self, value: &bool) -> &mut Self {
        self.set_value_from_bool(value.clone())
    }

    pub fn set_value_from_numberic<V: Into<f64>>(&mut self, value: V) -> &mut Self {
        self.value = Some(Value::Numeric(value.into()));
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_rich_text(&mut self, value: RichText) -> &mut Self {
        self.rich_text = Some(value);
        self
    }

    pub fn set_rich_text_ref(&mut self, value: &RichText) -> &mut Self {
        self.set_rich_text(value.clone())
    }

    pub fn set_formula<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.formula = Some(value.into());
        self
    }

    pub(crate) fn set_shared_string_item(&mut self, value: SharedStringItem) -> &mut Self {
        self.data_type = Self::TYPE_STRING.to_string();
        match value.get_text() {
            Some(v) => {
                self.value = Some(v.get_value().to_string());
            }
            None => {}
        }
        self.rich_text = value.get_rich_text().clone();
        self.formula = None;
        self
    }

    pub fn get_data_type(&self) -> &str {
        &self.data_type
    }

    pub fn set_data_type<S: AsRef<str>>(&mut self, value: S) -> &mut Self {
        match value.as_ref() {
            _ => {
                todo!()
            }
        }

        self
    }

    pub(crate) fn check_data_type<S: Into<String>>(
        value: S,
        data_type: S,
    ) -> Result<(), &'static str> {
        match data_type.into().as_str() {
            Self::TYPE_STRING2 => return Ok(()),
            Self::TYPE_STRING => return Ok(()),
            Self::TYPE_FORMULA => return Ok(()),
            Self::TYPE_NUMERIC => match &value.into().parse::<f64>() {
                Ok(_) => return Ok(()),
                Err(_) => return Err("Invalid numeric value for datatype Numeric"),
            },
            Self::TYPE_BOOL => {
                let check_value = &value.into().to_uppercase();
                if check_value == "TRUE" || check_value == "FALSE" {
                    return Ok(());
                } else {
                    return Err("Invalid value for datatype Bool");
                }
            }
            Self::TYPE_NULL => return Ok(()),
            _ => return Err("Invalid datatype"),
        }
    }

    pub fn is_formula(&self) -> bool {
        &self.data_type == Self::TYPE_FORMULA
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

    pub(crate) fn guess_typed_data(value: &str) -> Value {
        let uppercase_value = value.to_uppercase();

        // Match the value against a few data types
        if uppercase_value == "NULL" {
            return Value::Null;
        }

        if let Ok(f) = value.parse::<f64>() {
            return Value::Numeric(f);
        }

        if uppercase_value == "TRUE" {
            return  Value::Bool(true);
        }

        if uppercase_value == "FALSE" {
            return  Value::Bool(false);
        }

        Value::String(value.into())
    }

    pub(crate) fn data_type_for_value(value: &str) -> &str {
        let check_value = value.to_uppercase();

        // Match the value against a few data types
        if check_value == "NULL" {
            return Self::TYPE_NULL;
        }
        match check_value.parse::<f64>() {
            Ok(_) => return Self::TYPE_NUMERIC,
            Err(_) => {}
        }
        if check_value == "TRUE" || check_value == "FALSE" {
            return Self::TYPE_BOOL;
        }
        Self::TYPE_STRING
    }

    pub(crate) fn _get_hash_code_by_value(&self) -> String {
        format!(
            "{:x}",
            md5::Md5::digest(format!(
                "{}{}",
                match &self.value {
                    Some(v) => {
                        v
                    }
                    None => {
                        "None"
                    }
                },
                match &self.rich_text {
                    Some(v) => {
                        v.get_hash_code()
                    }
                    None => {
                        "None".into()
                    }
                },
            ))
        )
    }

    pub(crate) fn is_empty(&self) -> bool {
        match &self.value {
            Some(_) => return false,
            None => {}
        }
        match &self.rich_text {
            Some(_) => return false,
            None => {}
        }
        match &self.formula {
            Some(_) => return false,
            None => {}
        }
        true
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

        obj.set_value_from_string(String::from("TEST"));
        assert_eq!(obj.get_value(), "TEST");

        obj.set_value_from_string("TEST");
        assert_eq!(obj.get_value(), "TEST");

        obj.set_value_from_bool(true);
        assert_eq!(obj.get_value(), "TRUE");

        obj.set_value_from_bool_ref(&true);
        assert_eq!(obj.get_value(), "TRUE");

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
