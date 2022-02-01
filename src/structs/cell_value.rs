use super::RichText;
use super::SharedStringItem;
use helper::formula::*;

#[derive(Clone, Default, Debug)]
pub struct CellValue {
    pub(crate) data_type: String,
    pub(crate) value: Option<String>,
    pub(crate) rich_text: Option<RichText>,
    pub(crate) formula: Option<String>,
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

    pub fn get_value(&self) -> &str {
        match &self.value {
            Some(v) => {
                return v;
            }
            None => {}
        }
        match &self.rich_text {
            Some(v) => {
                return v.get_text();
            }
            None => {}
        }
        ""
    }

    pub(crate) fn get_width_point(&self) -> f64 {
        let mut max_point = 0f64;
        let value_list: Vec<&str> = self.get_value().split("\n").collect();
        for value in value_list {
            let mut point = 0f64;
            for chr in value.chars() {
                let mut clen = chr.len_utf8() as f64;
                if clen > 1f64 {
                    clen = 1.5;
                }
                point += clen;
            }
            if point > max_point {
                max_point = point;
            }
        }
        max_point
    }

    pub(crate) fn get_value_crate(&self) -> &Option<String> {
        &self.value
    }

    pub fn get_rich_text(&self) -> &Option<RichText> {
        &self.rich_text
    }

    pub fn set_value<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let value_org = value.into();
        self.data_type = Self::data_type_for_value(&value_org).to_string();
        self.value = Some(value_org);
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_string<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.data_type = Self::TYPE_STRING.to_string();
        self.value = Some(value.into());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_bool(&mut self, value: bool) -> &mut Self {
        self.data_type = Self::TYPE_BOOL.to_string();
        self.value = Some(match value {
            true => "TRUE".to_string(),
            false => "FALSE".to_string(),
        });
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_bool_ref(&mut self, value: &bool) -> &mut Self {
        self.set_value_from_bool(value.clone())
    }

    pub fn set_value_from_u16(&mut self, value: u16) -> &mut Self {
        self.data_type = Self::TYPE_NUMERIC.to_string();
        self.value = Some(value.to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_u16_ref(&mut self, value: &u16) -> &mut Self {
        self.set_value_from_u16(value.clone())
    }

    pub fn set_value_from_u32(&mut self, value: u32) -> &mut Self {
        self.data_type = Self::TYPE_NUMERIC.to_string();
        self.value = Some(value.to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_u32_ref(&mut self, value: &u32) -> &mut Self {
        self.set_value_from_u32(value.clone())
    }

    pub fn set_value_from_u64(&mut self, value: u64) -> &mut Self {
        self.data_type = Self::TYPE_NUMERIC.to_string();
        self.value = Some(value.to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_u64_ref(&mut self, value: &u64) -> &mut Self {
        self.set_value_from_u64(value.clone())
    }

    pub fn set_value_from_i16(&mut self, value: i16) -> &mut Self {
        self.data_type = Self::TYPE_NUMERIC.to_string();
        self.value = Some(value.to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_i16_ref(&mut self, value: &i16) -> &mut Self {
        self.set_value_from_i16(value.clone())
    }

    pub fn set_value_from_i32(&mut self, value: i32) -> &mut Self {
        self.data_type = Self::TYPE_NUMERIC.to_string();
        self.value = Some(value.to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_i32_ref(&mut self, value: &i32) -> &mut Self {
        self.set_value_from_i32(value.clone())
    }

    pub fn set_value_from_i64(&mut self, value: i64) -> &mut Self {
        self.data_type = Self::TYPE_NUMERIC.to_string();
        self.value = Some(value.to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_i64_ref(&mut self, value: &i64) -> &mut Self {
        self.set_value_from_i64(value.clone())
    }

    pub fn set_value_from_usize(&mut self, value: usize) -> &mut Self {
        self.data_type = Self::TYPE_NUMERIC.to_string();
        self.value = Some(value.to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_usize_ref(&mut self, value: &usize) -> &mut Self {
        self.set_value_from_usize(value.clone())
    }

    pub fn set_rich_text(&mut self, value: RichText) -> &mut Self {
        self.data_type = Self::TYPE_STRING.to_string();
        self.value = None;
        self.rich_text = Some(value);
        self.formula = None;
        self
    }

    pub fn set_rich_text_ref(&mut self, value: &RichText) -> &mut Self {
        self.set_rich_text(value.clone())
    }

    pub fn set_formula<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.data_type = Self::TYPE_FORMULA.to_string();
        self.value = None;
        self.rich_text = None;
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

    pub fn set_data_type<S: Into<String>>(&mut self, value: S) -> &mut Self {
        let data_type = value.into();
        match Self::check_data_type(self.get_value(), &data_type) {
            Ok(_) => self.data_type = data_type.into(),
            Err(e) => panic!("Error at set_data_type {:?}", e),
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
            md5::compute(format!(
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
