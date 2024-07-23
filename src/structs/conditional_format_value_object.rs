use super::ConditionalFormatValueObjectValues;
use super::EnumValue;
use super::StringValue;
use quick_xml::events::{BytesStart, Event};
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
        self.r#type.get_value()
    }

    pub fn set_type(&mut self, value: ConditionalFormatValueObjectValues) -> &mut Self {
        self.r#type.set_value(value);
        self
    }

    pub fn get_val(&self) -> &str {
        self.val.get_value_str()
    }

    pub fn set_val<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.val.set_value(value.into());
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        e: &BytesStart,
        empty_flg: bool,
    ) {
        set_string_from_xml!(self, e, r#type, "type");
        set_string_from_xml!(self, e, val, "val");

        if empty_flg {
            return;
        }

        xml_read_loop!(
            reader,
            Event::End(ref e) => {
                if e.name().into_inner() == b"cfvo" {
                    return
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "cfvo")
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>) {
        // cfvo
        let mut attributes: Vec<(&str, &str)> = Vec::new();
        let ctype = self.r#type.get_value_string();
        if self.r#type.has_value() {
            attributes.push(("type", ctype));
        }
        let val = self.val.get_value_str();
        if self.val.has_value() {
            attributes.push(("val", val));
        }

        write_start_tag(writer, "cfvo", attributes, true);
    }
}
