use super::RichText;
use super::Hyperlink;
use super::Coordinate;
use super::SharedStringItem;
use ::helper::formula::*;

#[derive(Default, Debug)]
pub struct Cell {
    coordinate: Coordinate,
    data_type: String,
    value: Option<String>,
    rich_text: Option<RichText>,
    formula: Option<String>,
    hyperlink: Option<Hyperlink>,
}
impl Cell {
    // Data types
    pub const TYPE_STRING2: &'static str = "str";
    pub const TYPE_STRING: &'static str = "s";
    pub const TYPE_FORMULA: &'static str = "f";
    pub const TYPE_NUMERIC: &'static str = "n";
    pub const TYPE_BOOL: &'static str = "b";
    pub const TYPE_NULL: &'static str = "null";
    pub const TYPE_INLINE: &'static str = "inlineStr";
    pub const TYPE_ERROR: &'static str = "e";

    pub fn get_coordinate(&self)-> &Coordinate {
        &self.coordinate
    }

    pub fn get_coordinate_mut(&mut self)-> &mut Coordinate {
        &mut self.coordinate
    }

    pub fn get_hyperlink(&self) -> &Option<Hyperlink> {
        &self.hyperlink
    }

    pub fn get_hyperlink_mut(&mut self) -> &mut Hyperlink {
        match &self.hyperlink {
            Some(_) => return self.hyperlink.as_mut().unwrap(),
            None => {}
        }
        let _ = self.set_hyperlink(Hyperlink::default());
        self.hyperlink.as_mut().unwrap()
    }

    pub fn set_hyperlink(&mut self, value:Hyperlink)-> &mut Cell {
        self.hyperlink = Some(value);
        self
    }

    pub fn get_value(&self)-> &str {
        match &self.value {
            Some(v) => {return v;},
            None => {},
        }
        match &self.rich_text {
            Some(v) => {
                return v.get_text();
            },
            None => {},
        }
        ""
    }
    
    pub(crate) fn get_value_crate(&self)-> &Option<String> {
        &self.value
    }

    pub fn get_rich_text(&self)-> &Option<RichText> {
        &self.rich_text
    }

    pub fn set_value<S: Into<String>>(&mut self, value:S)-> &mut Self {
        let value_org = value.into();
        self.data_type = Cell::data_type_for_value(&value_org).to_string();
        self.value = Some(value_org);
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_string<S: Into<String>>(&mut self, value:S)-> &mut Self {
        self.data_type = Cell::TYPE_STRING.to_string();
        self.value = Some(value.into());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_bool<S: Into<bool>>(&mut self, value:S)-> &mut Self {
        self.data_type = Cell::TYPE_BOOL.to_string();
        self.value = Some(match value.into() {true=>{"TRUE".to_string()}, false=>{"FALSE".to_string()}});
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_u16<S: Into<u16>>(&mut self, value:S)-> &mut Self {
        self.data_type = Cell::TYPE_NUMERIC.to_string();
        self.value = Some(value.into().to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_u32<S: Into<u32>>(&mut self, value:S)-> &mut Self {
        self.data_type = Cell::TYPE_NUMERIC.to_string();
        self.value = Some(value.into().to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_u64<S: Into<u64>>(&mut self, value:S)-> &mut Self {
        self.data_type = Cell::TYPE_NUMERIC.to_string();
        self.value = Some(value.into().to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_i16<S: Into<i16>>(&mut self, value:S)-> &mut Self {
        self.data_type = Cell::TYPE_NUMERIC.to_string();
        self.value = Some(value.into().to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_i32<S: Into<i32>>(&mut self, value:S)-> &mut Self {
        self.data_type = Cell::TYPE_NUMERIC.to_string();
        self.value = Some(value.into().to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_i64<S: Into<i64>>(&mut self, value:S)-> &mut Self {
        self.data_type = Cell::TYPE_NUMERIC.to_string();
        self.value = Some(value.into().to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_value_from_usize<S: Into<usize>>(&mut self, value:S)-> &mut Self {
        self.data_type = Cell::TYPE_NUMERIC.to_string();
        self.value = Some(value.into().to_string());
        self.rich_text = None;
        self.formula = None;
        self
    }

    pub fn set_rich_text<S: Into<RichText>>(&mut self, value:S)-> &mut Self {
        self.data_type = Cell::TYPE_STRING.to_string();
        self.value = None;
        self.rich_text = Some(value.into());
        self.formula = None;
        self
    }

    pub fn set_formula<S: Into<String>>(&mut self, value:S)-> &mut Cell {
        self.data_type = Cell::TYPE_FORMULA.to_string();
        self.value = None;
        self.rich_text = None;
        self.formula = Some(value.into());
        self
    }

    pub(crate) fn set_shared_string_item(&mut self, value:SharedStringItem)-> &mut Self {
        self.data_type = Cell::TYPE_STRING.to_string();
        match value.get_text() {
            Some(v) => {self.value = Some(v.get_value().to_string());},
            None => {}
        }
        self.rich_text = value.get_rich_text().clone();
        self.formula = None;
        self
    }

    pub fn get_data_type(&self)-> &str {
        &self.data_type
    }

    pub fn set_data_type<S: Into<String>>(&mut self, value:S)-> &mut Cell {
        let data_type = value.into();
        match Cell::check_data_type(self.get_value(), &data_type) {
            Ok(_) => self.data_type = data_type.into(),
            Err(e) => panic!("Error at set_data_type {:?}", e),
        }
        self
    }

    pub(crate) fn check_data_type<S: Into<String>>(value:S, data_type:S)->Result<(), &'static str> {
        match data_type.into().as_str() {
            Cell::TYPE_STRING2 => return Ok(()),
            Cell::TYPE_STRING => return Ok(()),
            Cell::TYPE_FORMULA => return Ok(()),
            Cell::TYPE_NUMERIC => {
                match &value.into().parse::<f64>() {
                    Ok(_) => return Ok(()),
                    Err(_) => return Err("Invalid numeric value for datatype Numeric")
                }
            },
            Cell::TYPE_BOOL => {
                let check_value = &value.into().to_uppercase();
                if check_value == "TRUE" || check_value == "FALSE" {
                    return Ok(());
                } else {
                    return Err("Invalid value for datatype Bool")
                }
            },
            Cell::TYPE_NULL => return Ok(()),
            _ => return Err("Invalid datatype")
        }
    }

    pub fn is_formula(&self) -> bool {
        &self.data_type == Cell::TYPE_FORMULA
    }

    pub fn get_formula(&self)-> &str {
        match &self.formula {
            Some(v) => {return v;},
            None => {},
        }
        ""
    }


    pub(crate) fn data_type_for_value(value:&str)-> &str {
        let check_value = value.to_uppercase();

        // Match the value against a few data types
        if check_value == "NULL" {
            return Cell::TYPE_NULL;
        }
        match check_value.parse::<f64>() {
            Ok(_) => return Cell::TYPE_NUMERIC,
            Err(_) => {}
        }
        if check_value == "TRUE" || check_value == "FALSE" {
            return Cell::TYPE_BOOL;
        }
        Cell::TYPE_STRING
    }
    
    pub(crate) fn get_hash_code_by_value(&self)-> String {
        format!("{:x}", md5::compute(format!("{}{}",
            match &self.value {Some(v) => {v}, None => {"None"}},
            match &self.rich_text {Some(v) => {v.get_hash_code()}, None => {"None".into()}},
        )))
    }

    pub(crate) fn adjustment_insert_formula_coordinate(&mut self, self_sheet_name:&str, sheet_name:&str, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        match &self.formula {
            Some(v) => {
                let formula = adjustment_insert_formula_coordinate(v, root_col_num, offset_col_num, root_row_num, offset_row_num, sheet_name, self_sheet_name);
                self.formula = Some(formula);
            },
            None => {},
        }
    }

    pub(crate) fn adjustment_remove_formula_coordinate(&mut self, self_sheet_name:&str, sheet_name:&str, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        match &self.formula {
            Some(v) => {
                let formula = adjustment_remove_formula_coordinate(v, root_col_num, offset_col_num, root_row_num, offset_row_num, sheet_name, self_sheet_name);
                self.formula = Some(formula);
            },
            None => {}
        }
    }
}