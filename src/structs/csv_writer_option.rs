use super::EnumValue;
use structs::CsvEncodeValues;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CsvWriterOption {
    pub(crate) csv_encode_values: EnumValue<CsvEncodeValues>,
    pub(crate) wrap_with_char: String,
    pub(crate) do_trim: bool,
}
impl CsvWriterOption {
    pub fn get_csv_encode_value(&self) -> &CsvEncodeValues {
        self.csv_encode_values.get_value()
    }

    pub fn set_csv_encode_value(&mut self, value: CsvEncodeValues) -> &mut Self {
        self.csv_encode_values.set_value(value);
        self
    }

    pub fn get_wrap_with_char(&self) -> &str {
        &self.wrap_with_char
    }

    pub fn set_wrap_with_char<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.wrap_with_char = value.into();
        self
    }

    pub fn get_do_trim(&self) -> &bool {
        &self.do_trim
    }

    pub fn set_do_trim(&mut self, value: bool) -> &mut Self {
        self.do_trim = value;
        self
    }
}
