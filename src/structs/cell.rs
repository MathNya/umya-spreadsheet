#[derive(Default, Debug)]
pub struct Cell {
    value: String,
    data_type: String,
    xf_index  : usize,
    formula_attributes: String,
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

    pub fn get_value(&self)-> &String {
        &self.value
    }
    pub fn set_value<S: Into<String>>(&mut self, value:S)->Result<(), &str> {
        let v = value.into();
        Cell::data_type_for_value(&v);
        self.value = v;
        Ok(())
    }
    pub fn get_data_type(&self)-> &str {
        &self.data_type
    }
    pub fn set_value_and_data_type<S: Into<String>>(&mut self, value:S, data_type:S)->Result<(), &'static str> {
        let v = value.into();
        let d = data_type.into();
        match Cell::check_data_type(&v, &d) {
            Ok(_) => {
                self.value = v;
                self.data_type = d;
            },
            Err(e) => return Err(e)
        }
        Ok(())
    }
    pub fn set_data_type<S: Into<String>>(&mut self, value:S)->Result<(), &'static str> {
        let data_type = value.into();
        match Cell::check_data_type(&self.value, &data_type) {
            Ok(_) => self.data_type = data_type.into(),
            Err(e) => return Err(e)
        }
        Ok(())
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
            Cell::TYPE_BOOL => return Ok(()),
            Cell::TYPE_NULL => return Ok(()),
            _ => return Err("Invalid datatype")
        }
    }
    pub fn is_formula(&self) -> bool {
        &self.data_type == Cell::TYPE_FORMULA
    }
    pub fn get_xf_index(&self)-> &usize {
        &self.xf_index
    }
    pub(crate) fn set_xf_index(&mut self, value:usize) {
        self.xf_index = value;
    }
    pub fn get_formula_attributes(&self)-> &String {
        &self.formula_attributes
    }
    pub(crate) fn set_formula_attributes<S: Into<String>>(&mut self, value:S) {
        self.formula_attributes = value.into();
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
}