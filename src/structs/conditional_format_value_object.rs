use super::StringValue;
use super::EnumValue;
use super::ConditionalFormatValueObjectValues;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use quick_xml::Writer;
use reader::driver::*;
use std::io::Cursor;
use writer::driver::*;

#[derive(Clone, Default, Debug)]
pub struct ConditionalFormatValueObject {
    r#type: EnumValue<ConditionalFormatValueObjectValues>,
    val: StringValue,
}
impl ConditionalFormatValueObject {
    pub fn get_type(&self) -> &ConditionalFormatValueObjectValues {
        &self.r#type.get_value()
    }

    pub fn set_type(&mut self, value: ConditionalFormatValueObjectValues) -> &mut Self {
        self.r#type.set_value(value);
        self
    }
    
    pub fn get_val(&self) -> &str {
        &self.val.get_value()
    }

    pub fn set_val<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.val.set_value(value.into());
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        _reader: &mut Reader<R>,
        e: &BytesStart,
    ) {
        match get_attribute(e, b"type") {
            Some(v) => {
                self.r#type.set_value_string(v);
            }
            None => {}
        }

        match get_attribute(e, b"val") {
            Some(v) => {
                self.val.set_value_string(v);
            }
            None => {}
        }
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // cfvo
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let ctype = self.r#type.get_value_string();
        if self.r#type.has_value() {
            attributes.push(("type", &ctype));
        }
        let val = self.val.get_value_string();
        if self.val.has_value() {
            attributes.push(("val", &val));
        }

        write_start_tag(writer, "cfvo", attributes, true);
    }
}
