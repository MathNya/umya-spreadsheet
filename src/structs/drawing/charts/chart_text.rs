// c:tx
use std::io::Cursor;

use quick_xml::{
    Reader,
    Writer,
    events::{
        BytesStart,
        Event,
    },
};

use super::RichText;
use crate::{
    Workbook,
    drawing::charts::{
        NumericValue,
        StringReference,
    },
    writer::driver::{
        write_end_tag,
        write_start_tag,
    },
    xml_read_loop,
};

#[derive(Clone, Default, Debug)]
pub struct ChartText {
    rich_text:        Option<RichText>,
    string_reference: Option<StringReference>,
    numeric_value:    Option<NumericValue>,
}

impl ChartText {
    #[must_use]
    pub fn rich_text(&self) -> Option<&RichText> {
        self.rich_text.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use rich_text()")]
    pub fn get_rich_text(&self) -> Option<&RichText> {
        self.rich_text()
    }

    pub fn rich_text_mut(&mut self) -> Option<&mut RichText> {
        self.rich_text.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use rich_text_mut()")]
    pub fn get_rich_text_mut(&mut self) -> Option<&mut RichText> {
        self.rich_text_mut()
    }

    pub fn set_rich_text(&mut self, value: RichText) -> &mut Self {
        self.rich_text = Some(value);
        self
    }

    #[must_use]
    pub fn string_reference(&self) -> Option<&StringReference> {
        self.string_reference.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use string_reference()")]
    pub fn get_string_reference(&self) -> Option<&StringReference> {
        self.string_reference()
    }

    pub fn string_reference_mut(&mut self) -> Option<&mut StringReference> {
        self.string_reference.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use string_reference_mut()")]
    pub fn get_string_reference_mut(&mut self) -> Option<&mut StringReference> {
        self.string_reference_mut()
    }

    pub fn set_string_reference(&mut self, value: StringReference) -> &mut Self {
        self.string_reference = Some(value);
        self
    }

    #[must_use]
    pub fn numeric_value(&self) -> Option<&NumericValue> {
        self.numeric_value.as_ref()
    }

    #[must_use]
    #[deprecated(since = "3.0.0", note = "Use numeric_value()")]
    pub fn get_numeric_value(&self) -> Option<&NumericValue> {
        self.numeric_value()
    }

    pub fn numeric_value_mut(&mut self) -> Option<&mut NumericValue> {
        self.numeric_value.as_mut()
    }

    #[deprecated(since = "3.0.0", note = "Use numeric_value_mut()")]
    pub fn get_numeric_value_mut(&mut self) -> Option<&mut NumericValue> {
        self.numeric_value_mut()
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

    pub(crate) fn write_to(&self, writer: &mut Writer<Cursor<Vec<u8>>>, wb: &Workbook) {
        // c:tx
        write_start_tag(writer, "c:tx", vec![], false);

        // c:rich
        if let Some(v) = &self.rich_text {
            v.write_to(writer);
        }

        // c:strRef
        if let Some(v) = &self.string_reference {
            v.write_to(writer, wb);
        }

        // c:v
        if let Some(v) = &self.numeric_value {
            v.write_to(writer);
        }

        write_end_tag(writer, "c:tx");
    }
}
