// c:tx
use super::NumericValue;
use super::RichText;
use super::StringReference;
use crate::structs::Spreadsheet;
use crate::writer::driver::*;
use crate::xml_read_loop;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use std::io::Cursor;

#[derive(Clone, Default, Debug)]
pub struct ChartText {
    rich_text: Option<RichText>,
    string_reference: Option<StringReference>,
    numeric_value: Option<NumericValue>,
}

impl ChartText {
    pub fn get_rich_text(&self) -> Option<&RichText> {
        self.rich_text.as_ref()
    }

    pub fn get_rich_text_mut(&mut self) -> Option<&mut RichText> {
        self.rich_text.as_mut()
    }

    pub fn set_rich_text(&mut self, value: RichText) -> &mut ChartText {
        self.rich_text = Some(value);
        self
    }

    pub fn get_string_reference(&self) -> Option<&StringReference> {
        self.string_reference.as_ref()
    }

    pub fn get_string_reference_mut(&mut self) -> Option<&mut StringReference> {
        self.string_reference.as_mut()
    }

    pub fn set_string_reference(&mut self, value: StringReference) -> &mut Self {
        self.string_reference = Some(value);
        self
    }

    pub fn get_numeric_value(&self) -> Option<&NumericValue> {
        self.numeric_value.as_ref()
    }

    pub fn get_numeric_value_mut(&mut self) -> Option<&mut NumericValue> {
        self.numeric_value.as_mut()
    }

    pub fn set_numeric_value(&mut self, value: NumericValue) -> &mut Self {
        self.numeric_value = Some(value);
        self
    }

    pub(crate) fn set_attributes<R: std::io::BufRead>(
        &mut self,
        reader: &mut Reader<R>,
        _e: &BytesStart,
    ) {
        xml_read_loop!(
            reader,
            Event::Start(ref e) => {
                match e.name().into_inner() {
                    b"c:rich" => {
                        let mut obj = RichText::default();
                        obj.set_attributes(reader, e);
                        self.set_rich_text(obj);
                    }
                    b"c:strRef" => {
                        let mut obj = StringReference::default();
                        obj.set_attributes(reader, e);
                        self.set_string_reference(obj);
                    }
                    b"c:v" => {
                        let mut obj = NumericValue::default();
                        obj.set_attributes(reader, e);
                        self.set_numeric_value(obj);
                    }
                    _ => (),
                }
            },
            Event::End(ref e) => {
                if e.name().into_inner() == b"c:tx" {
                    return;
                }
            },
            Event::Eof => panic!("Error: Could not find {} end element", "c:tx"),
        );
    }

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, spreadsheet: &Spreadsheet) {
        // c:tx
        write_start_tag(writer, "c:tx", vec![], false);

        // c:rich
        if let Some(v) = &self.rich_text {
            v.write_to(writer);
        }

        // c:strRef
        if let Some(v) = &self.string_reference {
            v.write_to(writer, spreadsheet);
        }

        // c:v
        if let Some(v) = &self.numeric_value {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:tx");
    }
}
