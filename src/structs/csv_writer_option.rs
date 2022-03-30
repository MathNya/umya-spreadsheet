use super::EnumValue;
use super::FontSchemeValues;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CsvWriterOption {
    pub(crate) csv_encode_values: EnumValue<CsvEncodeValues>,
}
impl CsvWriterOption {
    pub fn get_csv_encode_value(&self) -> &CsvEncodeValues {
        &self.csv_encode_values.get_value()
    }

    pub fn set_csv_encode_value(&mut self, value: CsvEncodeValues) -> &mut Self {
        self.csv_encode_values.set_value(value);
        self
    }
}
