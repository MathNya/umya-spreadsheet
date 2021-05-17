use super::rich_text::RichText;
use super::hyperlink::Hyperlink;
use super::coordinate::Coordinate;
use super::super::helper::formula::*;

#[derive(Default, Debug)]
pub struct Cell {
    coordinate: Coordinate,
    value: String,
    rich_text: Option<RichText>,
    data_type: String,
    formula: String,
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

    pub fn get_value(&self)-> &String {
        &self.value
    }

    pub fn get_rich_text(&self)-> &Option<RichText> {
        &self.rich_text
    }

    pub fn set_value<S: Into<String>>(&mut self, value:S)-> &mut Cell {
        let v = value.into();
        self.data_type = Cell::data_type_for_value(&v).to_string();
        self.set_value_crate(v);
        self
    }

    pub(crate) fn set_value_crate<S: Into<String>>(&mut self, value:S)-> &mut Cell {
        self.value = value.into();
        self.rich_text = None;
        self
    }

    pub(crate) fn set_all_param<S: Into<String>>(&mut self, value:S, rich_text:Option<RichText>, data_type:S, formula_attributes:S)-> &mut Cell {
        self.value = value.into();
        self.rich_text = rich_text;
        self.data_type = data_type.into();
        self.formula = formula_attributes.into();
        self
    }

    pub fn get_data_type(&self)-> &str {
        &self.data_type
    }

    pub fn set_value_and_data_type<S: Into<String>>(&mut self, value:S, data_type:S)-> &mut Cell {
        let v = value.into();
        let d = data_type.into();
        match Cell::check_data_type(&v, &d) {
            Ok(_) => {
                self.set_value_crate(v);
                if &d == Cell::TYPE_STRING2 {
                    self.data_type = Cell::TYPE_STRING.to_string();
                } else {
                    self.data_type = d;
                }
            },
            Err(e) => panic!("Error at set_value_and_data_type {:?}", e),
        }
        self
    }

    pub fn set_data_type<S: Into<String>>(&mut self, value:S)-> &mut Cell {
        let data_type = value.into();
        match Cell::check_data_type(&self.value, &data_type) {
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

    pub fn get_formula(&self)-> &String {
        &self.formula
    }

    pub fn set_formula<S: Into<String>>(&mut self, value:S)-> &mut Cell {
        self.data_type = Cell::TYPE_FORMULA.to_string();
        self.formula = value.into();
        self
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
            &self.value,
            match &self.rich_text {Some(v) => {v.get_hash_code()}, None => {"None".into()}},
        )))
    }

    pub(crate) fn adjustment_insert_formula_coordinate(&mut self, self_sheet_name:&str, sheet_name:&str, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        if self.is_formula() {
            let formula = adjustment_insert_formula_coordinate(&self.formula, root_col_num, offset_col_num, root_row_num, offset_row_num, sheet_name, self_sheet_name);
            self.formula = formula;
        }
    }

    pub(crate) fn adjustment_remove_formula_coordinate(&mut self, self_sheet_name:&str, sheet_name:&str, root_col_num:&usize, offset_col_num:&usize, root_row_num:&usize, offset_row_num:&usize) {
        if self.is_formula() {
            let formula = adjustment_remove_formula_coordinate(&self.formula, root_col_num, offset_col_num, root_row_num, offset_row_num, sheet_name, self_sheet_name);
            self.formula = formula;
        }
    }
}